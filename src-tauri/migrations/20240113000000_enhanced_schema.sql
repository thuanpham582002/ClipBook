-- Enhanced database schema for ClipBook Tauri migration
-- This migration adds all missing tables and relationships from the data model specification

-- Add system preferences table
CREATE TABLE IF NOT EXISTS system_preferences (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    max_history_items INTEGER NOT NULL DEFAULT 1000 CHECK (max_history_items >= 10 AND max_history_items <= 10000),
    auto_favorite BOOLEAN NOT NULL DEFAULT false,
    clipboard_monitoring BOOLEAN NOT NULL DEFAULT true,
    global_shortcuts BOOLEAN NOT NULL DEFAULT true,
    theme TEXT NOT NULL DEFAULT 'system' CHECK (theme IN ('light', 'dark', 'system')),
    notification_enabled BOOLEAN NOT NULL DEFAULT true,
    backup_enabled BOOLEAN NOT NULL DEFAULT false,
    backup_interval_hours INTEGER NOT NULL DEFAULT 24 CHECK (backup_interval_hours >= 1 AND backup_interval_hours <= 168),
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Add application state table
CREATE TABLE IF NOT EXISTS application_state (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    window_visible BOOLEAN NOT NULL DEFAULT true,
    window_x INTEGER NOT NULL DEFAULT 0,
    window_y INTEGER NOT NULL DEFAULT 0,
    window_width INTEGER NOT NULL DEFAULT 800,
    window_height INTEGER NOT NULL DEFAULT 600,
    last_activity TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    session_id TEXT NOT NULL UNIQUE,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Add global shortcuts table
CREATE TABLE IF NOT EXISTS global_shortcuts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    action TEXT NOT NULL UNIQUE CHECK (action IN ('show_hide', 'copy_current', 'paste_latest', 'toggle_favorite', 'clear_history')),
    key_combination TEXT NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT true,
    registered_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_used TEXT,
    use_count INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Add performance metrics table
CREATE TABLE IF NOT EXISTS performance_metrics (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    operation_name TEXT NOT NULL,
    duration_ms REAL NOT NULL,
    success BOOLEAN NOT NULL DEFAULT true,
    error_message TEXT,
    memory_usage_mb REAL,
    timestamp TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    session_id TEXT NOT NULL,
    metadata TEXT NOT NULL DEFAULT '{}'
);

-- Add system info table
CREATE TABLE IF NOT EXISTS system_info (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    os_name TEXT NOT NULL,
    os_version TEXT NOT NULL,
    architecture TEXT NOT NULL,
    total_memory_mb INTEGER NOT NULL,
    available_memory_mb INTEGER NOT NULL,
    app_version TEXT NOT NULL,
    tauri_version TEXT NOT NULL,
    rust_version TEXT NOT NULL,
    last_updated TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Add permission status table
CREATE TABLE IF NOT EXISTS permission_status (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    permission_type TEXT NOT NULL CHECK (permission_type IN ('clipboard', 'accessibility', 'notifications', 'system_tray')),
    status TEXT NOT NULL DEFAULT 'not_determined' CHECK (status IN ('granted', 'denied', 'not_determined')),
    can_request BOOLEAN NOT NULL DEFAULT true,
    last_checked TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_requested TEXT,
    message TEXT
);

-- Add database stats table
CREATE TABLE IF NOT EXISTS database_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    total_items INTEGER NOT NULL DEFAULT 0,
    favorite_items INTEGER NOT NULL DEFAULT 0,
    text_items INTEGER NOT NULL DEFAULT 0,
    image_items INTEGER NOT NULL DEFAULT 0,
    file_items INTEGER NOT NULL DEFAULT 0,
    html_items INTEGER NOT NULL DEFAULT 0,
    total_size_bytes INTEGER NOT NULL DEFAULT 0,
    oldest_item_timestamp TEXT,
    newest_item_timestamp TEXT,
    calculated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Add system tray menu items table
CREATE TABLE IF NOT EXISTS system_tray_menu (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    item_id TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT true,
    action TEXT NOT NULL,
    position INTEGER NOT NULL DEFAULT 0,
    submenu TEXT DEFAULT NULL,
    icon_path TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Add clipboard monitoring sessions table
CREATE TABLE IF NOT EXISTS clipboard_monitoring_sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id TEXT NOT NULL UNIQUE,
    start_time TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    end_time TEXT,
    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'stopped', 'error')),
    items_detected INTEGER NOT NULL DEFAULT 0,
    errors_count INTEGER NOT NULL DEFAULT 0,
    debounce_ms INTEGER NOT NULL DEFAULT 500,
    ignore_own_changes BOOLEAN NOT NULL DEFAULT true,
    config TEXT NOT NULL DEFAULT '{}'
);

-- Add backup/restore tracking table
CREATE TABLE IF NOT EXISTS backup_restore_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    operation_type TEXT NOT NULL CHECK (operation_type IN ('backup', 'restore')),
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'in_progress', 'completed', 'failed')),
    file_path TEXT NOT NULL,
    file_size_bytes INTEGER,
    items_count INTEGER,
    start_time TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    end_time TEXT,
    error_message TEXT,
    metadata TEXT NOT NULL DEFAULT '{}'
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_system_preferences_updated_at ON system_preferences(updated_at);
CREATE INDEX IF NOT EXISTS idx_application_state_session_id ON application_state(session_id);
CREATE INDEX IF NOT EXISTS idx_global_shortcuts_action ON global_shortcuts(action);
CREATE INDEX IF NOT EXISTS idx_global_shortcuts_enabled ON global_shortcuts(enabled);
CREATE INDEX IF NOT EXISTS idx_performance_metrics_timestamp ON performance_metrics(timestamp);
CREATE INDEX IF NOT EXISTS idx_performance_metrics_operation ON performance_metrics(operation_name);
CREATE INDEX IF NOT EXISTS idx_performance_metrics_session ON performance_metrics(session_id);
CREATE INDEX IF NOT EXISTS idx_permission_status_type ON permission_status(permission_type);
CREATE INDEX IF NOT EXISTS idx_permission_status_status ON permission_status(status);
CREATE INDEX IF NOT EXISTS idx_system_tray_menu_position ON system_tray_menu(position);
CREATE INDEX IF NOT EXISTS idx_system_tray_menu_enabled ON system_tray_menu(enabled);
CREATE INDEX IF NOT EXISTS idx_monitoring_sessions_status ON clipboard_monitoring_sessions(status);
CREATE INDEX IF NOT EXISTS idx_monitoring_sessions_session ON clipboard_monitoring_sessions(session_id);
CREATE INDEX IF NOT EXISTS idx_backup_restore_operation ON backup_restore_logs(operation_type);
CREATE INDEX IF NOT EXISTS idx_backup_restore_status ON backup_restore_logs(status);
CREATE INDEX IF NOT EXISTS idx_backup_restore_timestamp ON backup_restore_logs(start_time);

