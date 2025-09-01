import React from 'react'
import { useVimMode } from '@/hooks/use-vim-mode'
import { prefGetVimShowStatusLine } from '@/pref'

export interface VimStatusLineProps {
  className?: string
}

export default function VimStatusLine({ className = '' }: VimStatusLineProps) {
  const vimMode = useVimMode()
  const showStatusLine = prefGetVimShowStatusLine()

  // Don't render anything if vim mode is disabled or status line is hidden
  if (!vimMode.enabled || !showStatusLine) {
    return null
  }

  const modeText = vimMode.getModeText()
  const panelIndicators = vimMode.getPanelIndicators()

  return (
    <div className={`flex items-center space-x-2 ${className}`}>
      {/* Vim Mode Indicator */}
      <div className={`px-2 py-1 text-xs font-mono rounded-sm ${getModeBgColor(vimMode.mode)}`}>
        {modeText}
      </div>

      {/* Panel Focus Indicators */}
      {vimMode.enabled && (
        <div className="flex items-center space-x-1 text-xs text-muted-foreground font-mono">
          <span className={`px-1 ${panelIndicators.left ? 'text-blue-500 font-bold' : 'text-muted-foreground'}`}>
            [L]
          </span>
          <span className={`px-1 ${panelIndicators.middle ? 'text-blue-500 font-bold' : 'text-muted-foreground'}`}>
            [M]
          </span>
          <span className={`px-1 ${panelIndicators.right ? 'text-blue-500 font-bold' : 'text-muted-foreground'}`}>
            [R]
          </span>
        </div>
      )}
    </div>
  )
}

// Helper function to get background color for different vim modes
function getModeBgColor(mode: string): string {
  switch (mode) {
    case 'normal':
      return 'bg-blue-600 text-white'
    case 'visual':
    case 'visual-line':
      return 'bg-orange-600 text-white'
    default:
      return 'bg-gray-600 text-white'
  }
}

// Alternative compact version for smaller spaces
export function VimStatusLineCompact({ className = '' }: VimStatusLineProps) {
  const vimMode = useVimMode()
  const showStatusLine = prefGetVimShowStatusLine()

  if (!vimMode.enabled || !showStatusLine) {
    return null
  }

  const panelIndicators = vimMode.getPanelIndicators()
  const activePanel = panelIndicators.left ? 'L' : panelIndicators.middle ? 'M' : panelIndicators.right ? 'R' : 'M'

  return (
    <div className={`flex items-center space-x-1 ${className}`}>
      {/* Mode + Active Panel in compact format */}
      <div className={`px-1.5 py-0.5 text-xs font-mono rounded-sm ${getModeBgColor(vimMode.mode)}`}>
        {getModeShortText(vimMode.mode)}{activePanel}
      </div>
    </div>
  )
}

// Helper function to get short mode text
function getModeShortText(mode: string): string {
  switch (mode) {
    case 'normal':
      return 'N:'
    case 'visual':
      return 'V:'
    case 'visual-line':
      return 'VL:'
    default:
      return 'N:'
  }
}