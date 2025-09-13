-- Create initial database schema for clipboard history
CREATE TABLE clipboard_items (
    id TEXT PRIMARY KEY NOT NULL,
    content TEXT NOT NULL,
    content_type TEXT NOT NULL CHECK (content_type IN ('text', 'image', 'file', 'html', 'unknown')),
    timestamp TEXT NOT NULL,
    app_source TEXT,
    is_favorite BOOLEAN NOT NULL DEFAULT false,
    tags TEXT NOT NULL DEFAULT '[]',
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for better performance
CREATE INDEX idx_clipboard_items_timestamp ON clipboard_items(timestamp);
CREATE INDEX idx_clipboard_items_content_type ON clipboard_items(content_type);
CREATE INDEX idx_clipboard_items_is_favorite ON clipboard_items(is_favorite);
CREATE INDEX idx_clipboard_items_app_source ON clipboard_items(app_source);

-- Create full-text search index for content search
CREATE VIRTUAL TABLE clipboard_items_fts USING fts5(
    content,
    app_source,
    tags,
    content='clipboard_items',
    content_rowid='rowid'
);

-- Triggers to keep FTS index updated
CREATE TRIGGER clipboard_items_fts_insert AFTER INSERT ON clipboard_items BEGIN
    INSERT INTO clipboard_items_fts(rowid, content, app_source, tags)
    VALUES (new.rowid, new.content, new.app_source, new.tags);
END;

CREATE TRIGGER clipboard_items_fts_delete AFTER DELETE ON clipboard_items BEGIN
    INSERT INTO clipboard_items_fts(clipboard_items_fts, id) VALUES('delete', old.rowid);
END;

CREATE TRIGGER clipboard_items_fts_update AFTER UPDATE ON clipboard_items BEGIN
    INSERT INTO clipboard_items_fts(clipboard_items_fts, id) VALUES('delete', old.rowid);
    INSERT INTO clipboard_items_fts(rowid, content, app_source, tags)
    VALUES (new.rowid, new.content, new.app_source, new.tags);
END;