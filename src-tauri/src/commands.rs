use crate::error::Result;
use crate::clipboard::ClipboardManager;
use crate::system::SystemManager;
use crate::database::DatabaseManager;
use crate::models::{
    BackupRestoreJob
};
use crate::clipboard::ClipboardItem as ClipboardClipboardItem;
use crate::system::SystemPreferences as SystemSystemPreferences;
use crate::system::SystemInfo as SystemSystemInfo;
use crate::system::PermissionStatus as SystemPermissionStatus;
use crate::performance::PerformanceMetrics as PerfPerformanceMetrics;

#[cfg(target_os = "macos")]
use crate::mac_os::{GlobalShortcutManager, ClipboardMonitor, SystemTrayManager, TrayItem, Shortcut};

use tauri::State;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::path::PathBuf;

// =============================================
// Clipboard API Commands
// =============================================

#[tauri::command]
pub async fn clipboard_read(
    clipboard_manager: State<'_, Arc<RwLock<ClipboardManager>>>,
) -> Result<ClipboardClipboardItem> {
    let manager = clipboard_manager.read().await;
    manager.read_clipboard().await
}

#[tauri::command]
pub async fn clipboard_write(
    clipboard_manager: State<'_, Arc<RwLock<ClipboardManager>>>,
    content: String,
) -> Result<()> {
    let manager = clipboard_manager.write().await;
    manager.write_clipboard(content).await
}

#[tauri::command]
pub async fn get_clipboard_history(
    database_manager: State<'_, Arc<RwLock<DatabaseManager>>>,
    limit: Option<usize>,
) -> Result<Vec<ClipboardClipboardItem>> {
    let manager = database_manager.read().await;
    manager.get_clipboard_history(limit).await
}

#[tauri::command]
pub async fn search_clipboard_history(
    database_manager: State<'_, Arc<RwLock<DatabaseManager>>>,
    query: String,
) -> Result<Vec<ClipboardClipboardItem>> {
    let manager = database_manager.read().await;
    manager.search_clipboard_items(&query).await
}

#[tauri::command]
pub async fn add_to_clipboard_history(
    database_manager: State<'_, Arc<RwLock<DatabaseManager>>>,
    item: ClipboardClipboardItem,
) -> Result<()> {
    let manager = database_manager.write().await;
    manager.save_clipboard_item(&item).await
}

#[tauri::command]
pub async fn toggle_clipboard_favorite(
    database_manager: State<'_, Arc<RwLock<DatabaseManager>>>,
    item_id: String,
) -> Result<bool> {
    let manager = database_manager.write().await;
    manager.toggle_favorite(&item_id).await
}

#[tauri::command]
pub async fn delete_clipboard_item(
    database_manager: State<'_, Arc<RwLock<DatabaseManager>>>,
    item_id: String,
) -> Result<()> {
    let manager = database_manager.write().await;
    manager.delete_clipboard_item(&item_id).await
}

#[tauri::command]
pub async fn clear_clipboard_history(
    database_manager: State<'_, Arc<RwLock<DatabaseManager>>>,
) -> Result<()> {
    let manager = database_manager.write().await;
    manager.clear_clipboard_history().await
}

#[tauri::command]
pub async fn get_favorite_items(
    database_manager: State<'_, Arc<RwLock<DatabaseManager>>>,
) -> Result<Vec<ClipboardClipboardItem>> {
    let manager = database_manager.read().await;
    manager.get_favorite_items().await
}

#[tauri::command]
pub async fn add_tag_to_item(
    database_manager: State<'_, Arc<RwLock<DatabaseManager>>>,
    item_id: String,
    tag: String,
) -> Result<()> {
    let manager = database_manager.write().await;
    manager.add_tag_to_item(&item_id, &tag).await
}

#[tauri::command]
pub async fn remove_tag_from_item(
    database_manager: State<'_, Arc<RwLock<DatabaseManager>>>,
    item_id: String,
    tag: String,
) -> Result<()> {
    let manager = database_manager.write().await;
    manager.remove_tag_from_item(&item_id, &tag).await
}

