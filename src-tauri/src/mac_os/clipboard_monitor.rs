use crate::error::{ClipBookError, Result};
use crate::clipboard::ClipboardItem;
use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use log::{info, warn};
use std::time::Duration;
use tokio::time::interval;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardEvent {
    pub item: ClipboardItem,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub source: String,
}

pub type ClipboardCallback = Arc<dyn Fn(ClipboardEvent) + Send + Sync>;

pub struct ClipboardMonitor {
    is_running: Arc<Mutex<bool>>,
    last_content: Arc<RwLock<Option<String>>>,
    callbacks: Arc<RwLock<Vec<ClipboardCallback>>>,
    monitor_handle: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
}

impl ClipboardMonitor {
    pub fn new() -> Result<Self> {
        info!("Clipboard monitor initialized");
        
        Ok(Self {
            is_running: Arc::new(Mutex::new(false)),
            last_content: Arc::new(RwLock::new(None)),
            callbacks: Arc::new(RwLock::new(Vec::new())),
            monitor_handle: Arc::new(Mutex::new(None)),
        })
    }
    
    pub async fn start_monitoring(&self) -> Result<()> {
        let mut running = self.is_running.lock().unwrap();
        if *running {
            return Ok(());
        }
        
        *running = true;
        info!("Started clipboard monitoring");
        
        // Start background monitoring task
        let is_running_clone = self.is_running.clone();
        let last_content_clone = self.last_content.clone();
        let callbacks_clone = self.callbacks.clone();
        
        let handle = tokio::spawn(async move {
            let mut interval = interval(Duration::from_millis(500)); // Check every 500ms
            
            loop {
                if !*is_running_clone.lock().unwrap() {
                    break;
                }
                
                interval.tick().await;
                
                if let Err(e) = Self::check_clipboard_change(
                    &last_content_clone,
                    &callbacks_clone
                ).await {
                    warn!("Clipboard monitoring error: {}", e);
                }
            }
        });
        
        *self.monitor_handle.lock().unwrap() = Some(handle);
        Ok(())
    }
    
    pub async fn stop_monitoring(&self) -> Result<()> {
        let mut running = self.is_running.lock().unwrap();
        if !*running {
            return Ok(());
        }
        
        *running = false;
        
        // Stop the monitoring task
        if let Some(handle) = self.monitor_handle.lock().unwrap().take() {
            handle.abort();
        }
        
        info!("Stopped clipboard monitoring");
        Ok(())
    }
    
    pub fn is_monitoring(&self) -> bool {
        *self.is_running.lock().unwrap()
    }
    
    pub async fn add_callback(&self, callback: ClipboardCallback) {
        let mut callbacks = self.callbacks.write().await;
        callbacks.push(callback);
        info!("Added clipboard callback, total: {}", callbacks.len());
    }
    
    pub async fn remove_callback(&self, callback_id: usize) {
        let mut callbacks = self.callbacks.write().await;
        if callback_id < callbacks.len() {
            callbacks.remove(callback_id);
            info!("Removed clipboard callback, total: {}", callbacks.len());
        }
    }
    
    async fn check_clipboard_change(
        last_content: &Arc<RwLock<Option<String>>>,
        callbacks: &Arc<RwLock<Vec<ClipboardCallback>>>
    ) -> Result<()> {
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            
            // Use macOS pbpaste to get current clipboard content
            let output = Command::new("pbpaste")
                .output()
                .map_err(|e| ClipBookError::ClipboardError(format!("Failed to execute pbpaste: {}", e)))?;
            
            if output.status.success() {
                let content = String::from_utf8_lossy(&output.stdout).to_string();
                
                // Check if content has changed
                let mut last = last_content.write().await;
                
                if last.as_ref() != Some(&content) && !content.trim().is_empty() {
                    // Create clipboard event
                    let event = ClipboardEvent {
                        item: ClipboardItem {
                            id: uuid::Uuid::new_v4().to_string(),
                            content: content.clone(),
                            content_type: crate::clipboard::ClipboardContentType::Text,
                            timestamp: chrono::Utc::now(),
                            app_source: Self::get_active_application().await,
                            is_favorite: false,
                            tags: Vec::new(),
                        },
                        timestamp: chrono::Utc::now(),
                        source: "pbpaste".to_string(),
                    };
                    
                    // Trigger callbacks
                    let callbacks_guard = callbacks.read().await;
                    for callback in callbacks_guard.iter() {
                        callback(event.clone());
                    }
                    
                    // Update last content
                    *last = Some(content);
                    
                    info!("Detected clipboard change");
                }
            }
        }
        
