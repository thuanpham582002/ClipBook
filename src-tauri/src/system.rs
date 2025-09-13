use crate::error::Result;
use crate::performance::PerformanceMonitor;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use log::{info, warn};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemPreferences {
    pub max_history_size: usize,
    pub auto_paste_enabled: bool,
    pub global_shortcut_enabled: bool,
    pub start_at_login: bool,
    pub theme: String,
    pub language: String,
    pub notification_enabled: bool,
    pub performance_monitoring: bool,
}

impl Default for SystemPreferences {
    fn default() -> Self {
        Self {
            max_history_size: 1000,
            auto_paste_enabled: true,
            global_shortcut_enabled: true,
            start_at_login: false,
            theme: "system".to_string(),
            language: "en".to_string(),
            notification_enabled: true,
            performance_monitoring: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationState {
    pub is_running: bool,
    pub window_visible: bool,
    pub clipboard_monitoring: bool,
    pub last_activity: chrono::DateTime<chrono::Utc>,
    pub session_start: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub struct SystemManager {
    preferences: Arc<RwLock<SystemPreferences>>,
    state: Arc<RwLock<ApplicationState>>,
    performance_monitor: Arc<std::sync::Mutex<PerformanceMonitor>>,
    shortcuts: HashMap<String, String>,
}

impl SystemManager {
    pub fn new() -> Result<Self> {
        let preferences = Arc::new(RwLock::new(SystemPreferences::default()));
        let state = Arc::new(RwLock::new(ApplicationState {
            is_running: true,
            window_visible: true,
            clipboard_monitoring: false,
            last_activity: chrono::Utc::now(),
            session_start: chrono::Utc::now(),
        }));
        
        let mut shortcuts = HashMap::new();
        shortcuts.insert("toggle_clipboard".to_string(), "Cmd+Shift+V".to_string());
        shortcuts.insert("clear_history".to_string(), "Cmd+Shift+Delete".to_string());
        shortcuts.insert("toggle_favorite".to_string(), "Cmd+Shift+F".to_string());
        
        info!("System manager initialized");
        
        Ok(Self {
            preferences,
            state,
            performance_monitor: Arc::new(std::sync::Mutex::new(PerformanceMonitor::new())),
            shortcuts,
        })
    }
    
    pub async fn get_preferences(&self) -> Result<SystemPreferences> {
        let prefs = self.preferences.read().await;
        Ok(prefs.clone())
    }
    
    pub async fn update_preferences(&self, updates: SystemPreferences) -> Result<()> {
        let mut prefs = self.preferences.write().await;
        *prefs = updates;
        
        info!("Preferences updated");
        
        // Apply preferences immediately
        self.apply_preferences().await?;
        
        Ok(())
    }
    
    async fn apply_preferences(&self) -> Result<()> {
        let prefs = self.preferences.read().await;
        
        // Apply start at login
        #[cfg(target_os = "macos")]
        {
            if prefs.start_at_login {
                self.set_login_item(true).await?;
            } else {
                self.set_login_item(false).await?;
            }
        }
        
        // Apply theme and other preferences
        info!("Applied preferences: start_at_login={}, theme={}", 
              prefs.start_at_login, prefs.theme);
        
        Ok(())
    }
    
    #[cfg(target_os = "macos")]
    async fn set_login_item(&self, enabled: bool) -> Result<()> {
        use std::process::Command;
        
        let status = if enabled { "true" } else { "false" };
        
        let output = Command::new("osascript")
            .args(&["-e", &format!("tell application \"System Events\" to set login item of \"ClipBook\" to {}", status)])
            .output()?;
        
        if !output.status.success() {
            warn!("Failed to set login item: {}", String::from_utf8_lossy(&output.stderr));
        }
        
        Ok(())
    }
    
    pub async fn get_state(&self) -> Result<ApplicationState> {
        let state = self.state.read().await;
        Ok(state.clone())
    }
    
    pub async fn update_activity(&self) -> Result<()> {
        let mut state = self.state.write().await;
        state.last_activity = chrono::Utc::now();
        
        Ok(())
    }
    
    pub async fn set_clipboard_monitoring(&self, enabled: bool) -> Result<()> {
        let mut state = self.state.write().await;
        state.clipboard_monitoring = enabled;
        
        info!("Clipboard monitoring: {}", enabled);
        Ok(())
    }
    
    pub async fn set_window_visible(&self, visible: bool) -> Result<()> {
        let mut state = self.state.write().await;
        state.window_visible = visible;
        
        info!("Window visibility: {}", visible);
        Ok(())
    }
    
    pub async fn get_shortcuts(&self) -> Result<HashMap<String, String>> {
        Ok(self.shortcuts.clone())
    }
    
    pub async fn set_shortcut(&mut self, action: &str, shortcut: &str) -> Result<()> {
        self.shortcuts.insert(action.to_string(), shortcut.to_string());
        info!("Shortcut updated: {} -> {}", action, shortcut);
        Ok(())
    }
    
    pub async fn show_notification(&self, title: &str, body: &str) -> Result<()> {
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            
            let output = Command::new("osascript")
                .args(&[
                    "-e",
                    &format!(
                        "display notification \"{}\" with title \"{}\"",
                        body.replace("\"", "\\\""),
                        title.replace("\"", "\\\"")
                    )
                ])
                .output()?;
            
            if !output.status.success() {
                warn!("Failed to show notification: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        
        info!("Notification: {} - {}", title, body);
        Ok(())
    }
    
    pub async fn get_system_info(&self) -> Result<SystemInfo> {
        let mut monitor = self.performance_monitor.lock().unwrap();
        
        monitor.measure_operation("get_system_info", || {
            SystemInfo::new()
        })
    }
    
    pub async fn check_permissions(&self) -> Result<PermissionStatus> {
        let mut status = PermissionStatus::default();
        
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            
            // Check accessibility permissions
            let output = Command::new("osascript")
                .args(&["-e", "tell application \"System Events\" to get UI elements enabled"])
                .output()?;
            
            if output.status.success() {
                let result = String::from_utf8_lossy(&output.stdout);
                status.accessibility = result.trim() == "true";
            }
            
            // Check full disk access (simplified check)
            status.full_disk_access = true; // Simplified for now
        }
        
        info!("Permission status: {:?}", status);
        Ok(status)
    }
    
    pub async fn request_permissions(&self) -> Result<()> {
        #[cfg(target_os = "macos")]
        {
            // Open system preferences for accessibility
            use std::process::Command;
            
            Command::new("open")
                .args(&["x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility"])
                .status()?;
            
            info!("Opened accessibility preferences");
        }
        
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os_version: String,
    pub architecture: String,
    pub total_memory_mb: u64,
    pub available_memory_mb: u64,
    pub cpu_cores: usize,
    pub disk_space_gb: u64,
}

impl SystemInfo {
    pub fn new() -> Result<Self> {
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            
            // Get OS version
            let os_version = Command::new("sw_vers")
                .arg("-productVersion")
                .output()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                .unwrap_or_else(|_| "Unknown".to_string());
            
            // Get architecture
            let architecture = Command::new("uname")
                .arg("-m")
                .output()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                .unwrap_or_else(|_| "Unknown".to_string());
            
            // Get memory info
            let total_memory = Command::new("sysctl")
                .args(&["-n", "hw.memsize"])
                .output()
                .map(|o| {
                    String::from_utf8_lossy(&o.stdout)
                        .trim()
                        .parse::<u64>()
                        .unwrap_or(0) / (1024 * 1024)
                })
                .unwrap_or(0);
            
            // Get CPU info
            let cpu_cores = Command::new("sysctl")
                .args(&["-n", "hw.ncpu"])
                .output()
                .map(|o| {
                    String::from_utf8_lossy(&o.stdout)
                        .trim()
                        .parse::<usize>()
                        .unwrap_or(1)
                })
                .unwrap_or(1);
            
            Ok(Self {
                os_version,
                architecture,
                total_memory_mb: total_memory,
                available_memory_mb: total_memory, // Simplified
                cpu_cores,
                disk_space_gb: 0, // Would need more complex implementation
            })
        }
        
        #[cfg(not(target_os = "macos"))]
        {
            Ok(Self {
                os_version: "Unknown".to_string(),
                architecture: "Unknown".to_string(),
                total_memory_mb: 0,
                available_memory_mb: 0,
                cpu_cores: 1,
                disk_space_gb: 0,
            })
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PermissionStatus {
    pub accessibility: bool,
    pub full_disk_access: bool,
    pub automation: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_system_manager() {
        let manager = SystemManager::new().unwrap();
        
        // Test preferences
        let prefs = manager.get_preferences().await.unwrap();
        assert_eq!(prefs.max_history_size, 1000);
        
        // Test state
        let state = manager.get_state().await.unwrap();
        assert!(state.is_running);
        
        // Test shortcuts
        let shortcuts = manager.get_shortcuts().await.unwrap();
        assert!(shortcuts.contains_key("toggle_clipboard"));
    }
    
    #[tokio::test]
    async fn test_preferences_update() {
        let manager = SystemManager::new().unwrap();
        
        let mut prefs = manager.get_preferences().await.unwrap();
        prefs.max_history_size = 500;
        prefs.theme = "dark".to_string();
        
        manager.update_preferences(prefs).await.unwrap();
        
        let updated = manager.get_preferences().await.unwrap();
        assert_eq!(updated.max_history_size, 500);
        assert_eq!(updated.theme, "dark");
    }
}