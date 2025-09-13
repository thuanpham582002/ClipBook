use crate::error::{ClipBookError, Result};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::path::PathBuf;
use uuid::Uuid;
use std::collections::HashMap;

// =============================================
// Core Clipboard Data Models
// =============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardItem {
    pub id: String,
    pub content: String,
    pub content_type: ClipboardContentType,
    pub timestamp: DateTime<Utc>,
    pub app_source: Option<String>,
    pub is_favorite: bool,
    pub tags: Vec<String>,
    pub preview: Option<String>,
    pub size_bytes: u64,
    pub application: Option<String>,
    pub hash_value: Option<String>,
    pub metadata: Option<ClipboardItemMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClipboardContentType {
    Text,
    Image,
    File,
    Html,
    RichText,
    Unknown,
}

impl Default for ClipboardContentType {
    fn default() -> Self {
        ClipboardContentType::Unknown
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardItemMetadata {
    pub file_path: Option<PathBuf>,
    pub mime_type: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub file_size: Option<u64>,
    pub url: Option<String>,
    pub image_format: Option<String>,
}

// =============================================
// System Configuration Models
// =============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemPreferences {
    pub max_history_items: u32,
    pub auto_favorite: bool,
    pub clipboard_monitoring: bool,
    pub global_shortcuts: bool,
    pub theme: String,
    pub notification_enabled: bool,
    pub backup_enabled: bool,
    pub backup_interval_hours: u32,
    pub debounce_ms: u32,
    pub ignore_own_changes: bool,
    pub custom_sound_path: Option<String>,
    pub locale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationState {
    pub window_visible: bool,
    pub window_x: i32,
    pub window_y: i32,
    pub window_width: u32,
    pub window_height: u32,
    pub session_id: String,
    pub last_activity: DateTime<Utc>,
    pub ui_state: HashMap<String, serde_json::Value>,
}

// =============================================
// Global Shortcuts Models
// =============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalShortcut {
    pub action: String,
    pub key_combination: String,
    pub enabled: bool,
    pub registered_at: DateTime<Utc>,
    pub last_used: Option<DateTime<Utc>>,
    pub use_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShortcutAction {
    ShowHide,
    CopyCurrent,
    PasteLatest,
    ToggleFavorite,
    ClearHistory,
    SearchHistory,
    ShowPreferences,
    QuitApplication,
}

// =============================================
// System Tray Models
// =============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemTrayItem {
    pub id: String,
    pub title: String,
    pub enabled: bool,
    pub action: String,
    pub position: u32,
    pub submenu: Option<Vec<SystemTrayItem>>,
    pub icon_path: Option<PathBuf>,
}

// =============================================
// Performance Metrics Models
// =============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub operation_times: HashMap<String, f64>,
    pub total_operations: u64,
    pub average_operation_time_ms: f64,
    pub error_count: u64,
    pub success_rate: f64,
    pub memory_usage_mb: f64,
    pub database_metrics: DatabaseMetrics,
    pub session_id: String,
    pub calculated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseMetrics {
    pub total_items: u64,
    pub favorite_items: u64,
    pub total_size_bytes: u64,
    pub average_query_time_ms: f64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub total_operations: u64,
    pub error_count: u64,
}

// =============================================
// System Information Models
// =============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os_name: String,
    pub os_version: String,
    pub architecture: String,
    pub total_memory_mb: u64,
    pub available_memory_mb: u64,
    pub app_version: String,
    pub tauri_version: String,
    pub rust_version: String,
    pub last_updated: DateTime<Utc>,
    pub capabilities: SystemCapabilities,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemCapabilities {
    pub global_hotkeys: bool,
    pub clipboard_monitoring: bool,
    pub system_tray: bool,
    pub notifications: bool,
    pub accessibility_permissions: bool,
    pub full_disk_access: bool,
}

// =============================================
// Permission Models
// =============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionStatus {
    pub permission_type: PermissionType,
    pub status: PermissionStatusValue,
    pub can_request: bool,
    pub last_checked: DateTime<Utc>,
    pub last_requested: Option<DateTime<Utc>>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PermissionType {
    Clipboard,
    Accessibility,
    Notifications,
    SystemTray,
    FullDiskAccess,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PermissionStatusValue {
    Granted,
    Denied,
    NotDetermined,
    Restricted,
}

// =============================================
// Database Stats Models
// =============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseStats {
    pub total_items: u64,
    pub favorite_items: u64,
    pub text_items: u64,
    pub image_items: u64,
    pub file_items: u64,
    pub html_items: u64,
    pub total_size_bytes: u64,
    pub oldest_item_timestamp: Option<DateTime<Utc>>,
    pub newest_item_timestamp: Option<DateTime<Utc>>,
    pub calculated_at: DateTime<Utc>,
}

// =============================================
// Backup/Restore Models
// =============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupRestoreJob {
    pub job_id: String,
    pub operation_type: OperationType,
    pub status: JobStatus,
    pub file_path: PathBuf,
    pub file_size_bytes: Option<u64>,
    pub items_count: Option<u64>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub metadata: BackupRestoreMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationType {
    Backup,
    Restore,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum JobStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupRestoreMetadata {
    pub version: u32,
    pub app_version: String,
    pub created_at: DateTime<Utc>,
    pub description: Option<String>,
    pub compression: Option<String>,
    pub encryption: Option<String>,
}

// =============================================
// Validation Implementation
// =============================================

impl ClipboardItem {
    pub fn validate(&self) -> Result<()> {
        // Validate ID
        if self.id.len() > 36 || self.id.is_empty() {
            return Err(ClipBookError::SerializationError("Invalid ID length".to_string()));
        }
        
        Uuid::parse_str(&self.id)
            .map_err(|_| ClipBookError::SerializationError("Invalid UUID format".to_string()))?;
        
        // Validate content
        if self.content.len() > 1000000 { // 1MB max
            return Err(ClipBookError::SerializationError("Content too large".to_string()));
        }
        
        // Validate timestamp
        let now = Utc::now();
        let duration = now.signed_duration_since(self.timestamp);
        if duration.num_days() > 365 {
            return Err(ClipBookError::SerializationError("Timestamp cannot be more than 1 year in the future".to_string()));
        }
        if duration.num_days() < -365 * 10 {
            return Err(ClipBookError::SerializationError("Timestamp cannot be more than 10 years in the past".to_string()));
        }
        
        // Validate app source
        if let Some(ref source) = self.app_source {
            if source.len() > 255 {
                return Err(ClipBookError::SerializationError("App source too long".to_string()));
            }
        }
        
        // Validate tags
        if self.tags.len() > 50 {
            return Err(ClipBookError::SerializationError("Too many tags".to_string()));
        }
        
        for tag in &self.tags {
            if tag.len() > 50 {
                return Err(ClipBookError::SerializationError("Tag too long".to_string()));
            }
            if !tag.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
                return Err(ClipBookError::SerializationError("Invalid tag format".to_string()));
            }
        }
        
        // Validate preview
        if let Some(ref preview) = self.preview {
            if preview.len() > 500 {
                return Err(ClipBookError::SerializationError("Preview too long".to_string()));
            }
        }
        
        // Validate size
        if self.size_bytes > 10000000 { // 10MB
            return Err(ClipBookError::SerializationError("Size too large".to_string()));
        }
        
        // Validate application
        if let Some(ref app) = self.application {
            if app.len() > 100 {
                return Err(ClipBookError::SerializationError("Application name too long".to_string()));
            }
        }
        
        // Validate hash
        if let Some(ref hash) = self.hash_value {
            if hash.len() > 64 {
                return Err(ClipBookError::SerializationError("Hash too long".to_string()));
            }
            if !hash.chars().all(|c| c.is_ascii_hexdigit()) {
                return Err(ClipBookError::SerializationError("Invalid hash format".to_string()));
            }
        }
        
        // Validate metadata
        if let Some(ref metadata) = self.metadata {
            metadata.validate()?;
        }
        
        Ok(())
    }
}

impl ClipboardItemMetadata {
    pub fn validate(&self) -> Result<()> {
        if let Some(ref path) = self.file_path {
            if path.as_os_str().len() > 255 {
                return Err(ClipBookError::SerializationError("File path too long".to_string()));
            }
        }
        
        if let Some(ref mime) = self.mime_type {
            if mime.len() > 100 {
                return Err(ClipBookError::SerializationError("MIME type too long".to_string()));
            }
        }
        
        if let Some(size) = self.file_size {
            if size > 10 * 1024 * 1024 * 1024 { // 10GB
                return Err(ClipBookError::SerializationError("File size too large".to_string()));
            }
        }
        
        if let Some(ref format) = self.image_format {
            let valid_formats = ["PNG", "JPEG", "JPG", "GIF", "BMP", "TIFF", "WEBP", "HEIC"];
            if !valid_formats.contains(&format.to_uppercase().as_str()) {
                return Err(ClipBookError::SerializationError("Invalid image format".to_string()));
            }
        }
        
        Ok(())
    }
}

impl SystemPreferences {
    pub fn validate(&self) -> Result<()> {
        if self.max_history_items < 10 || self.max_history_items > 10000 {
            return Err(ClipBookError::SerializationError("Invalid max history items".to_string()));
        }
        
        if self.backup_interval_hours < 1 || self.backup_interval_hours > 168 {
            return Err(ClipBookError::SerializationError("Invalid backup interval".to_string()));
        }
        
        if self.debounce_ms < 100 || self.debounce_ms > 10000 {
            return Err(ClipBookError::SerializationError("Invalid debounce time".to_string()));
        }
        
        match self.theme.as_str() {
            "light" | "dark" | "system" => (),
            _ => return Err(ClipBookError::SerializationError("Invalid theme".to_string())),
        }
        
        // Validate locale format (e.g., "en_US")
        let locale_parts: Vec<&str> = self.locale.split('_').collect();
        if locale_parts.len() != 2 {
            return Err(ClipBookError::SerializationError("Invalid locale format".to_string()));
        }
        
        if locale_parts[0].len() != 2 || locale_parts[1].len() != 2 {
            return Err(ClipBookError::SerializationError("Invalid locale format".to_string()));
        }
        
        Ok(())
    }
}

impl GlobalShortcut {
    pub fn validate(&self) -> Result<()> {
        if self.action.len() > 50 || self.action.is_empty() {
            return Err(ClipBookError::SerializationError("Invalid action".to_string()));
        }
        
        if self.key_combination.len() > 20 || self.key_combination.is_empty() {
            return Err(ClipBookError::SerializationError("Invalid key combination".to_string()));
        }
        
        // Validate action
        match self.action.as_str() {
            "show_hide" | "copy_current" | "paste_latest" | "toggle_favorite" 
            | "clear_history" | "search_history" | "show_preferences" | "quit_application" => (),
            _ => return Err(ClipBookError::SerializationError("Invalid shortcut action".to_string())),
        }
        
        // Validate key combination format
        let valid_modifiers = ["Cmd", "Shift", "Ctrl", "Alt", "Option"];
        let valid_keys = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", 
                          "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
                          "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
                          "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12",
                          "Escape", "Tab", "Space", "Enter", "Return", "Backspace", "Delete",
                          "ArrowUp", "ArrowDown", "ArrowLeft", "ArrowRight"];
        