-- Create full-text search indexes for searchable fields
CREATE VIRTUAL TABLE IF NOT EXISTS system_preferences_fts USING fts5(
    theme,
    content='system_preferences',
    content_rowid='rowid'
);

-- Create triggers for FTS maintenance
CREATE TRIGGER IF NOT EXISTS system_preferences_fts_insert AFTER INSERT ON system_preferences BEGIN
    INSERT INTO system_preferences_fts(rowid, theme) VALUES (new.rowid, new.theme);
END;

CREATE TRIGGER IF NOT EXISTS system_preferences_fts_delete AFTER DELETE ON system_preferences BEGIN
    INSERT INTO system_preferences_fts(system_preferences_fts, id) VALUES('delete', old.rowid);
END;

CREATE TRIGGER IF NOT EXISTS system_preferences_fts_update AFTER UPDATE ON system_preferences BEGIN
    INSERT INTO system_preferences_fts(system_preferences_fts, id) VALUES('delete', old.rowid);
    INSERT INTO system_preferences_fts(rowid, theme) VALUES (new.rowid, new.theme);
END;

-- Create view for current database statistics
CREATE VIEW IF NOT EXISTS current_database_stats AS
SELECT 
    COUNT(*) as total_items,
    SUM(CASE WHEN is_favorite = 1 THEN 1 ELSE 0 END) as favorite_items,
    SUM(CASE WHEN content_type = 'text' THEN 1 ELSE 0 END) as text_items,
    SUM(CASE WHEN content_type = 'image' THEN 1 ELSE 0 END) as image_items,
    SUM(CASE WHEN content_type = 'file' THEN 1 ELSE 0 END) as file_items,
    SUM(CASE WHEN content_type = 'html' THEN 1 ELSE 0 END) as html_items,
    COALESCE(SUM(LENGTH(content)), 0) as total_size_bytes,
    MIN(timestamp) as oldest_item_timestamp,
    MAX(timestamp) as newest_item_timestamp
