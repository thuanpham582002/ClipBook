import { ClipBookTauriAPI, ClipboardItem, SystemPreferences, ApplicationState } from '../types/tauri';
import { MacOSService } from './macos';

export class ClipboardService {
  private static instance: ClipboardService;
  private preferences: SystemPreferences | null = null;
  private state: ApplicationState | null = null;
  private macOSService: MacOSService | null = null;

  static getInstance(): ClipboardService {
    if (!ClipboardService.instance) {
      ClipboardService.instance = new ClipboardService();
    }
    return ClipboardService.instance;
  }

  private getMacOSService(): MacOSService {
    if (!this.macOSService) {
      this.macOSService = MacOSService.getInstance();
    }
    return this.macOSService;
  }

  // Core clipboard operations
  async readClipboard(): Promise<ClipboardItem> {
    try {
      return await ClipBookTauriAPI.readClipboard();
    } catch (error) {
      console.error('Failed to read clipboard:', error);
      throw new Error('Failed to read clipboard');
    }
  }

  async writeClipboard(content: string): Promise<void> {
    try {
      await ClipBookTauriAPI.writeClipboard(content);
    } catch (error) {
      console.error('Failed to write to clipboard:', error);
      throw new Error('Failed to write to clipboard');
    }
  }

  // History management
  async getHistory(limit: number = 50): Promise<ClipboardItem[]> {
    try {
      return await ClipBookTauriAPI.getClipboardHistory(limit);
    } catch (error) {
      console.error('Failed to get clipboard history:', error);
      return [];
    }
  }

  async searchHistory(query: string): Promise<ClipboardItem[]> {
    try {
      return await ClipBookTauriAPI.searchClipboardHistory(query);
    } catch (error) {
      console.error('Failed to search clipboard history:', error);
      return [];
    }
  }

  async addToHistory(item: ClipboardItem): Promise<void> {
    try {
      await ClipBookTauriAPI.addToClipboardHistory(item);
    } catch (error) {
      console.error('Failed to add item to history:', error);
    }
  }

  async toggleFavorite(itemId: string): Promise<boolean> {
    try {
      return await ClipBookTauriAPI.toggleClipboardFavorite(itemId);
    } catch (error) {
      console.error('Failed to toggle favorite:', error);
      return false;
    }
  }

  async deleteItem(itemId: string): Promise<void> {
    try {
      await ClipBookTauriAPI.deleteClipboardItem(itemId);
    } catch (error) {
      console.error('Failed to delete item:', error);
    }
  }

  async clearHistory(): Promise<void> {
    try {
      await ClipBookTauriAPI.clearClipboardHistory();
    } catch (error) {
      console.error('Failed to clear history:', error);
    }
  }

  // System preferences
  async getPreferences(): Promise<SystemPreferences> {
    if (!this.preferences) {
      try {
        this.preferences = await ClipBookTauriAPI.getSystemPreferences();
      } catch (error) {
        console.error('Failed to get preferences:', error);
        this.preferences = this.getDefaultPreferences();
      }
    }
    return this.preferences;
  }

  async updatePreferences(preferences: SystemPreferences): Promise<void> {
    try {
      await ClipBookTauriAPI.updateSystemPreferences(preferences);
      this.preferences = preferences;
    } catch (error) {
      console.error('Failed to update preferences:', error);
      throw error;
    }
  }

  // System state
  async getState(): Promise<ApplicationState> {
    if (!this.state) {
      try {
        // Check if Tauri APIs are available
        if (!window.__TAURI__) {
          console.log('Tauri APIs not available - using default state');
          this.state = this.getDefaultState();
          return this.state;
        }
        this.state = await ClipBookTauriAPI.getSystemState();
      } catch (error) {
        console.error('Failed to get system state:', error);
        this.state = this.getDefaultState();
      }
    }
    return this.state;
  }

  // System information
  async getSystemInfo() {
    try {
      return await ClipBookTauriAPI.getSystemInfo();
    } catch (error) {
      console.error('Failed to get system info:', error);
      throw error;
    }
  }

  // Permissions
  async checkPermissions() {
    try {
      // Check if Tauri APIs are available
      if (!window.__TAURI__) {
        console.log('Tauri APIs not available - using default permissions');
        return { clipboard: true, accessibility: true };
      }
      return await ClipBookTauriAPI.checkPermissions();
    } catch (error) {
      console.error('Failed to check permissions:', error);
      throw error;
    }
  }

  async requestPermissions(): Promise<void> {
    try {
      await ClipBookTauriAPI.requestPermissions();
    } catch (error) {
      console.error('Failed to request permissions:', error);
      throw error;
    }
  }

  // Notifications
  async showNotification(title: string, body: string): Promise<void> {
    try {
      await ClipBookTauriAPI.showNotification(title, body);
    } catch (error) {
      console.error('Failed to show notification:', error);
    }
  }

  // Performance monitoring
  async getPerformanceMetrics() {
    try {
      return await ClipBookTauriAPI.getPerformanceMetrics();
    } catch (error) {
      console.error('Failed to get performance metrics:', error);
      throw error;
    }
  }

  // Helper methods
  private getDefaultPreferences(): SystemPreferences {
    return {
      max_history_size: 1000,
      auto_paste_enabled: true,
      global_shortcut_enabled: true,
      start_at_login: false,
      theme: 'system',
      language: 'en',
      notification_enabled: true,
      performance_monitoring: true,
    };
  }

  private getDefaultState(): ApplicationState {
    return {
      is_running: true,
      window_visible: true,
      clipboard_monitoring: false,
      last_activity: new Date().toISOString(),
      session_start: new Date().toISOString(),
    };
  }

