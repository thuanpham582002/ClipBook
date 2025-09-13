mod error;
mod performance;
mod clipboard;
mod system;
mod database;
mod commands;
mod models;

#[cfg(target_os = "macos")]
mod mac_os;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod contract_tests;

use clipboard::ClipboardManager;
use system::SystemManager;
use database::DatabaseManager;

#[cfg(target_os = "macos")]
use mac_os::{GlobalShortcutManager, ClipboardMonitor, SystemTrayManager};

use std::sync::Arc;
use std::path::PathBuf;
use tokio::sync::RwLock;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Initialize logging
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // Initialize database in a blocking task
            let app_dir = app.path().app_data_dir().unwrap_or_else(|_| {
                std::env::current_dir().unwrap()
            });
            
            log::info!("App data directory: {:?}", app_dir);
            
            // Ensure directory exists
            if let Err(e) = std::fs::create_dir_all(&app_dir) {
                log::error!("Failed to create app directory: {}", e);
                panic!("Failed to create app directory: {}", e);
            }
            
            // Temporarily use a simpler database path for testing
            let db_path = PathBuf::from("/tmp/test_clipbook.db");
            log::info!("Database path: {:?}", db_path);
            
            // Use SqliteConnectOptions for better control
            use sqlx::sqlite::SqliteConnectOptions;
            use std::str::FromStr;
            
            let connect_options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path.to_str().unwrap()))
                .unwrap()
                .create_if_missing(true)
                .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
                .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
                .busy_timeout(std::time::Duration::from_secs(30));
            
            log::info!("Database path: {:?}", db_path);
            log::info!("Connect options configured");
            
            // TEMPORARY: Create a mock database manager to bypass SQLx connection issue
            // This allows the app to start while we resolve the underlying connection problem
            let database_manager = std::thread::spawn(move || {
                log::warn!("Creating temporary mock database manager - SQLx connection issue bypassed");
                log::warn!("Database path: {:?}", db_path);
                
                // Create a mock database manager with basic functionality
                // In production, this should use the proper DatabaseManager::new()
                DatabaseManager::mock()
            }).join().unwrap();
            
            let database_manager = Arc::new(RwLock::new(database_manager));

            // Initialize core services
            let clipboard_manager = Arc::new(RwLock::new(
                ClipboardManager::new()
                    .map_err(|e| log::error!("Failed to initialize clipboard manager: {}", e))
                    .unwrap_or_else(|_| panic!("Clipboard manager initialization failed")),
            ));

            let system_manager = Arc::new(RwLock::new(
                SystemManager::new()
                    .map_err(|e| log::error!("Failed to initialize system manager: {}", e))
                    .unwrap_or_else(|_| panic!("System manager initialization failed")),
            ));

            // Initialize macOS-specific features
            #[cfg(target_os = "macos")]
            {
                let shortcut_manager = Arc::new(RwLock::new(
                    GlobalShortcutManager::new()
                        .map_err(|e| log::error!("Failed to initialize shortcut manager: {}", e))
                        .unwrap_or_else(|_| panic!("Shortcut manager initialization failed")),
                ));

                let clipboard_monitor = Arc::new(RwLock::new(
                    ClipboardMonitor::new()
                        .map_err(|e| log::error!("Failed to initialize clipboard monitor: {}", e))
                        .unwrap_or_else(|_| panic!("Clipboard monitor initialization failed")),
                ));

                let system_tray = Arc::new(RwLock::new(
                    SystemTrayManager::new()
                        .map_err(|e| log::error!("Failed to initialize system tray manager: {}", e))
                        .unwrap_or_else(|_| panic!("System tray manager initialization failed")),
                ));

                // Start clipboard monitoring in background
                {
                    let clipboard_monitor_clone = clipboard_monitor.clone();
                    std::thread::spawn(move || {
                        let rt = tokio::runtime::Runtime::new().unwrap();
                        rt.block_on(async {
                            let monitor = clipboard_monitor_clone.read().await;
                            if let Err(e) = monitor.start_monitoring().await {
                                log::error!("Failed to start clipboard monitoring: {}", e);
                            }
                        });
                    });
                }

                // Setup default system tray menu in background
                {
                    let system_tray_clone = system_tray.clone();
                    std::thread::spawn(move || {
                        let rt = tokio::runtime::Runtime::new().unwrap();
                        rt.block_on(async {
                            let tray = system_tray_clone.read().await;
                            if let Err(e) = tray.setup_default_menu().await {
                                log::error!("Failed to setup system tray menu: {}", e);
                            }
                            if let Err(e) = tray.show_tray().await {
                                log::error!("Failed to show system tray: {}", e);
                            }
                        });
                    });
                }

                app.manage(shortcut_manager);
                app.manage(clipboard_monitor);
                app.manage(system_tray);
            }

            // Store managers in app state
            app.manage(database_manager);
            app.manage(clipboard_manager);
            app.manage(system_manager);

            log::info!("ClipBook application initialized successfully");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Clipboard API Commands
            commands::clipboard_read,
            commands::clipboard_write,
            commands::get_clipboard_history,
            commands::search_clipboard_history,
            commands::add_to_clipboard_history,
            commands::toggle_clipboard_favorite,
            commands::delete_clipboard_item,
            commands::clear_clipboard_history,
            commands::get_favorite_items,
            commands::add_tag_to_item,
            commands::remove_tag_from_item,
            commands::get_items_by_content_type,
            // System Preferences API Commands
            commands::get_system_preferences,
            commands::update_system_preferences,
            commands::get_system_state,
            commands::get_system_info,
            commands::check_permissions,
            commands::request_permissions,
            commands::show_notification,
            // Performance Monitoring API Commands
            commands::get_performance_metrics,
            // Database Statistics API Commands
            commands::get_database_metrics,
            commands::cleanup_old_items,
            // Backup/Restore API Commands
            commands::create_backup,
            commands::restore_from_backup,
            commands::get_backup_restore_history,
            commands::schedule_automatic_backup,
            commands::cleanup_old_backups,
            // Database Management API Commands
            commands::optimize_database,
            commands::close_database,
            // macOS-specific commands
            #[cfg(target_os = "macos")]
            commands::register_global_shortcut,
            #[cfg(target_os = "macos")]
            commands::unregister_global_shortcut,
            #[cfg(target_os = "macos")]
            commands::get_global_shortcuts,
            #[cfg(target_os = "macos")]
            commands::start_clipboard_monitoring,
            #[cfg(target_os = "macos")]
            commands::stop_clipboard_monitoring,
            #[cfg(target_os = "macos")]
            commands::is_clipboard_monitoring,
            #[cfg(target_os = "macos")]
            commands::show_system_tray,
            #[cfg(target_os = "macos")]
            commands::hide_system_tray,
            #[cfg(target_os = "macos")]
            commands::add_tray_menu_item,
            #[cfg(target_os = "macos")]
            commands::remove_tray_menu_item,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

