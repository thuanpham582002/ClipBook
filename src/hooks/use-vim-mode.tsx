import { useState, useCallback, useEffect } from 'react'
import { prefGetVimModeEnabled, prefSetVimModeEnabled, prefGetVimShowStatusLine, prefGetVimPanelNavigation } from '@/pref'

// Vim mode types
export type VimMode = 'normal' | 'visual' | 'visual-line'
export type PanelFocus = 'left' | 'middle' | 'right'

// Vim mode state interface
export interface VimModeState {
  enabled: boolean
  mode: VimMode
  panelFocus: PanelFocus
  visualStartIndex?: number
}

// Vim key handler result
export interface VimKeyResult {
  handled: boolean
  preventDefault?: boolean
  action?: string
}

export const useVimMode = () => {
  // Initial state - load from preferences
  const [state, setState] = useState<VimModeState>({
    enabled: prefGetVimModeEnabled(),
    mode: 'normal',
    panelFocus: 'middle',
    visualStartIndex: undefined
  })

  // Enable/disable vim mode
  const setEnabled = useCallback((enabled: boolean) => {
    prefSetVimModeEnabled(enabled) // Save to preferences
    setState(prev => ({
      ...prev,
      enabled,
      mode: 'normal', // Reset to normal when toggling
      panelFocus: 'middle',
      visualStartIndex: undefined
    }))
  }, [])

  // Set vim mode
  const setMode = useCallback((mode: VimMode, visualStartIndex?: number) => {
    setState(prev => ({
      ...prev,
      mode,
      visualStartIndex: mode === 'visual' || mode === 'visual-line' ? visualStartIndex : undefined
    }))
  }, [])

  // Set panel focus
  const setPanelFocus = useCallback((panelFocus: PanelFocus) => {
    setState(prev => ({
      ...prev,
      panelFocus
    }))
  }, [])

  // Handle vim key presses
  const handleVimKeyPress = useCallback((
    event: KeyboardEvent,
    currentSelectedIndex?: number
  ): VimKeyResult => {
    if (!state.enabled) {
      return { handled: false }
    }

    const key = event.key.toLowerCase()
    const code = event.code
    const ctrl = event.ctrlKey
    const shift = event.shiftKey

    // Handle mode transitions
    switch (key) {
      case 'escape':
        if (state.mode === 'visual' || state.mode === 'visual-line') {
          setMode('normal')
          return { handled: true, preventDefault: true, action: 'exit-visual-mode' }
        }
        // Reset panel focus to middle on escape from other panels
        if (state.panelFocus !== 'middle') {
          setPanelFocus('middle')
          return { handled: true, preventDefault: true, action: 'focus-middle-panel' }
        }
        return { handled: false }

      case 'v':
        if (state.mode === 'normal') {
          if (shift) {
            // Shift+V = Visual line mode
            setMode('visual-line', currentSelectedIndex)
            return { handled: true, preventDefault: true, action: 'enter-visual-line-mode' }
          } else {
            // v = Visual mode
            setMode('visual', currentSelectedIndex)
            return { handled: true, preventDefault: true, action: 'enter-visual-mode' }
          }
        }
        return { handled: false }
    }

    // Handle panel navigation (h/l)
    if (state.mode === 'normal' || state.mode === 'visual' || state.mode === 'visual-line') {
      switch (key) {
        case 'h':
          if (state.panelFocus === 'middle') {
            setPanelFocus('left')
            return { handled: true, preventDefault: true, action: 'focus-left-panel' }
          } else if (state.panelFocus === 'right') {
            setPanelFocus('middle')
            return { handled: true, preventDefault: true, action: 'focus-middle-panel' }
          }
          return { handled: false }

        case 'l':
          if (state.panelFocus === 'left') {
            setPanelFocus('middle')
            return { handled: true, preventDefault: true, action: 'focus-middle-panel' }
          } else if (state.panelFocus === 'middle') {
            setPanelFocus('right')
            return { handled: true, preventDefault: true, action: 'focus-right-panel' }
          }
          return { handled: false }
      }
    }

    // Handle vertical navigation (j/k) - will be implemented in later phases
    if (state.panelFocus === 'middle') {
      switch (key) {
        case 'j':
          return { handled: true, preventDefault: true, action: 'move-down' }
        case 'k':
          return { handled: true, preventDefault: true, action: 'move-up' }
        case 'g':
          if (event.key === 'g') { // Will need to handle gg sequence later
            return { handled: true, preventDefault: true, action: 'goto-first' }
          }
          return { handled: false }
      }

      // Handle G key
      if (shift && key === 'g') {
        return { handled: true, preventDefault: true, action: 'goto-last' }
      }

      // Handle Ctrl combinations for scrolling
      if (ctrl) {
        switch (key) {
          case 'd':
            return { handled: true, preventDefault: true, action: 'page-down' }
          case 'u':
            return { handled: true, preventDefault: true, action: 'page-up' }
          case 'f':
            return { handled: true, preventDefault: true, action: 'page-down-full' }
          case 'b':
            return { handled: true, preventDefault: true, action: 'page-up-full' }
        }
      }
    }

    // Handle actions (y/d/p/f/space/enter)
    switch (key) {
      case 'y':
        return { handled: true, preventDefault: true, action: 'yank-copy' }
      case 'd':
        if (!ctrl) { // Avoid conflict with Ctrl+d
          return { handled: true, preventDefault: true, action: 'delete' }
        }
        return { handled: false }
      case 'p':
        return { handled: true, preventDefault: true, action: 'paste' }
      case 'f':
        return { handled: true, preventDefault: true, action: 'toggle-favorite' }
      case ' ':
        return { handled: true, preventDefault: true, action: 'toggle-preview' }
      case 'enter':
        return { handled: true, preventDefault: true, action: 'context-action' }
      case '/':
        return { handled: true, preventDefault: true, action: 'focus-search' }
    }

    return { handled: false }
  }, [state, setMode, setPanelFocus])

  // Get current mode display text
  const getModeText = useCallback((): string => {
    if (!state.enabled) return ''
    
    switch (state.mode) {
      case 'normal':
        return '-- VIM --'
      case 'visual':
        return '-- VISUAL --'
      case 'visual-line':
        return '-- VISUAL LINE --'
      default:
        return '-- VIM --'
    }
  }, [state.enabled, state.mode])

  // Get panel focus indicators
  const getPanelIndicators = useCallback((): { left: boolean, middle: boolean, right: boolean } => {
    if (!state.enabled) return { left: false, middle: true, right: false }
    
    return {
      left: state.panelFocus === 'left',
      middle: state.panelFocus === 'middle',
      right: state.panelFocus === 'right'
    }
  }, [state.enabled, state.panelFocus])

  return {
    // State
    ...state,
    
    // Actions
    setEnabled,
    setMode,
    setPanelFocus,
    handleVimKeyPress,
    
    // Computed values
    getModeText,
    getPanelIndicators,
    
    // Helper functions
    isNormalMode: state.mode === 'normal',
    isVisualMode: state.mode === 'visual' || state.mode === 'visual-line',
    isMiddlePanelFocused: state.panelFocus === 'middle',
    isLeftPanelFocused: state.panelFocus === 'left',
    isRightPanelFocused: state.panelFocus === 'right'
  }
}