  // Utility methods
  async checkSystemHealth(): Promise<{
    permissions: boolean;
    clipboardAccess: boolean;
    storage: boolean;
  }> {
    try {
      const permissions = await this.checkPermissions();
      const testItem = await this.readClipboard();
      
      return {
        permissions: permissions.accessibility,
        clipboardAccess: !!testItem,
        storage: true, // Simplified check
      };
    } catch (error) {
      console.error('System health check failed:', error);
      return {
        permissions: false,
        clipboardAccess: false,
        storage: false,
      };
    }
  }

  async initializeServices(): Promise<void> {
    try {
      // Load preferences and state
      await Promise.all([
        this.getPreferences(),
        this.getState(),
        this.checkPermissions(),
      ]);

      // Initialize macOS-specific services if available
      if (this.getMacOSService().isMacOS()) {
        await this.initializeMacOSServices();
      }

      console.log('Clipboard services initialized successfully');
    } catch (error) {
      console.error('Failed to initialize services:', error);
      throw error;
    }
  }

  // macOS-specific functionality
  async initializeMacOSServices(): Promise<void> {
    try {
      const macOSService = this.getMacOSService();
      
      // Setup default shortcuts and tray menu
      await Promise.all([
        macOSService.setupDefaultShortcuts(),
        macOSService.setupDefaultTrayMenu(),
      ]);

      console.log('macOS services initialized successfully');
    } catch (error) {
      console.warn('Failed to initialize macOS services:', error);
      // Non-fatal error, continue without macOS features
    }
  }

  // Global shortcuts management
  async registerGlobalShortcut(action: string, keyCombination: string): Promise<void> {
    if (!this.getMacOSService().isMacOS()) {
      throw new Error('Global shortcuts are only available on macOS');
    }
    
    try {
      return await this.getMacOSService().registerGlobalShortcut(action, keyCombination);
    } catch (error) {
      console.error('Failed to register global shortcut:', error);
      throw error;
    }
  }

  async unregisterGlobalShortcut(action: string): Promise<void> {
    if (!this.getMacOSService().isMacOS()) {
      return; // Silently ignore on non-macOS
    }
    
    try {
      await this.getMacOSService().unregisterGlobalShortcut(action);
    } catch (error) {
      console.error('Failed to unregister global shortcut:', error);
      throw error;
    }
  }

  async getGlobalShortcuts(): Promise<Record<string, any>> {
    if (!this.getMacOSService().isMacOS()) {
      return {};
    }
    
    try {
      return await this.getMacOSService().getGlobalShortcuts();
    } catch (error) {
      console.error('Failed to get global shortcuts:', error);
      return {};
    }
  }

  // Clipboard monitoring management
  async startClipboardMonitoring(): Promise<void> {
    if (!this.getMacOSService().isMacOS()) {
      throw new Error('Clipboard monitoring is only available on macOS');
    }
    
    try {
      return await this.getMacOSService().startClipboardMonitoring();
    } catch (error) {
      console.error('Failed to start clipboard monitoring:', error);
      throw error;
    }
  }

  async stopClipboardMonitoring(): Promise<void> {
    if (!this.getMacOSService().isMacOS()) {
      return; // Silently ignore on non-macOS
    }
    
    try {
      await this.getMacOSService().stopClipboardMonitoring();
    } catch (error) {
      console.error('Failed to stop clipboard monitoring:', error);
      throw error;
    }
  }

  async isClipboardMonitoring(): Promise<boolean> {
    if (!this.getMacOSService().isMacOS()) {
      return false;
    }
    
    try {
      return await this.getMacOSService().isClipboardMonitoring();
    } catch (error) {
      console.error('Failed to check clipboard monitoring status:', error);
      return false;
    }
  }

  // System tray management
  async showSystemTray(): Promise<void> {
    if (!this.getMacOSService().isMacOS()) {
      throw new Error('System tray is only available on macOS');
    }
    
    try {
      return await this.getMacOSService().showSystemTray();
    } catch (error) {
      console.error('Failed to show system tray:', error);
      throw error;
    }
  }

  async hideSystemTray(): Promise<void> {
    if (!this.getMacOSService().isMacOS()) {
      return; // Silently ignore on non-macOS
    }
    
    try {
      await this.getMacOSService().hideSystemTray();
    } catch (error) {
      console.error('Failed to hide system tray:', error);
      throw error;
    }
  }

  async addTrayMenuItem(item: any): Promise<void> {
    if (!this.getMacOSService().isMacOS()) {
      throw new Error('System tray is only available on macOS');
    }
    
    try {
      return await this.getMacOSService().addTrayMenuItem(item);
    } catch (error) {
      console.error('Failed to add tray menu item:', error);
      throw error;
    }
  }

  async removeTrayMenuItem(itemId: string): Promise<void> {
    if (!this.getMacOSService().isMacOS()) {
      return; // Silently ignore on non-macOS
    }
    
    try {
      await this.getMacOSService().removeTrayMenuItem(itemId);
    } catch (error) {
      console.error('Failed to remove tray menu item:', error);
      throw error;
    }
  }

  // Event handlers for real-time updates
  async onClipboardChange(callback: (item: ClipboardItem) => void): Promise<void> {
    // This would integrate with native clipboard monitoring
    // For now, we'll use polling
    const pollInterval = 1000; // 1 second
    let lastContent = '';

    const pollClipboard = async () => {
      try {
        const currentItem = await this.readClipboard();
        if (currentItem.content !== lastContent) {
          lastContent = currentItem.content;
          callback(currentItem);
        }
      } catch (error) {
        console.error('Clipboard polling error:', error);
      }
    };

    // Start polling
    setInterval(pollClipboard, pollInterval);
    
    // Initial check
    pollClipboard();
  }
}