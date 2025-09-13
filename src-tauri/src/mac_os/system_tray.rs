use crate::error::{ClipBookError, Result};
use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use log::{info, warn, error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrayItem {
    pub id: String,
    pub title: String,
    pub enabled: bool,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrayMenu {
    pub title: String,
    pub items: Vec<TrayItem>,
}

pub struct SystemTrayManager {
    is_visible: Arc<Mutex<bool>>,
    menu_items: Arc<RwLock<Vec<TrayItem>>>,
    tray_icon_path: Option<String>,
}

impl SystemTrayManager {
    pub fn new() -> Result<Self> {
        info!("System tray manager initialized");
        
        Ok(Self {
            is_visible: Arc::new(Mutex::new(false)),
            menu_items: Arc::new(RwLock::new(Vec::new())),
            tray_icon_path: None,
        })
    }
    
    pub async fn show_tray(&self) -> Result<()> {
        let mut visible = self.is_visible.lock().unwrap();
        if *visible {
            return Ok(());
        }
        
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            
            // Create a simple AppleScript to show a basic system tray presence
            // Note: This is a simplified implementation
            // In a real app, you'd use Tauri's system tray API or a native macOS library
            
            let script = r#"
            tell application "System Events"
                tell process "SystemUIServer"
                    # This is a placeholder - actual system tray implementation
                    # would require macOS API integration
                    do shell script "echo 'System tray would be shown here'"
                end tell
            end tell
            "#;
            
            let output = Command::new("osascript")
                .args(&["-e", script])
                .output();
            
            match output {
                Ok(_) => {
                    *visible = true;
                    info!("System tray shown");
                    Ok(())
                }
                Err(e) => {
                    error!("Failed to show system tray: {}", e);
                    Err(ClipBookError::SystemError(format!("Failed to show system tray: {}", e)))
                }
            }
        }
        
        #[cfg(not(target_os = "macos"))]
        {
            *visible = true;
            warn!("System tray not implemented for this platform");
            Ok(())
        }
    }
    
    pub async fn hide_tray(&self) -> Result<()> {
        let mut visible = self.is_visible.lock().unwrap();
        if !*visible {
            return Ok(());
        }
        
        *visible = false;
        info!("System tray hidden");
        Ok(())
    }
    
    pub fn is_tray_visible(&self) -> bool {
        *self.is_visible.lock().unwrap()
    }
    
    pub async fn set_tray_icon(&self, icon_path: &str) -> Result<()> {
        #[cfg(target_os = "macos")]
        {
            
            // Validate icon path exists
            if std::path::Path::new(icon_path).exists() {
                // Store icon path in tray icon path (would need to be mutable in real implementation)
                info!("Tray icon set to: {}", icon_path);
                Ok(())
            } else {
                Err(ClipBookError::SystemError(format!("Icon file not found: {}", icon_path)))
            }
        }
        
        #[cfg(not(target_os = "macos"))]
        {
            warn!("Tray icon setting not implemented for this platform");
            Ok(())
        }
    }
    
    pub async fn add_menu_item(&self, item: TrayItem) -> Result<()> {
        let title = item.title.clone();
        let mut items = self.menu_items.write().await;
        items.push(item);
        
        info!("Added menu item: {}", title);
        self.update_tray_menu().await
    }
    
    pub async fn remove_menu_item(&self, item_id: &str) -> Result<()> {
        let mut items = self.menu_items.write().await;
        items.retain(|item| item.id != item_id);
        
        info!("Removed menu item: {}", item_id);
        self.update_tray_menu().await
    }
    
    pub async fn update_menu_item(&self, item_id: &str, enabled: bool) -> Result<()> {
        let mut items = self.menu_items.write().await;
        
        if let Some(item) = items.iter_mut().find(|item| item.id == item_id) {
            item.enabled = enabled;
            info!("Updated menu item {}: enabled={}", item_id, enabled);
            self.update_tray_menu().await
        } else {
            Err(ClipBookError::SystemError(format!("Menu item '{}' not found", item_id)))
        }
    }
    
    pub async fn get_menu_items(&self) -> Result<Vec<TrayItem>> {
        let items = self.menu_items.read().await;
        Ok(items.clone())
    }
    
    async fn update_tray_menu(&self) -> Result<()> {
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            
            // Update the system tray menu
            // This is a simplified implementation
            let items = self.menu_items.read().await;
            
            let mut menu_script = String::new();
            for item in items.iter() {
                if item.enabled {
                    menu_script.push_str(&format!("\"{}\"\n", item.title));
                }
            }
            
            let script = format!(
                r#"
                tell application "System Events"
                    # Update system tray menu
                    # This is a placeholder implementation
                    do shell script "echo 'Tray menu updated with {} items'"
                end tell
                "#,
                items.len()
            );
            
            let _output = Command::new("osascript")
                .args(&["-e", &script])
                .output();
            
            Ok(())
        }
        
        #[cfg(not(target_os = "macos"))]
        {
            warn!("Tray menu update not implemented for this platform");
            Ok(())
        }
    }
    
    pub async fn setup_default_menu(&self) -> Result<()> {
        let default_items = vec![
            TrayItem {
                id: "show".to_string(),
                title: "Show ClipBook".to_string(),
                enabled: true,
                action: "show_window".to_string(),
            },
            TrayItem {
                id: "hide".to_string(),
                title: "Hide ClipBook".to_string(),
                enabled: true,
                action: "hide_window".to_string(),
            },
            TrayItem {
                id: "separator1".to_string(),
                title: "---".to_string(),
                enabled: true,
                action: "separator".to_string(),
            },
            TrayItem {
                id: "toggle_monitoring".to_string(),
                title: "Toggle Clipboard Monitoring".to_string(),
                enabled: true,
                action: "toggle_monitoring".to_string(),
            },
            TrayItem {
                id: "clear_history".to_string(),
                title: "Clear Clipboard History".to_string(),
                enabled: true,
                action: "clear_history".to_string(),
            },
            TrayItem {
                id: "separator2".to_string(),
                title: "---".to_string(),
                enabled: true,
                action: "separator".to_string(),
            },
            TrayItem {
                id: "preferences".to_string(),
                title: "Preferences".to_string(),
                enabled: true,
                action: "show_preferences".to_string(),
            },
            TrayItem {
                id: "about".to_string(),
                title: "About ClipBook".to_string(),
                enabled: true,
                action: "show_about".to_string(),
            },
            TrayItem {
                id: "separator3".to_string(),
                title: "---".to_string(),
                enabled: true,
                action: "separator".to_string(),
            },
            TrayItem {
                id: "quit".to_string(),
                title: "Quit ClipBook".to_string(),
                enabled: true,
                action: "quit_app".to_string(),
            },
        ];
        
        // Clear existing items
        let mut items = self.menu_items.write().await;
        items.clear();
        drop(items);
        
        // Add default items
        for item in default_items {
            self.add_menu_item(item).await?;
        }
        
        info!("Default system tray menu setup completed");
        Ok(())
    }
    
    pub async fn handle_menu_action(&self, action: &str) -> Result<()> {
        info!("Handling tray menu action: {}", action);
        
        match action {
            "show_window" => {
                // Show main window logic would go here
                info!("Action: Show window");
            }
            "hide_window" => {
                // Hide main window logic would go here
                info!("Action: Hide window");
            }
            "toggle_monitoring" => {
                // Toggle clipboard monitoring logic would go here
                info!("Action: Toggle clipboard monitoring");
            }
            "clear_history" => {
                // Clear history logic would go here
                info!("Action: Clear clipboard history");
            }
            "show_preferences" => {
                // Show preferences logic would go here
                info!("Action: Show preferences");
            }
            "show_about" => {
                // Show about dialog logic would go here
                info!("Action: Show about");
            }
            "quit_app" => {
                // Quit application logic would go here
                info!("Action: Quit application");
                std::process::exit(0);
            }
            _ => {
                warn!("Unknown tray menu action: {}", action);
            }
        }
        
        Ok(())
    }
}

