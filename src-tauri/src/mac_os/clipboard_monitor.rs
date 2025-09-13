use crate::error::{ClipBookError, Result};
use crate::clipboard::ClipboardItem;
use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use log::{info, warn, error};
use std::time::Duration;
use tokio::time::interval;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[cfg(target_os = "macos")]
use arboard::Clipboard;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardEvent {
    pub item: ClipboardItem,
    pub timestamp: DateTime<Utc>,
    pub source: String,
    pub change_type: ClipboardChangeType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClipboardChangeType {
    Text,
    Image,
    File,
    Html,
    RichText,
    Unknown,
}

pub type ClipboardCallback = Arc<dyn Fn(ClipboardEvent) + Send + Sync>;

pub struct ClipboardMonitor {
    is_running: Arc<Mutex<bool>>,
    last_content: Arc<RwLock<Option<ClipboardItem>>>,
    callbacks: Arc<RwLock<Vec<ClipboardCallback>>>,
    monitor_handle: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
    monitoring_interval: Duration,
    debounce_threshold: Duration,
    ignore_applications: Vec<String>,
    statistics: Arc<RwLock<ClipboardStatistics>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardStatistics {
    pub total_changes_detected: u64,
    pub text_changes: u64,
    pub image_changes: u64,
    pub file_changes: u64,
    pub html_changes: u64,
    pub last_change: Option<DateTime<Utc>>,
    pub average_change_interval_seconds: f64,
}

impl ClipboardMonitor {
    pub fn new() -> Result<Self> {
        info!("Clipboard monitor initialized with enhanced macOS support");
        
        Ok(Self {
            is_running: Arc::new(Mutex::new(false)),
            last_content: Arc::new(RwLock::new(None)),
            callbacks: Arc::new(RwLock::new(Vec::new())),
            monitor_handle: Arc::new(Mutex::new(None)),
            monitoring_interval: Duration::from_millis(250), // Check every 250ms
            debounce_threshold: Duration::from_millis(100), // Debounce rapid changes
            ignore_applications: vec![
                "ClipBook".to_string(), // Ignore our own app
                "SystemUIServer".to_string(), // Ignore system UI server
                "WindowServer".to_string(), // Ignore window server
            ],
            statistics: Arc::new(RwLock::new(ClipboardStatistics {
                total_changes_detected: 0,
                text_changes: 0,
                image_changes: 0,
                file_changes: 0,
                html_changes: 0,
                last_change: None,
                average_change_interval_seconds: 0.0,
            })),
        })
    }
    
    pub async fn start_monitoring(&self) -> Result<()> {
        let mut running = self.is_running.lock().unwrap();
        if *running {
            return Ok(());
        }
        
        *running = true;
        info!("Started enhanced clipboard monitoring");
        
        // Start background monitoring task
        let is_running_clone = self.is_running.clone();
        let last_content_clone = self.last_content.clone();
        let callbacks_clone = self.callbacks.clone();
        let monitoring_interval = self.monitoring_interval;
        let ignore_applications_clone = self.ignore_applications.clone();
        let statistics_clone = self.statistics.clone();
        
        let handle = tokio::spawn(async move {
            let mut interval = interval(monitoring_interval);
            let mut last_change_time = Utc::now();
            
            loop {
                if !*is_running_clone.lock().unwrap() {
                    break;
                }
                
                interval.tick().await;
                
                if let Err(e) = Self::check_clipboard_change_enhanced(
                    &last_content_clone,
                    &callbacks_clone,
                    &ignore_applications_clone,
                    &statistics_clone,
                    &mut last_change_time,
                ).await {
                    warn!("Enhanced clipboard monitoring error: {}", e);
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
    
    #[cfg(target_os = "macos")]
    async fn check_clipboard_change_enhanced(
        last_content: &Arc<RwLock<Option<ClipboardItem>>>,
        callbacks: &Arc<RwLock<Vec<ClipboardCallback>>>,
        ignore_applications: &[String],
        statistics: &Arc<RwLock<ClipboardStatistics>>,
        last_change_time: &mut DateTime<Utc>,
    ) -> Result<()> {
        // Get current clipboard content using enhanced method
        if let Ok(current_item) = Self::get_current_clipboard_item_enhanced().await {
            let mut last = last_content.write().await;
            
            // Check if content has actually changed and debounce
            let now = Utc::now();
            let time_since_last_change = now.signed_duration_since(*last_change_time).num_milliseconds();
            
            if last.as_ref() != Some(&current_item) 
                && !current_item.content.trim().is_empty()
                && time_since_last_change > 100 // Debounce threshold
            {
                // Check if we should ignore this change based on source application
                if let Some(ref app_source) = current_item.app_source {
                    if ignore_applications.contains(app_source) {
                        return Ok(());
                    }
                }
                
                // Create enhanced clipboard event
                let change_type = Self::determine_change_type(&current_item);
                let event = ClipboardEvent {
                    item: current_item.clone(),
                    timestamp: now,
                    source: current_item.app_source.clone().unwrap_or_else(|| "Unknown".to_string()),
                    change_type: change_type.clone(),
                };
                
                // Update statistics
                {
                    let mut stats = statistics.write().await;
                    stats.total_changes_detected += 1;
                    match change_type {
                        ClipboardChangeType::Text => stats.text_changes += 1,
                        ClipboardChangeType::Image => stats.image_changes += 1,
                        ClipboardChangeType::File => stats.file_changes += 1,
                        ClipboardChangeType::Html => stats.html_changes += 1,
                        ClipboardChangeType::RichText => stats.html_changes += 1, // Count as HTML for now
                        ClipboardChangeType::Unknown => {}
                    }
                    stats.last_change = Some(now);
                    
                    // Update average interval
                    if stats.total_changes_detected > 1 {
                        let total_seconds = now.signed_duration_since(stats.last_change.unwrap()).num_seconds() as f64;
                        stats.average_change_interval_seconds = total_seconds / (stats.total_changes_detected - 1) as f64;
                    }
                }
                
                // Trigger callbacks
                let callbacks_guard = callbacks.read().await;
                for callback in callbacks_guard.iter() {
                    callback(event.clone());
                }
                
                // Update last content and change time
                *last = Some(current_item);
                *last_change_time = now;
                
                info!("Detected clipboard change: {:?} from {}", change_type, event.source);
            }
        }
        
        Ok(())
    }
    
    #[cfg(target_os = "macos")]
    async fn get_current_clipboard_item_enhanced() -> Result<ClipboardItem> {
        // Try to use arboard for better clipboard access
        match Clipboard::new() {
            Ok(mut clipboard) => {
                // Try to get text content first
                if let Ok(text) = clipboard.get_text() {
                    return Ok(ClipboardItem {
                        id: Uuid::new_v4().to_string(),
                        content: text,
                        content_type: crate::clipboard::ClipboardContentType::Text,
                        timestamp: Utc::now(),
                        app_source: Self::get_active_application().await,
                        is_favorite: false,
                        tags: Vec::new(),
                    });
                }
                
                // Try to get image content (simplified for now)
                // In a real implementation, you would handle image data
                Err(ClipBookError::ClipboardError("No text content found".to_string()))
            }
            Err(e) => {
                warn!("Failed to access clipboard via arboard: {}", e);
                // Fallback to pbpaste
                Self::get_clipboard_via_pbpaste().await
            }
        }
    }
    
    #[cfg(target_os = "macos")]
    async fn get_clipboard_via_pbpaste() -> Result<ClipboardItem> {
        use std::process::Command;
        
        let output = Command::new("pbpaste")
            .output()
            .map_err(|e| ClipBookError::ClipboardError(format!("Failed to execute pbpaste: {}", e)))?;
        
        if output.status.success() {
            let content = String::from_utf8_lossy(&output.stdout).to_string();
            
            if !content.trim().is_empty() {
                return Ok(ClipboardItem {
                    id: Uuid::new_v4().to_string(),
                    content,
                    content_type: crate::clipboard::ClipboardContentType::Text,
                    timestamp: Utc::now(),
                    app_source: Self::get_active_application().await,
                    is_favorite: false,
                    tags: Vec::new(),
                });
            }
        }
        
        Err(ClipBookError::ClipboardError("No content found in clipboard".to_string()))
    }
    
    #[cfg(target_os = "macos")]
    fn determine_change_type(item: &ClipboardItem) -> ClipboardChangeType {
        match item.content_type {
            crate::clipboard::ClipboardContentType::Text => {
                // Try to detect if it's HTML by looking for HTML tags
                if item.content.contains("<") && item.content.contains(">") {
                    if item.content.to_lowercase().contains("<html") {
                        ClipboardChangeType::Html
                    } else {
                        ClipboardChangeType::Text
                    }
                } else {
                    ClipboardChangeType::Text
                }
            },
            crate::clipboard::ClipboardContentType::Image => ClipboardChangeType::Image,
            crate::clipboard::ClipboardContentType::File => ClipboardChangeType::File,
            crate::clipboard::ClipboardContentType::Html => ClipboardChangeType::Html,
            crate::clipboard::ClipboardContentType::Unknown => ClipboardChangeType::Unknown,
        }
    }
    
    #[cfg(not(target_os = "macos"))]
    async fn check_clipboard_change_enhanced(
        _last_content: &Arc<RwLock<Option<ClipboardItem>>>,
        _callbacks: &Arc<RwLock<Vec<ClipboardCallback>>>,
        _ignore_applications: &[String],
        _statistics: &Arc<RwLock<ClipboardStatistics>>,
        _last_change_time: &mut DateTime<Utc>,
    ) -> Result<()> {
        // Fallback for other platforms
        warn!("Enhanced clipboard monitoring not implemented for this platform");
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
    
    // Keep the old method for backward compatibility
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
                    // Create clipboard event (simplified for backward compatibility)
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
                        change_type: ClipboardChangeType::Text,
                    };
                    
                    // Trigger callbacks
                    let callbacks_guard = callbacks.read().await;
                    for callback in callbacks_guard.iter() {
                        callback(event.clone());
                    }
                    
                    // Update last content
                    *last = Some(content);
                    
                    info!("Detected clipboard change (legacy method)");
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
                *last = Some(ClipboardItem {
                    id: uuid::Uuid::new_v4().to_string(),
                    content: content.to_string(),
                    content_type: crate::clipboard::ClipboardContentType::Text,
                    timestamp: chrono::Utc::now(),
                    app_source: None,
                    is_favorite: false,
                    tags: Vec::new(),
                });
                
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