#[tauri::command]
pub async fn get_items_by_content_type(
    database_manager: State<'_, Arc<RwLock<DatabaseManager>>>,
    content_type: String,
) -> Result<Vec<ClipboardClipboardItem>> {
    let manager = database_manager.read().await;
    manager.get_items_by_content_type(&content_type).await
}

// =============================================
// System Preferences API Commands
// =============================================

#[tauri::command]
pub async fn get_system_preferences(
    system_manager: State<'_, Arc<RwLock<SystemManager>>>,
) -> Result<SystemSystemPreferences> {
    let manager = system_manager.read().await;
    manager.get_preferences().await
}

#[tauri::command]
pub async fn update_system_preferences(
    system_manager: State<'_, Arc<RwLock<SystemManager>>>,
    preferences: SystemSystemPreferences,
) -> Result<()> {
    let manager = system_manager.write().await;
    manager.update_preferences(preferences).await
}

#[tauri::command]
pub async fn get_system_state(
    system_manager: State<'_, Arc<RwLock<SystemManager>>>,
) -> Result<crate::system::ApplicationState> {
    let manager = system_manager.read().await;
    manager.get_state().await
}

#[tauri::command]
pub async fn get_system_info(
    system_manager: State<'_, Arc<RwLock<SystemManager>>>,
) -> Result<SystemSystemInfo> {
    let manager = system_manager.read().await;
    manager.get_system_info().await
}

#[tauri::command]
pub async fn check_permissions(
    system_manager: State<'_, Arc<RwLock<SystemManager>>>,
) -> Result<SystemPermissionStatus> {
    let manager = system_manager.read().await;
    manager.check_permissions().await
}

#[tauri::command]
pub async fn request_permissions(
    system_manager: State<'_, Arc<RwLock<SystemManager>>>,
) -> Result<()> {
    let manager = system_manager.write().await;
    manager.request_permissions().await
}

#[tauri::command]
pub async fn show_notification(
    system_manager: State<'_, Arc<RwLock<SystemManager>>>,
    title: String,
    body: String,
) -> Result<()> {
    let manager = system_manager.read().await;
    manager.show_notification(&title, &body).await
}

// =============================================
// Performance Monitoring API Commands
// =============================================

#[tauri::command]
pub async fn get_performance_metrics(
    clipboard_manager: State<'_, Arc<RwLock<ClipboardManager>>>,
) -> Result<PerfPerformanceMetrics> {
    let manager = clipboard_manager.read().await;
    manager.get_performance_metrics().await
}

// =============================================
// Database Statistics API Commands
// =============================================

#[tauri::command]
pub async fn get_database_metrics(
    database_manager: State<'_, Arc<RwLock<DatabaseManager>>>,
) -> Result<crate::models::DatabaseMetrics> {
    let manager = database_manager.read().await;
    manager.get_database_metrics().await
}

#[tauri::command]
pub async fn cleanup_old_items(
    database_manager: State<'_, Arc<RwLock<DatabaseManager>>>,
    max_age_days: u32,
) -> Result<usize> {
    let manager = database_manager.write().await;
    manager.cleanup_old_items(max_age_days).await
}

// =============================================
// Backup/Restore API Commands
// =============================================

#[tauri::command]
pub async fn create_backup(
    database_manager: State<'_, Arc<RwLock<DatabaseManager>>>,
    file_path: String,
) -> Result<BackupRestoreJob> {
    let manager = database_manager.write().await;
    let backup_path = PathBuf::from(file_path);
    manager.create_backup(&backup_path).await
}

#[tauri::command]
pub async fn restore_from_backup(
    database_manager: State<'_, Arc<RwLock<DatabaseManager>>>,
    file_path: String,
) -> Result<BackupRestoreJob> {
    let manager = database_manager.write().await;
    let backup_path = PathBuf::from(file_path);
    manager.restore_from_backup(&backup_path).await
}

#[tauri::command]
pub async fn get_backup_restore_history(
    database_manager: State<'_, Arc<RwLock<DatabaseManager>>>,
    limit: Option<usize>,
) -> Result<Vec<BackupRestoreJob>> {
    let manager = database_manager.read().await;
    manager.get_backup_restore_history(limit).await
}