impl Default for SystemTrayManager {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_system_tray_manager() {
        let tray = SystemTrayManager::new().unwrap();
        
        // Test initial state
        assert!(!tray.is_tray_visible());
        
        // Test show/hide
        tray.show_tray().await.unwrap();
        assert!(tray.is_tray_visible());
        
        tray.hide_tray().await.unwrap();
        assert!(!tray.is_tray_visible());
    }
    
    #[tokio::test]
    async fn test_menu_items() {
        let tray = SystemTrayManager::new().unwrap();
        
        // Test adding menu item
        let item = TrayItem {
            id: "test".to_string(),
            title: "Test Item".to_string(),
            enabled: true,
            action: "test_action".to_string(),
        };
        
        tray.add_menu_item(item.clone()).await.unwrap();
        
        // Test getting menu items
        let items = tray.get_menu_items().await.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].title, "Test Item");
        
        // Test removing menu item
        tray.remove_menu_item("test").await.unwrap();
        let items = tray.get_menu_items().await.unwrap();
        assert_eq!(items.len(), 0);
    }
    
    #[tokio::test]
    async fn test_default_menu() {
        let tray = SystemTrayManager::new().unwrap();
        
        // Setup default menu
        tray.setup_default_menu().await.unwrap();
        
        // Check that default items were added
        let items = tray.get_menu_items().await.unwrap();
        assert!(items.len() > 0);
        
        // Check for some expected default items
        let titles: Vec<String> = items.iter().map(|item| item.title.clone()).collect();
        assert!(titles.contains(&"Show ClipBook".to_string()));
        assert!(titles.contains(&"Quit ClipBook".to_string()));
    }
    
    #[tokio::test]
    async fn test_menu_actions() {
        let tray = SystemTrayManager::new().unwrap();
        
        // Test handling various menu actions
        let actions = vec![
            "show_window",
            "hide_window", 
            "toggle_monitoring",
            "clear_history",
            "show_preferences",
            "show_about",
            "quit_app",
            "unknown_action"
        ];
        
        for action in actions {
            // These should not panic even for unknown actions
            let result = tray.handle_menu_action(action).await;
            if action == "unknown_action" {
                // Unknown actions should still succeed but log a warning
                assert!(result.is_ok());
            } else {
                assert!(result.is_ok());
            }
        }
    }
}