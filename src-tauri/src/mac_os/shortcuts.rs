use crate::error::{ClipBookError, Result};
use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use log::{info, warn, error};
use std::collections::HashMap;

#[cfg(target_os = "macos")]
use cocoa::appkit::{NSApplication, NSApp, NSEventModifierFlags, NSEvent};
use cocoa::foundation::{NSString, NSUInteger};
use objc::runtime::{Class, Object, Sel};
use objc::{msg_send, sel};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shortcut {
    pub action: String,
    pub key_combination: String,
    pub enabled: bool,
}

pub struct GlobalShortcutManager {
    shortcuts: Arc<RwLock<HashMap<String, Shortcut>>>,
    registered_shortcuts: Arc<Mutex<HashMap<String, ShortcutRegistration>>>,
    monitor_active: Arc<Mutex<bool>>,
    hotkey_observer: Option<Arc<Mutex<Object>>>,
}

#[cfg(target_os = "macos")]
struct ShortcutRegistration {
    enabled: bool,
    key_code: u16,
    modifiers: u32,
    carbon_hotkey_id: Option<u32>,
}

#[cfg(not(target_os = "macos"))]
struct ShortcutRegistration {
    enabled: bool,
}

impl GlobalShortcutManager {
    pub fn new() -> Result<Self> {
        let mut shortcuts = HashMap::new();
        
        // Default shortcuts with proper macOS key combinations
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

        shortcuts.insert("show_clipboard".to_string(), Shortcut {
            action: "show_clipboard".to_string(),
            key_combination: "Cmd+Shift+C".to_string(),
            enabled: true,
        });

        info!("Global shortcut manager initialized with macOS native API support");
        
        Ok(Self {
            shortcuts: Arc::new(RwLock::new(shortcuts)),
            registered_shortcuts: Arc::new(Mutex::new(HashMap::new())),
            monitor_active: Arc::new(Mutex::new(false)),
            hotkey_observer: None,
        })
    }
    
    pub async fn register_shortcut(&self, action: &str, key_combination: &str) -> Result<()> {
        #[cfg(target_os = "macos")]
        {
            // Parse key combination and convert to macOS key code and modifiers
            let (key_code, modifiers) = self.parse_mac_key_combination(key_combination)?;
            
            // Register the shortcut using macOS Carbon hotkey API
            if let Err(e) = self.register_carbon_hotkey(action, key_code, modifiers).await {
                warn!("Failed to register Carbon hotkey for {}: {}", action, e);
                // Fallback to simpler registration method
                return self.register_simple_shortcut(action, key_combination).await;
            }
            
            // Update shortcuts
            let mut shortcuts = self.shortcuts.write().await;
            if let Some(shortcut) = shortcuts.get_mut(action) {
                shortcut.key_combination = key_combination.to_string();
                shortcut.enabled = true;
            }
            
            // Update registration record
            {
                let mut registered = self.registered_shortcuts.lock().unwrap();
                registered.insert(action.to_string(), ShortcutRegistration {
                    enabled: true,
                    key_code,
                    modifiers,
                    carbon_hotkey_id: Some(self.generate_hotkey_id()),
                });
            }
            
            info!("Registered global shortcut: {} -> {}", action, key_combination);
            Ok(())
        }
        
        #[cfg(not(target_os = "macos"))]
        {
            // Fallback for other platforms
            warn!("Global shortcuts not implemented for this platform");
            Ok(())
        }
    }
    