FROM clipboard_items;

-- Create view for active monitoring sessions
CREATE VIEW IF NOT EXISTS active_monitoring_sessions AS
SELECT 
    session_id,
    start_time,
    items_detected,
    errors_count,
    debounce_ms,
    ignore_own_changes,
    config
FROM clipboard_monitoring_sessions 
WHERE status = 'active';

-- Create view for enabled global shortcuts
CREATE VIEW IF NOT EXISTS enabled_global_shortcuts AS
SELECT 
    action,
    key_combination,
    registered_at,
    last_used,
    use_count
FROM global_shortcuts 
WHERE enabled = 1;

-- Insert default system preferences
INSERT OR IGNORE INTO system_preferences (
    max_history_items, auto_favorite, clipboard_monitoring, global_shortcuts,
    theme, notification_enabled, backup_enabled, backup_interval_hours
) VALUES (
    1000, false, true, true,
    'system', true, false, 24
);

-- Insert default application state
INSERT OR IGNORE INTO application_state (
    window_visible, window_x, window_y, window_width, window_height, session_id
) VALUES (
    true, 0, 0, 800, 600, 
    'default-session-' || strftime('%Y%m%d%H%M%S', 'now')
);

-- Insert default global shortcuts
INSERT OR IGNORE INTO global_shortcuts (action, key_combination) VALUES 
('show_hide', 'Cmd+Shift+C'),
('copy_current', 'Cmd+Shift+V'),
('paste_latest', 'Cmd+Shift+P'),
('toggle_favorite', 'Cmd+Shift+F'),
('clear_history', 'Cmd+Shift+X');

-- Insert default system tray menu items
INSERT OR IGNORE INTO system_tray_menu (item_id, title, action, position) VALUES
('show', 'Show ClipBook', 'show_window', 1),
('hide', 'Hide ClipBook', 'hide_window', 2),
('', '', '', 3), -- separator
('copy_current', 'Copy Current', 'copy_current', 4),
('paste_latest', 'Paste Latest', 'paste_latest', 5),
('', '', '', 6), -- separator
('toggle_favorite', 'Toggle Favorite', 'toggle_favorite', 7),
('clear_history', 'Clear History', 'clear_history', 8),
('', '', '', 9), -- separator
('quit', 'Quit', 'quit_app', 10);

-- Update existing clipboard_items table to match our schema better
ALTER TABLE clipboard_items ADD COLUMN IF NOT EXISTS preview TEXT;
ALTER TABLE clipboard_items ADD COLUMN IF NOT EXISTS size_bytes INTEGER DEFAULT 0;
ALTER TABLE clipboard_items ADD COLUMN IF NOT EXISTS application TEXT DEFAULT 'unknown';
ALTER TABLE clipboard_items ADD COLUMN IF NOT EXISTS hash_value TEXT;

-- Create index for new columns
CREATE INDEX IF NOT EXISTS idx_clipboard_items_hash ON clipboard_items(hash_value);
CREATE INDEX IF NOT EXISTS idx_clipboard_items_size ON clipboard_items(size_bytes);
CREATE INDEX IF NOT EXISTS idx_clipboard_items_application ON clipboard_items(application);

-- Add trigger to automatically update preview for text content
CREATE TRIGGER IF NOT EXISTS update_text_preview 
AFTER INSERT OR UPDATE OF content ON clipboard_items
WHEN NEW.content_type = 'text' AND (NEW.preview IS NULL OR NEW.content <> OLD.content)
BEGIN
    UPDATE clipboard_items 
    SET preview = CASE 
        WHEN LENGTH(NEW.content) > 100 THEN substr(NEW.content, 1, 97) || '...'
        ELSE NEW.content 
    END
    WHERE rowid = NEW.rowid;
END;

-- Add trigger to automatically calculate size
CREATE TRIGGER IF NOT EXISTS calculate_item_size
AFTER INSERT OR UPDATE OF content ON clipboard_items
WHEN NEW.size_bytes = 0 OR (NEW.content <> OLD.content)
BEGIN
    UPDATE clipboard_items 
    SET size_bytes = LENGTH(NEW.content)
    WHERE rowid = NEW.rowid;
END;