        let parts: Vec<&str> = self.key_combination.split('+').collect();
        if parts.is_empty() || parts.len() > 4 {
            return Err(ClipBookError::SerializationError("Invalid key combination format".to_string()));
        }
        
        // Last part should be a valid key
        let key = parts.last().unwrap();
        if !valid_keys.contains(key) {
            return Err(ClipBookError::SerializationError("Invalid key in combination".to_string()));
        }
        
        // Other parts should be valid modifiers
        for part in parts.iter().take(parts.len() - 1) {
            if !valid_modifiers.contains(part) {
                return Err(ClipBookError::SerializationError("Invalid modifier in combination".to_string()));
            }
        }
        
        Ok(())
    }
}

// =============================================
// Builder Implementations
// =============================================

impl ClipboardItem {
    pub fn new(content: String, content_type: ClipboardContentType) -> Self {
        let content_len = content.len();
        Self {
            id: Uuid::new_v4().to_string(),
            content,
            content_type,
            timestamp: Utc::now(),
            app_source: None,
            is_favorite: false,
            tags: Vec::new(),
            preview: None,
            size_bytes: content_len as u64,
            application: None,
            hash_value: None,
            metadata: None,
        }
    }
    
    pub fn with_app_source(mut self, app_source: String) -> Self {
        self.app_source = Some(app_source);
        self
    }
    
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }
    
    pub fn with_metadata(mut self, metadata: ClipboardItemMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

impl SystemPreferences {
    pub fn default() -> Self {
        Self {
            max_history_items: 1000,
            auto_favorite: false,
            clipboard_monitoring: true,
            global_shortcuts: true,
            theme: "system".to_string(),
            notification_enabled: true,
            backup_enabled: false,
            backup_interval_hours: 24,
            debounce_ms: 500,
            ignore_own_changes: true,
            custom_sound_path: None,
            locale: "en_US".to_string(),
        }
    }
}

impl GlobalShortcut {
    pub fn new(action: String, key_combination: String) -> Self {
        Self {
            action,
            key_combination,
            enabled: true,
            registered_at: Utc::now(),
            last_used: None,
            use_count: 0,
        }
    }
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self {
            operation_times: HashMap::new(),
            total_operations: 0,
            average_operation_time_ms: 0.0,
            error_count: 0,
            success_rate: 100.0,
            memory_usage_mb: 0.0,
            database_metrics: DatabaseMetrics::new(),
            session_id: Uuid::new_v4().to_string(),
            calculated_at: Utc::now(),
        }
    }
}