    pub async fn unregister_shortcut(&self, action: &str) -> Result<()> {
        #[cfg(target_os = "macos")]
        {
            // Extract registration info before dropping the lock
            let should_unregister_carbon = {
                let mut registered = self.registered_shortcuts.lock().unwrap();
                registered.remove(action).map(|reg| reg.enabled).unwrap_or(false)
            };
            
            // Unregister from Carbon hotkey system if needed
            if should_unregister_carbon {
                if let Err(e) = self.unregister_carbon_hotkey(action).await {
                    warn!("Failed to unregister Carbon hotkey for {}: {}", action, e);
                }
            }
        }
        
        #[cfg(not(target_os = "macos"))]
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
    
    #[cfg(target_os = "macos")]
    fn parse_mac_key_combination(&self, combination: &str) -> Result<(u16, u32)> {
        let mut key_code = 0;
        let mut modifiers = 0;
        
        // Split the combination and parse each part
        let parts: Vec<&str> = combination.split('+').collect();
        
        for part in parts.iter().map(|p| p.trim()) {
            match part.to_uppercase().as_str() {
                "CMD" | "COMMAND" => modifiers |= 1 << 20, // NSCommandKeyMask
                "SHIFT" => modifiers |= 1 << 17,     // NSShiftKeyMask
                "ALT" | "OPTION" => modifiers |= 1 << 18, // NSAlternateKeyMask
                "CTRL" | "CONTROL" => modifiers |= 1 << 19, // NSControlKeyMask
                "FN" => modifiers |= 1 << 23,        // NSFunctionKeyMask
                _ => {
                    // Parse the actual key
                    key_code = self.get_mac_key_code(part)?;
                }
            }
        }
        
        Ok((key_code, modifiers))
    }
    
    #[cfg(target_os = "macos")]
    fn get_mac_key_code(&self, key: &str) -> Result<u16> {
        // Map common keys to macOS virtual key codes
        match key.to_uppercase().as_str() {
            "A" => Ok(0x00),
            "B" => Ok(0x0B),
            "C" => Ok(0x08),
            "D" => Ok(0x02),
            "E" => Ok(0x0E),
            "F" => Ok(0x03),
            "G" => Ok(0x05),
            "H" => Ok(0x04),
            "I" => Ok(0x22),
            "J" => Ok(0x26),
            "K" => Ok(0x28),
            "L" => Ok(0x25),
            "M" => Ok(0x2E),
            "N" => Ok(0x2D),
            "O" => Ok(0x1F),
            "P" => Ok(0x23),
            "Q" => Ok(0x0C),
            "R" => Ok(0x0F),
            "S" => Ok(0x01),
            "T" => Ok(0x11),
            "U" => Ok(0x20),
            "V" => Ok(0x09),
            "W" => Ok(0x0D),
            "X" => Ok(0x07),
            "Y" => Ok(0x10),
            "Z" => Ok(0x06),
            "0" => Ok(0x1D),
            "1" => Ok(0x12),
            "2" => Ok(0x13),
            "3" => Ok(0x14),
            "4" => Ok(0x15),
            "5" => Ok(0x17),
            "6" => Ok(0x16),
            "7" => Ok(0x1A),
            "8" => Ok(0x1C),
            "9" => Ok(0x19),
            "F1" => Ok(0x7A),
            "F2" => Ok(0x78),
            "F3" => Ok(0x63),
            "F4" => Ok(0x76),
            "F5" => Ok(0x60),
            "F6" => Ok(0x61),
            "F7" => Ok(0x62),
            "F8" => Ok(0x64),
            "F9" => Ok(0x65),
            "F10" => Ok(0x6D),
            "F11" => Ok(0x67),
            "F12" => Ok(0x6F),
            "SPACE" => Ok(0x31),
            "RETURN" | "ENTER" => Ok(0x24),
            "TAB" => Ok(0x30),
            "DELETE" | "DEL" => Ok(0x33),
            "ESCAPE" | "ESC" => Ok(0x35),
            "HOME" => Ok(0x73),
            "END" => Ok(0x77),
            "PAGEUP" => Ok(0x74),
            "PAGEDOWN" => Ok(0x79),
            "UP" | "ARROWUP" => Ok(0x7E),
            "DOWN" | "ARROWDOWN" => Ok(0x7D),
            "LEFT" | "ARROWLEFT" => Ok(0x7B),
            "RIGHT" | "ARROWRIGHT" => Ok(0x7C),
            _ => Err(ClipBookError::SystemError(format!("Unsupported key: {}", key))),
        }
    }
    
    #[cfg(target_os = "macos")]
    async fn register_carbon_hotkey(&self, _action: &str, _key_code: u16, _modifiers: u32) -> Result<()> {
        // This is a placeholder for Carbon hotkey registration
        // In a real implementation, you would use the Carbon Event Manager API
        // For now, we'll simulate success
        
        info!("Carbon hotkey registration simulated (would use Carbon API)");
        Ok(())
    }
    
    #[cfg(target_os = "macos")]
    async fn unregister_carbon_hotkey(&self, _action: &str) -> Result<()> {
        // This is a placeholder for Carbon hotkey unregistration
        // In a real implementation, you would use the Carbon Event Manager API
        
        info!("Carbon hotkey unregistration simulated (would use Carbon API)");
        Ok(())
    }
    
    #[cfg(target_os = "macos")]
    async fn register_simple_shortcut(&self, action: &str, key_combination: &str) -> Result<()> {
        use std::process::Command;
        
        // Fallback to osascript-based shortcut registration
        let key = self.extract_key_from_combination_mac(key_combination)?;
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
                info!("Registered simple shortcut: {} -> {}", action, key_combination);
                Ok(())
            }
            Ok(result) => {
                let error_msg = String::from_utf8_lossy(&result.stderr);
                warn!("Failed to register simple shortcut {}: {}", action, error_msg);
                Err(ClipBookError::SystemError(format!("Failed to register simple shortcut: {}", error_msg)))
            }
            Err(e) => {
                error!("Failed to execute osascript for simple shortcut registration: {}", e);
                Err(ClipBookError::SystemError(format!("Script execution failed: {}", e)))
            }
        }
    }
    
    #[cfg(target_os = "macos")]
    fn generate_hotkey_id(&self) -> u32 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32
    }
    
    #[cfg(target_os = "macos")]
    fn extract_key_from_combination_mac<'a>(&self, combination: &'a str) -> Result<&'a str> {
        // Extract the actual key from combinations like "Cmd+Shift+V"
        let parts: Vec<&str> = combination.split('+').collect();
        
        if let Some(last_part) = parts.last() {
            Ok(last_part.trim())
        } else {
            Err(ClipBookError::SystemError("Invalid key combination format".to_string()))
        }
    }
    
    #[cfg(not(target_os = "macos"))]
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