#[tauri::command]
pub async fn schedule_automatic_backup(
    database_manager: State<'_, Arc<RwLock<DatabaseManager>>>,
    backup_directory: String,
) -> Result<BackupRestoreJob> {
    let manager = database_manager.write().await;
    let backup_dir = PathBuf::from(backup_directory);
    manager.schedule_automatic_backup(&backup_dir).await
}

#[tauri::command]
pub async fn cleanup_old_backups(
    database_manager: State<'_, Arc<RwLock<DatabaseManager>>>,
    backup_directory: String,
    max_backups: usize,
) -> Result<usize> {
    let manager = database_manager.write().await;
    let backup_dir = PathBuf::from(backup_directory);
    manager.cleanup_old_backups(&backup_dir, max_backups).await
}

// =============================================
// Database Management API Commands
// =============================================

#[tauri::command]
pub async fn optimize_database(
    database_manager: State<'_, Arc<RwLock<DatabaseManager>>>,
) -> Result<()> {
    let manager = database_manager.write().await;
    manager.optimize_database().await
}

#[tauri::command]
pub async fn close_database(
    database_manager: State<'_, Arc<RwLock<DatabaseManager>>>,
) -> Result<()> {
    let manager = database_manager.write().await;
    manager.close().await
}

// =============================================
// macOS-specific commands
// =============================================

#[cfg(target_os = "macos")]
#[tauri::command]
pub async fn register_global_shortcut(
    shortcut_manager: State<'_, Arc<RwLock<GlobalShortcutManager>>>,
    action: String,
    key_combination: String,
) -> Result<()> {
    let manager = shortcut_manager.write().await;
    manager.register_shortcut(&action, &key_combination).await
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub async fn unregister_global_shortcut(
    shortcut_manager: State<'_, Arc<RwLock<GlobalShortcutManager>>>,
    action: String,
) -> Result<()> {
    let manager = shortcut_manager.read().await;
    manager.unregister_shortcut(&action).await
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub async fn get_global_shortcuts(
    shortcut_manager: State<'_, Arc<RwLock<GlobalShortcutManager>>>,
) -> Result<std::collections::HashMap<String, Shortcut>> {
    let manager = shortcut_manager.read().await;
    manager.get_shortcuts().await
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub async fn start_clipboard_monitoring(
    clipboard_monitor: State<'_, Arc<RwLock<ClipboardMonitor>>>,
) -> Result<()> {
    let monitor = clipboard_monitor.read().await;
    monitor.start_monitoring().await
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub async fn stop_clipboard_monitoring(
    clipboard_monitor: State<'_, Arc<RwLock<ClipboardMonitor>>>,
) -> Result<()> {
    let monitor = clipboard_monitor.read().await;
    monitor.stop_monitoring().await
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub async fn is_clipboard_monitoring(
    clipboard_monitor: State<'_, Arc<RwLock<ClipboardMonitor>>>,
) -> Result<bool> {
    let monitor = clipboard_monitor.read().await;
    Ok(monitor.is_monitoring())
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub async fn show_system_tray(
    system_tray: State<'_, Arc<RwLock<SystemTrayManager>>>,
) -> Result<()> {
    let tray = system_tray.read().await;
    tray.show_tray().await
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub async fn hide_system_tray(
    system_tray: State<'_, Arc<RwLock<SystemTrayManager>>>,
) -> Result<()> {
    let tray = system_tray.read().await;
    tray.hide_tray().await
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub async fn add_tray_menu_item(
    system_tray: State<'_, Arc<RwLock<SystemTrayManager>>>,
    item: TrayItem,
) -> Result<()> {
    let tray = system_tray.write().await;
    tray.add_menu_item(item).await
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub async fn remove_tray_menu_item(
    system_tray: State<'_, Arc<RwLock<SystemTrayManager>>>,
    item_id: String,
) -> Result<()> {
    let tray = system_tray.write().await;
    tray.remove_menu_item(&item_id).await
}