impl DatabaseMetrics {
    pub fn new() -> Self {
        Self {
            total_items: 0,
            favorite_items: 0,
            total_size_bytes: 0,
            average_query_time_ms: 0.0,
            cache_hits: 0,
            cache_misses: 0,
            total_operations: 0,
            error_count: 0,
        }
    }
}

impl SystemInfo {
    pub fn new() -> Self {
        Self {
            os_name: std::env::consts::OS.to_string(),
            os_version: "Unknown".to_string(),
            architecture: std::env::consts::ARCH.to_string(),
            total_memory_mb: 0,
            available_memory_mb: 0,
            app_version: env!("CARGO_PKG_VERSION").to_string(),
            tauri_version: "2.8.5".to_string(),
            rust_version: std::env::var("RUSTC_VERSION").unwrap_or_else(|_| "Unknown".to_string()),
            last_updated: Utc::now(),
            capabilities: SystemCapabilities::default(),
        }
    }
}

impl Default for SystemCapabilities {
    fn default() -> Self {
        Self {
            global_hotkeys: true,
            clipboard_monitoring: true,
            system_tray: true,
            notifications: true,
            accessibility_permissions: false,
            full_disk_access: false,
        }
    }
}

impl BackupRestoreJob {
    pub fn new(operation_type: OperationType, file_path: PathBuf) -> Self {
        Self {
            job_id: Uuid::new_v4().to_string(),
            operation_type,
            status: JobStatus::Pending,
            file_path,
            file_size_bytes: None,
            items_count: None,
            start_time: Utc::now(),
            end_time: None,
            error_message: None,
            metadata: BackupRestoreMetadata::new(),
        }
    }
}

impl BackupRestoreMetadata {
    pub fn new() -> Self {
        Self {
            version: 1,
            app_version: env!("CARGO_PKG_VERSION").to_string(),
            created_at: Utc::now(),
            description: None,
            compression: None,
            encryption: None,
        }
    }
}