        #[cfg(not(target_os = "macos"))]
        {
            // Fallback for other platforms - would need platform-specific implementation
            warn!("Clipboard monitoring not implemented for this platform");
        }
        
        Ok(())
    }
    
    #[cfg(target_os = "macos")]
    async fn get_active_application() -> Option<String> {
        use std::process::Command;
        
        // Get the frontmost application using macOS-specific commands
        if let Ok(output) = Command::new("lsappinfo").arg("front").output() {
            if let Ok(front_app) = String::from_utf8(output.stdout) {
                if let Some(app_name) = front_app.trim().split('"').nth(1) {
                    return Some(app_name.to_string());
                }
            }
        }
        
        None
    }
    
    pub async fn get_current_clipboard_content(&self) -> Result<Option<ClipboardItem>> {
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            
            let output = Command::new("pbpaste")
                .output()
                .map_err(|e| ClipBookError::ClipboardError(format!("Failed to execute pbpaste: {}", e)))?;
            
            if output.status.success() {
                let content = String::from_utf8_lossy(&output.stdout).to_string();
                
                if !content.trim().is_empty() {
                    Ok(Some(ClipboardItem {
                        id: uuid::Uuid::new_v4().to_string(),
                        content,
                        content_type: crate::clipboard::ClipboardContentType::Text,
                        timestamp: chrono::Utc::now(),
                        app_source: Self::get_active_application().await,
                        is_favorite: false,
                        tags: Vec::new(),
                    }))
                } else {
                    Ok(None)
                }
            } else {
                Ok(None)
            }
        }
        
        #[cfg(not(target_os = "macos"))]
        {
            warn!("Getting clipboard content not implemented for this platform");
            Ok(None)
        }
    }
    
    pub async fn set_clipboard_content(&self, content: &str) -> Result<()> {
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            
            let output = Command::new("pbcopy")
                .output()
                .map_err(|e| ClipBookError::ClipboardError(format!("Failed to execute pbcopy: {}", e)))?;
            
            if output.status.success() {
                info!("Set clipboard content: {} chars", content.len());
                
                // Update last content cache
                let mut last = self.last_content.write().await;
                *last = Some(content.to_string());
                
                Ok(())
            } else {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                Err(ClipBookError::ClipboardError(format!("Failed to set clipboard: {}", error_msg)))
            }
        }
        
        #[cfg(not(target_os = "macos"))]
        {
            warn!("Setting clipboard content not implemented for this platform");
            Ok(())
        }
    }
}

impl Drop for ClipboardMonitor {
    fn drop(&mut self) {
        // Ensure monitoring is stopped when the monitor is dropped
        if let Ok(handle) = self.monitor_handle.lock() {
            if handle.is_some() {
                warn!("Clipboard monitor dropped while still running");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    
    #[tokio::test]
    async fn test_clipboard_monitor() {
        let monitor = ClipboardMonitor::new().unwrap();
        
        // Test initial state
        assert!(!monitor.is_monitoring());
        
        // Test start/stop
        monitor.start_monitoring().await.unwrap();
        assert!(monitor.is_monitoring());
        
        monitor.stop_monitoring().await.unwrap();
        assert!(!monitor.is_monitoring());
    }
    
    #[tokio::test]
    async fn test_clipboard_content() {
        let monitor = ClipboardMonitor::new().unwrap();
        
        // Test setting content
        let test_content = "Test clipboard content";
        monitor.set_clipboard_content(test_content).await.unwrap();
        
        // Test getting content
        if let Ok(Some(item)) = monitor.get_current_clipboard_content().await {
            assert_eq!(item.content, test_content);
        }
    }
    
    #[tokio::test]
    async fn test_callbacks() {
        let monitor = ClipboardMonitor::new().unwrap();
        
        // Add a callback
        let callback_called = Arc::new(Mutex::new(false));
        let callback_called_clone = callback_called.clone();
        
        let callback: ClipboardCallback = Arc::new(move |_event| {
            *callback_called_clone.lock().unwrap() = true;
        });
        
        monitor.add_callback(callback).await;
        
        // In a real test, we would simulate a clipboard change
        // For now, just verify the callback was added
        let callbacks = monitor.callbacks.read().await;
        assert_eq!(callbacks.len(), 1);
    }
}