import { PanelFocus } from '@/hooks/use-vim-mode'

// Navigation actions that can be performed
export type NavigationAction = 
  | 'move-down' 
  | 'move-up' 
  | 'move-left'
  | 'move-right'
  | 'goto-first' 
  | 'goto-last'
  | 'page-down'
  | 'page-up'
  | 'page-down-full'
  | 'page-up-full'
  | 'focus-left-panel'
  | 'focus-middle-panel'
  | 'focus-right-panel'
  | 'focus-search'
  | 'exit-visual-mode'
  | 'enter-visual-mode'
  | 'enter-visual-line-mode'
  | 'yank-copy'
  | 'delete'
  | 'paste'
  | 'toggle-favorite'
  | 'toggle-preview'
  | 'context-action'

// Panel focus utilities
export class VimNavigationManager {
  // Get the next panel focus when navigating left
  static getLeftPanelFocus(currentFocus: PanelFocus): PanelFocus | null {
    switch (currentFocus) {
      case 'middle':
        return 'left'
      case 'right':
        return 'middle'
      case 'left':
        return null // Already at leftmost panel
      default:
        return null
    }
  }

  // Get the next panel focus when navigating right
  static getRightPanelFocus(currentFocus: PanelFocus): PanelFocus | null {
    switch (currentFocus) {
      case 'left':
        return 'middle'
      case 'middle':
        return 'right'
      case 'right':
        return null // Already at rightmost panel
      default:
        return null
    }
  }

  // Get CSS classes for panel focus indicators
  static getPanelFocusClasses(panelType: PanelFocus, currentFocus: PanelFocus): string {
    const isActive = panelType === currentFocus
    return isActive 
      ? 'ring-2 ring-blue-500 ring-opacity-50 bg-blue-50 dark:bg-blue-900/20' 
      : ''
  }

  // Check if a specific panel has focus
  static isPanelFocused(panelType: PanelFocus, currentFocus: PanelFocus): boolean {
    return panelType === currentFocus
  }

  // Get focus element selector for a panel
  static getPanelFocusSelector(panelType: PanelFocus): string {
    switch (panelType) {
      case 'left':
        return '[data-vim-panel="left"]'
      case 'middle':
        return '[data-vim-panel="middle"]'
      case 'right':
        return '[data-vim-panel="right"]'
      default:
        return '[data-vim-panel="middle"]' // Default to middle
    }
  }

  // Focus management for DOM elements
  static focusPanel(panelType: PanelFocus): void {
    // Remove focus from all panels
    document.querySelectorAll('[data-vim-panel]').forEach(panel => {
      panel.removeAttribute('data-vim-focused')
    })

    // Add focus to the target panel
    const targetPanel = document.querySelector(VimNavigationManager.getPanelFocusSelector(panelType))
    if (targetPanel) {
      targetPanel.setAttribute('data-vim-focused', 'true')
      
      // Try to focus the first focusable element within the panel
      const focusableElements = targetPanel.querySelectorAll(
        'button:not([disabled]), input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])'
      )
      
      if (focusableElements.length > 0) {
        (focusableElements[0] as HTMLElement).focus()
      }
    }
  }

  // Handle keyboard navigation within a panel
  static handlePanelNavigation(
    panelType: PanelFocus,
    action: NavigationAction,
    context?: {
      currentIndex?: number
      itemCount?: number
      selectNextItem?: () => void
      selectPreviousItem?: () => void
      selectFirstItem?: () => void
      selectLastItem?: () => void
      jumpToNextGroupOfItems?: () => void
      jumpToPrevGroupOfItems?: () => void
    }
  ): boolean {
    if (!context) return false

    switch (action) {
      case 'move-down':
        if (panelType === 'middle' && context.selectNextItem) {
          context.selectNextItem()
          return true
        }
        break

      case 'move-up':
        if (panelType === 'middle' && context.selectPreviousItem) {
          context.selectPreviousItem()
          return true
        }
        break

      case 'goto-first':
        if (panelType === 'middle' && context.selectFirstItem) {
          context.selectFirstItem()
          return true
        }
        break

      case 'goto-last':
        if (panelType === 'middle' && context.selectLastItem) {
          context.selectLastItem()
          return true
        }
        break

      case 'page-down':
        if (panelType === 'middle' && context.jumpToNextGroupOfItems) {
          context.jumpToNextGroupOfItems()
          return true
        }
        break

      case 'page-up':
        if (panelType === 'middle' && context.jumpToPrevGroupOfItems) {
          context.jumpToPrevGroupOfItems()
          return true
        }
        break

      default:
        return false
    }

    return false
  }

  // Get panel title for accessibility
  static getPanelTitle(panelType: PanelFocus): string {
    switch (panelType) {
      case 'left':
        return 'Filters Sidebar'
      case 'middle':
        return 'Clipboard History'
      case 'right':
        return 'Preview Panel'
      default:
        return 'Panel'
    }
  }

  // Check if panel navigation is available
  static isPanelNavigationAvailable(panelType: PanelFocus): boolean {
    const panelElement = document.querySelector(VimNavigationManager.getPanelFocusSelector(panelType))
    return panelElement !== null && panelElement.getAttribute('data-vim-available') !== 'false'
  }

  // Get available panels
  static getAvailablePanels(): PanelFocus[] {
    const availablePanels: PanelFocus[] = []
    
    if (VimNavigationManager.isPanelNavigationAvailable('left')) {
      availablePanels.push('left')
    }
    
    // Middle panel is always available
    availablePanels.push('middle')
    
    if (VimNavigationManager.isPanelNavigationAvailable('right')) {
      availablePanels.push('right')
    }
    
    return availablePanels
  }

  // Handle sequence keys (like gg for goto first)
  static handleSequenceKey(
    key: string, 
    lastKey: string | null, 
    timestamp: number,
    lastTimestamp: number
  ): { action: NavigationAction | null, consumedSequence: boolean } {
    const sequenceTimeout = 1000 // 1 second timeout for sequences
    
    // Handle 'gg' sequence
    if (key === 'g' && lastKey === 'g' && (timestamp - lastTimestamp) < sequenceTimeout) {
      return { action: 'goto-first', consumedSequence: true }
    }
    
    // Handle 'G' (shift+g) 
    if (key === 'G') {
      return { action: 'goto-last', consumedSequence: true }
    }
    
    // No sequence matched
    return { action: null, consumedSequence: false }
  }
}

// Export utilities for external use
export const panelFocusUtils = {
  getLeftPanel: VimNavigationManager.getLeftPanelFocus,
  getRightPanel: VimNavigationManager.getRightPanelFocus,
  getFocusClasses: VimNavigationManager.getPanelFocusClasses,
  isPanelFocused: VimNavigationManager.isPanelFocused,
  focusPanel: VimNavigationManager.focusPanel,
  getPanelTitle: VimNavigationManager.getPanelTitle,
  getAvailablePanels: VimNavigationManager.getAvailablePanels
}

export default VimNavigationManager