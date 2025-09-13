use crate::error::{ClipBookError, Result};
use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use log::{info, warn, error};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shortcut {
    pub action: String,
    pub key_combination: String,
    pub enabled: bool,
}

pub struct GlobalShortcutManager {
    shortcuts: Arc<RwLock<HashMap<String, Shortcut>>>,
    registered_shortcuts: Arc<Mutex<HashMap<String, bool>>>,
    monitor_active: Arc<Mutex<bool>>,
}

impl GlobalShortcutManager {
    pub fn new() -> Result<Self> {
        let mut shortcuts = HashMap::new();
        
        // Default shortcuts
        shortcuts.insert("toggle_clipboard".to_string(), Shortcut {
            action: "toggle_clipboard".to_string(),
            key_combination: "Cmd+Shift+V".to_string(),
            enabled: true,
        });
        
        shortcuts.insert("clear_history".to_string(), Shortcut {
            action: "clear_history".to_string(),
            key_combination: "Cmd+Shift+Delete".to_string(),
            enabled: true,
        });
        
        shortcuts.insert("toggle_favorite".to_string(), Shortcut {
            action: "toggle_favorite".to_string(),
            key_combination: "Cmd+Shift+F".to_string(),
            enabled: true,
        });

        info!("Global shortcut manager initialized");
        
        Ok(Self {
            shortcuts: Arc::new(RwLock::new(shortcuts)),
            registered_shortcuts: Arc::new(Mutex::new(HashMap::new())),
            monitor_active: Arc::new(Mutex::new(false)),
        })
    }
    
    pub async fn register_shortcut(&self, action: &str, key_combination: &str) -> Result<()> {
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            
            // Use macOS osascript to register global hotkeys
            let key = self.extract_key_from_combination(key_combination)?;
            let script = format!(
                r#"
                tell application "System Events"
                    keystroke "{}" using {{command down, shift down}}
                end tell
                "#,
                key
            );
            
            let output = Command::new("osascript")
                .args(&["-e", &script])
                .output();
            
            match output {
                Ok(result) if result.status.success() => {
                    {
                        let mut registered = self.registered_shortcuts.lock().unwrap();
                        registered.insert(action.to_string(), true);
                    }
                    
                    // Update shortcuts
                    let mut shortcuts = self.shortcuts.write().await;
                    if let Some(shortcut) = shortcuts.get_mut(action) {
                        shortcut.key_combination = key_combination.to_string();
                        shortcut.enabled = true;
                    }
                    
                    info!("Registered shortcut: {} -> {}", action, key_combination);
                    Ok(())
                }
                Ok(result) => {
                    let error_msg = String::from_utf8_lossy(&result.stderr);
                    warn!("Failed to register shortcut {}: {}", action, error_msg);
                    Err(ClipBookError::SystemError(format!("Failed to register shortcut: {}", error_msg)))
                }
                Err(e) => {
                    error!("Failed to execute osascript for shortcut registration: {}", e);
                    Err(ClipBookError::SystemError(format!("Script execution failed: {}", e)))
                }
            }
        }
        
        #[cfg(not(target_os = "macos"))]
        {
            // Fallback for other platforms
            warn!("Global shortcuts not implemented for this platform");
            Ok(())
        }
    }
    
    pub async fn unregister_shortcut(&self, action: &str) -> Result<()> {
        {
            let mut registered = self.registered_shortcuts.lock().unwrap();
            registered.remove(action);
        }
        
        let mut shortcuts = self.shortcuts.write().await;
        if let Some(shortcut) = shortcuts.get_mut(action) {
            shortcut.enabled = false;
        }
        
        info!("Unregistered shortcut: {}", action);
        Ok(())
    }
    
    pub async fn get_shortcuts(&self) -> Result<HashMap<String, Shortcut>> {
        let shortcuts = self.shortcuts.read().await;
        Ok(shortcuts.clone())
    }
    
    pub async fn set_shortcut(&self, action: &str, key_combination: &str) -> Result<()> {
        // First unregister existing shortcut
        self.unregister_shortcut(action).await?;
        
        // Then register new shortcut
        self.register_shortcut(action, key_combination).await?;
        
        Ok(())
    }
    
    pub async fn toggle_shortcut(&self, action: &str, enabled: bool) -> Result<()> {
        let mut shortcuts = self.shortcuts.write().await;
        let key_combination = if let Some(shortcut) = shortcuts.get_mut(action) {
            let key_comb = shortcut.key_combination.clone();
            shortcut.enabled = enabled;
            key_comb
        } else {
            return Err(ClipBookError::SystemError(format!("Shortcut '{}' not found", action)));
        };
        
        drop(shortcuts); // Release lock before calling register_shortcut
        
        if enabled {
            // Register the shortcut
            self.register_shortcut(action, &key_combination).await?;
        } else {
            // Unregister the shortcut
            self.unregister_shortcut(action).await?;
        }
        
        info!("Toggled shortcut {}: {}", action, if enabled { "enabled" } else { "disabled" });
        Ok(())
    }
    
    pub async fn start_monitoring(&self) -> Result<()> {
        let mut active = self.monitor_active.lock().unwrap();
        if *active {
            return Ok(());
        }
        
        #[cfg(target_os = "macos")]
        {
            // Start monitoring for global shortcuts
            // This would typically use a more sophisticated event monitoring system
            // For now, we'll use a simplified approach
            
            *active = true;
            info!("Started global shortcut monitoring");
            
            // In a real implementation, you would start a background task here
            // to monitor for keyboard events using macOS APIs
        }
        
        #[cfg(not(target_os = "macos"))]
        {
            *active = true;
            warn!("Global shortcut monitoring not implemented for this platform");
        }
        
        Ok(())
    }
    
    pub async fn stop_monitoring(&self) -> Result<()> {
        let mut active = self.monitor_active.lock().unwrap();
        *active = false;
        
        // Unregister all shortcuts
        let registered = self.registered_shortcuts.lock().unwrap();
        for action in registered.keys() {
            if let Err(e) = self.unregister_shortcut(action).await {
                warn!("Failed to unregister shortcut {}: {}", action, e);
            }
        }
        
        info!("Stopped global shortcut monitoring");
        Ok(())
    }
    
    pub fn is_monitoring_active(&self) -> bool {
        *self.monitor_active.lock().unwrap()
    }
    
    fn extract_key_from_combination<'a>(&self, combination: &'a str) -> Result<&'a str> {
        // Extract the actual key from combinations like "Cmd+Shift+V"
        let parts: Vec<&str> = combination.split('+').collect();
        
        if let Some(last_part) = parts.last() {
            Ok(last_part.trim())
        } else {
            Err(ClipBookError::SystemError("Invalid key combination format".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_shortcut_manager() {
        let manager = GlobalShortcutManager::new().unwrap();
        
        // Test getting shortcuts
        let shortcuts = manager.get_shortcuts().await.unwrap();
        assert!(shortcuts.contains_key("toggle_clipboard"));
        
        // Test toggle functionality
        manager.toggle_shortcut("toggle_clipboard", false).await.unwrap();
        let shortcuts = manager.get_shortcuts().await.unwrap();
        assert!(!shortcuts["toggle_clipboard"].enabled);
    }
    
    #[test]
    fn test_key_extraction() {
        let manager = GlobalShortcutManager::new().unwrap();
        
        assert_eq!(manager.extract_key_from_combination("Cmd+Shift+V").unwrap(), "V");
        assert_eq!(manager.extract_key_from_combination("Cmd+F").unwrap(), "F");
        assert!(manager.extract_key_from_combination("Invalid").is_err());
    }
}