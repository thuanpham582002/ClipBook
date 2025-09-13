use crate::error::{ClipBookError, Result};
use crate::performance::PerformanceMonitor;
use arboard::Clipboard;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;
use log::{info, error};
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClipboardItem {
    pub id: String,
    pub content: String,
    pub content_type: ClipboardContentType,
    pub timestamp: chrono::DateTime<Utc>,
    pub app_source: Option<String>,
    pub is_favorite: bool,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ClipboardContentType {
    Text,
    Image,
    File,
    Html,
    Unknown,
}

impl From<&str> for ClipboardContentType {
    fn from(s: &str) -> Self {
        match s {
            "text/plain" => ClipboardContentType::Text,
            "text/html" => ClipboardContentType::Html,
            "image/png" => ClipboardContentType::Image,
            _ => ClipboardContentType::Unknown,
        }
    }
}

pub struct ClipboardManager {
    clipboard: Arc<RwLock<Clipboard>>,
    performance_monitor: Arc<Mutex<PerformanceMonitor>>,
    history: Arc<RwLock<Vec<ClipboardItem>>>,
    max_history_size: usize,
}

impl std::fmt::Debug for ClipboardManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClipboardManager")
            .field("performance_monitor", &self.performance_monitor)
            .field("history", &self.history)
            .field("max_history_size", &self.max_history_size)
            .field("clipboard", &"Clipboard(Arc<RwLock>)")
            .finish()
    }
}

impl ClipboardManager {
    pub fn new() -> Result<Self> {
        let clipboard = Clipboard::new()?;
        let performance_monitor = Arc::new(Mutex::new(PerformanceMonitor::new()));
        
        info!("Clipboard manager initialized");
        
        Ok(Self {
            clipboard: Arc::new(RwLock::new(clipboard)),
            performance_monitor,
            history: Arc::new(RwLock::new(Vec::new())),
            max_history_size: 1000, // Configurable
        })
    }
    
    pub async fn read_clipboard(&self) -> Result<ClipboardItem> {
        let mut monitor = self.performance_monitor.lock().unwrap();
        
        monitor.measure_operation("read_clipboard", || {
            // This would need to be async in real implementation
            self.read_clipboard_sync()
        })
    }
    
    fn read_clipboard_sync(&self) -> Result<ClipboardItem> {
        let mut clipboard = futures::executor::block_on(async {
            self.clipboard.write().await
        });
        
        match clipboard.get_text() {
            Ok(content) => {
                let item = ClipboardItem {
                    id: uuid::Uuid::new_v4().to_string(),
                    content,
                    content_type: ClipboardContentType::Text,
                    timestamp: Utc::now(),
                    app_source: self.get_active_app_name(),
                    is_favorite: false,
                    tags: Vec::new(),
                };
                
                info!("Read clipboard item: {} chars", item.content.len());
                Ok(item)
            }
            Err(e) => {
                error!("Failed to read clipboard: {}", e);
                Err(ClipBookError::ClipboardError(e.to_string()))
            }
        }
    }
    
    pub async fn write_clipboard(&self, content: String) -> Result<()> {
        let mut monitor = self.performance_monitor.lock().unwrap();
        
        monitor.measure_operation("write_clipboard", || {
            // This would need to be async in real implementation
            self.write_clipboard_sync(content)
        })
    }
    
    fn write_clipboard_sync(&self, content: String) -> Result<()> {
        let mut clipboard = futures::executor::block_on(async {
            self.clipboard.write().await
        });
        
        match clipboard.set_text(content.clone()) {
            Ok(_) => {
                info!("Wrote to clipboard: {} chars", content.len());
                Ok(())
            }
            Err(e) => {
                error!("Failed to write clipboard: {}", e);
                Err(ClipBookError::ClipboardError(e.to_string()))
            }
        }
    }
    
    pub async fn add_to_history(&self, item: ClipboardItem) -> Result<()> {
        let mut history = self.history.write().await;
        
        // Check for duplicates
        if history.iter().any(|existing| existing.content == item.content) {
            info!("Duplicate clipboard item, skipping");
            return Ok(());
        }
        
        history.push(item.clone());
        
        // Maintain history size
        if history.len() > self.max_history_size {
            history.remove(0);
        }
        
        info!("Added item to history, total: {}", history.len());
        Ok(())
    }
    
