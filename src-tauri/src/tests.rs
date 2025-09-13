use crate::clipboard::{ClipboardManager, ClipboardItem, ClipboardItemType};
use crate::system::SystemManager;
use crate::error::ClipBookError;
use crate::performance::PerformanceMonitor;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    #[tokio::test]
    async fn test_clipboard_manager_creation() {
        let manager = ClipboardManager::new();
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_clipboard_item_creation() {
        let item = ClipboardItem {
            id: "test-id".to_string(),
            content: "test content".to_string(),
            timestamp: chrono::Utc::now(),
            item_type: ClipboardItemType::Text,
            favorite: false,
            tags: Vec::new(),
        };

        assert_eq!(item.id, "test-id");
        assert_eq!(item.content, "test content");
        assert_eq!(item.item_type, ClipboardItemType::Text);
        assert!(!item.favorite);
        assert!(item.tags.is_empty());
    }

    #[tokio::test]
    async fn test_system_manager_creation() {
        let manager = SystemManager::new();
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_error_handling() {
        let error = ClipBookError::ClipboardError("Test error".to_string());
        assert!(matches!(error, ClipBookError::ClipboardError(_)));
    }

    #[tokio::test]
    async fn test_performance_monitor() {
        let mut monitor = PerformanceMonitor::new();
        assert_eq!(monitor.get_alerts().len(), 0);
        
        monitor.measure_operation("test_operation", || {
            42
        });
        
        assert!(monitor.get_metrics().operation_times.contains_key("test_operation"));
    }
}