    pub async fn get_history(&self, limit: Option<usize>) -> Result<Vec<ClipboardItem>> {
        let history = self.history.read().await;
        let limit = limit.unwrap_or(50);
        
        let result = history.iter()
            .rev()
            .take(limit)
            .cloned()
            .collect();
        
        Ok(result)
    }
    
    pub async fn search_history(&self, query: &str) -> Result<Vec<ClipboardItem>> {
        let history = self.history.read().await;
        
        let results: Vec<ClipboardItem> = history.iter()
            .filter(|item| {
                item.content.to_lowercase().contains(&query.to_lowercase()) ||
                item.tags.iter().any(|tag| tag.to_lowercase().contains(&query.to_lowercase()))
            })
            .cloned()
            .collect();
        
        info!("Found {} items matching '{}'", results.len(), query);
        Ok(results)
    }
    
    pub async fn toggle_favorite(&self, item_id: &str) -> Result<bool> {
        let mut history = self.history.write().await;
        
        for item in history.iter_mut() {
            if item.id == item_id {
                item.is_favorite = !item.is_favorite;
                info!("Toggled favorite status for item {}", item_id);
                return Ok(item.is_favorite);
            }
        }
        
        Err(ClipBookError::ClipboardError(format!("Item {} not found", item_id)))
    }
    
    pub async fn delete_item(&self, item_id: &str) -> Result<()> {
        let mut history = self.history.write().await;
        
        let initial_len = history.len();
        history.retain(|item| item.id != item_id);
        
        if history.len() < initial_len {
            info!("Deleted item {}", item_id);
            Ok(())
        } else {
            Err(ClipBookError::ClipboardError(format!("Item {} not found", item_id)))
        }
    }
    
    pub async fn clear_history(&self) -> Result<()> {
        let mut history = self.history.write().await;
        let count = history.len();
        history.clear();
        
        info!("Cleared {} items from history", count);
        Ok(())
    }
    
    pub async fn get_performance_metrics(&self) -> Result<crate::performance::PerformanceMetrics> {
        let monitor = self.performance_monitor.lock().unwrap();
        Ok(monitor.get_metrics().clone())
    }
    
    fn get_active_app_name(&self) -> Option<String> {
        // This would need macOS-specific implementation
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            
            if let Ok(output) = Command::new("lsappinfo").arg("front").output() {
                if let Ok(front_app) = String::from_utf8(output.stdout) {
                    if let Some(app_name) = front_app.trim().split('"').nth(1) {
                        return Some(app_name.to_string());
                    }
                }
            }
        }
        
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test;
    
    #[tokio::test]
    async fn test_clipboard_manager() {
        let manager = ClipboardManager::new().unwrap();
        
        // Test write and read
        let test_content = "Test clipboard content".to_string();
        manager.write_clipboard(test_content.clone()).await.unwrap();
        
        let item = manager.read_clipboard().await.unwrap();
        assert_eq!(item.content, test_content);
        
        // Test history
        manager.add_to_history(item.clone()).await.unwrap();
        let history = manager.get_history(Some(10)).await.unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].content, test_content);
    }
    
    #[tokio::test]
    async fn test_search_history() {
        let manager = ClipboardManager::new().unwrap();
        
        let item1 = ClipboardItem {
            id: uuid::Uuid::new_v4().to_string(),
            content: "Hello world".to_string(),
            content_type: ClipboardContentType::Text,
            timestamp: Utc::now(),
            app_source: None,
            is_favorite: false,
            tags: vec!["greeting".to_string()],
        };
        
        let item2 = ClipboardItem {
            id: uuid::Uuid::new_v4().to_string(),
            content: "Rust programming".to_string(),
            content_type: ClipboardContentType::Text,
            timestamp: Utc::now(),
            app_source: None,
            is_favorite: false,
            tags: vec!["programming".to_string()],
        };
        
        manager.add_to_history(item1).await.unwrap();
        manager.add_to_history(item2).await.unwrap();
        
        let results = manager.search_history("Hello").await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].content, "Hello world");
    }
}