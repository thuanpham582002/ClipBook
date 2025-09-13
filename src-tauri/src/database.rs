use crate::error::{Result, ClipBookError};
use crate::clipboard::ClipboardItem;
use crate::models::{DatabaseMetrics, OperationType, JobStatus, BackupRestoreJob, BackupRestoreMetadata};
use sqlx::{SqlitePool, Row};
use serde_json;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

// Implement SQLx traits for ClipboardContentType
impl sqlx::Type<sqlx::Sqlite> for crate::clipboard::ClipboardContentType {
    fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
        <str as sqlx::Type<sqlx::Sqlite>>::type_info()
    }
    
    fn compatible(ty: &sqlx::sqlite::SqliteTypeInfo) -> bool {
        <str as sqlx::Type<sqlx::Sqlite>>::compatible(ty)
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Sqlite> for crate::clipboard::ClipboardContentType {
    fn decode(value: sqlx::sqlite::SqliteValueRef<'r>) -> std::result::Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as sqlx::Decode<'r, sqlx::Sqlite>>::decode(value)?;
        match s {
            "text" => Ok(crate::clipboard::ClipboardContentType::Text),
            "image" => Ok(crate::clipboard::ClipboardContentType::Image),
            "file" => Ok(crate::clipboard::ClipboardContentType::File),
            "html" => Ok(crate::clipboard::ClipboardContentType::Html),
            "unknown" => Ok(crate::clipboard::ClipboardContentType::Unknown),
            _ => Err("Invalid content type".into()),
        }
    }
}

impl<'q> sqlx::Encode<'q, sqlx::Sqlite> for crate::clipboard::ClipboardContentType {
    fn encode_by_ref(&self, args: &mut Vec<sqlx::sqlite::SqliteArgumentValue<'q>>) -> std::result::Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        match self {
            crate::clipboard::ClipboardContentType::Text => <&str as sqlx::Encode<'q, sqlx::Sqlite>>::encode_by_ref(&"text", args),
            crate::clipboard::ClipboardContentType::Image => <&str as sqlx::Encode<'q, sqlx::Sqlite>>::encode_by_ref(&"image", args),
            crate::clipboard::ClipboardContentType::File => <&str as sqlx::Encode<'q, sqlx::Sqlite>>::encode_by_ref(&"file", args),
            crate::clipboard::ClipboardContentType::Html => <&str as sqlx::Encode<'q, sqlx::Sqlite>>::encode_by_ref(&"html", args),
            crate::clipboard::ClipboardContentType::Unknown => <&str as sqlx::Encode<'q, sqlx::Sqlite>>::encode_by_ref(&"unknown", args),
        }
    }
}

pub struct DatabaseManager {
    pool: SqlitePool,
    config: DatabaseConfig,
    metrics: Arc<RwLock<DatabaseMetrics>>,
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout_seconds: u64,
    pub idle_timeout_seconds: u64,
    pub max_lifetime_seconds: u64,
    pub enable_wal: bool,
    pub enable_foreign_keys: bool,
    pub busy_timeout_ms: u64,
    pub cache_size_kb: u32,
    pub journal_mode: String,
    pub synchronous_mode: String,
    pub temp_store: String,
    pub mmap_size_kb: u32,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            max_connections: 10,
            min_connections: 2,
            connect_timeout_seconds: 30,
            idle_timeout_seconds: 600,
            max_lifetime_seconds: 3600,
            enable_wal: true,
            enable_foreign_keys: true,
            busy_timeout_ms: 5000,
            cache_size_kb: 2000,
            journal_mode: "WAL".to_string(),
            synchronous_mode: "NORMAL".to_string(),
            temp_store: "MEMORY".to_string(),
            mmap_size_kb: 0,
        }
    }
}

impl DatabaseManager {
    pub async fn new(database_url: &str) -> Result<Self> {
        Self::with_config(database_url, DatabaseConfig::default()).await
    }
    
    pub async fn with_config(database_url: &str, config: DatabaseConfig) -> Result<Self> {
        // Create pool with basic configuration (connection pooling is handled by SqlitePool)
        let pool = SqlitePool::connect(database_url).await?;
        
        // Apply database configuration pragmas
        if config.enable_wal {
            sqlx::query("PRAGMA journal_mode = WAL")
                .execute(&pool)
                .await?;
        }
        
        if config.enable_foreign_keys {
            sqlx::query("PRAGMA foreign_keys = ON")
                .execute(&pool)
                .await?;
        }
        
        sqlx::query(&format!("PRAGMA cache_size = {}", config.cache_size_kb))
            .execute(&pool)
            .await?;
            
        sqlx::query(&format!("PRAGMA busy_timeout = {}", config.busy_timeout_ms))
            .execute(&pool)
            .await?;
        
        // Run database migrations to ensure schema is up to date
        Self::run_migrations(&pool).await?;
        
        // Initialize metrics
        let metrics = Arc::new(RwLock::new(DatabaseMetrics::new()));
        
        Ok(Self { pool, config, metrics })
    }
    
    async fn run_migrations(pool: &SqlitePool) -> Result<()> {
        // Get migration files from the migrations directory
        let migration_dir = std::path::Path::new("migrations");
        
        if !migration_dir.exists() {
            log::warn!("Migrations directory not found, skipping migrations");
            return Ok(());
        }
        
        // Read migration files sorted by name (they should be timestamped)
        let mut migration_files: Vec<_> = std::fs::read_dir(migration_dir)
            .map_err(|e| ClipBookError::DatabaseError(format!("Failed to read migrations directory: {}", e)))?
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().extension().and_then(|s| s.to_str()) == Some("sql"))
            .collect();
        
        migration_files.sort_by_key(|entry| entry.path().file_name().unwrap_or_default().to_string_lossy().to_string());
        
        // Get the current migration version from the database
        let current_version = Self::get_current_migration_version(pool).await.unwrap_or(0);
        
        log::info!("Current migration version: {}, Available migrations: {}", current_version, migration_files.len());
        
        // Apply migrations that haven't been applied yet
        for (index, entry) in migration_files.iter().enumerate() {
            let migration_version = index as i64 + 1;
            
            if migration_version > current_version {
                let migration_path = entry.path();
                let migration_name = migration_path.file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                
                log::info!("Applying migration: {}", migration_name);
                
                // Read migration SQL
                let migration_sql = std::fs::read_to_string(&migration_path)
                    .map_err(|e| ClipBookError::DatabaseError(format!("Failed to read migration file {}: {}", migration_name, e)))?;
                
                // Execute migration in a transaction
                let mut tx = pool.begin().await
                    .map_err(|e| ClipBookError::DatabaseError(format!("Failed to start transaction: {}", e)))?;
                
                // Execute the migration SQL
                for statement in migration_sql.split(';') {
                    let statement = statement.trim();
                    if !statement.is_empty() && !statement.starts_with("--") {
                        sqlx::query(statement)
                            .execute(&mut *tx)
                            .await
                            .map_err(|e| {
                                log::error!("Failed to execute migration statement: {} - Error: {}", statement, e);
                                ClipBookError::DatabaseError(format!("Migration execution failed: {}", e))
                            })?;
                    }
                }
                
                // Record the migration
                sqlx::query(
                    "INSERT INTO schema_migrations (version, name, executed_at) VALUES (?, ?, CURRENT_TIMESTAMP)"
                )
                .bind(migration_version)
                .bind(&migration_name)
                .execute(&mut *tx)
                .await
                .map_err(|e| ClipBookError::DatabaseError(format!("Failed to record migration: {}", e)))?;
                
                tx.commit().await
                    .map_err(|e| ClipBookError::DatabaseError(format!("Failed to commit migration: {}", e)))?;
                
                log::info!("Successfully applied migration: {}", migration_name);
            }
        }
        
        Ok(())
    }
    
    async fn get_current_migration_version(pool: &SqlitePool) -> Result<i64> {
        // Create schema_migrations table if it doesn't exist
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS schema_migrations (
                version INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                executed_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            )"
        )
        .execute(pool)
        .await
        .map_err(|e| ClipBookError::DatabaseError(format!("Failed to create schema_migrations table: {}", e)))?;
        
        // Get the latest applied migration version
        let result = sqlx::query(
            "SELECT COALESCE(MAX(version), 0) as version FROM schema_migrations"
        )
        .fetch_one(pool)
        .await
        .map_err(|e| ClipBookError::DatabaseError(format!("Failed to get current migration version: {}", e)))?;
        
        let version: i64 = result.get("version");
        Ok(version)
    }
    
    pub async fn save_clipboard_item(&self, item: &ClipboardItem) -> Result<()> {
        let tags_json = serde_json::to_string(&item.tags)?;
        
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO clipboard_items 
            (id, content, content_type, timestamp, app_source, is_favorite, tags)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&item.id)
        .bind(&item.content)
        .bind(&item.content_type)
        .bind(&item.timestamp)
        .bind(&item.app_source)
        .bind(item.is_favorite)
        .bind(&tags_json)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn get_clipboard_history(&self, limit: Option<usize>) -> Result<Vec<ClipboardItem>> {
        let limit = limit.unwrap_or(100);
        
        let rows = sqlx::query(
            r#"
            SELECT id, content, content_type, timestamp, app_source, is_favorite, tags
            FROM clipboard_items
            ORDER BY timestamp DESC
            LIMIT ?
            "#
        )
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await?;
        
        let mut items = Vec::new();
        for row in rows {
            let tags: String = row.get("tags");
            let tags: Vec<String> = serde_json::from_str(&tags).unwrap_or_default();
            
            items.push(ClipboardItem {
                id: row.get("id"),
                content: row.get("content"),
                content_type: row.get("content_type"),
                timestamp: row.get("timestamp"),
                app_source: row.get("app_source"),
                is_favorite: row.get("is_favorite"),
                tags,
            });
        }
        
        Ok(items)
    }
    
    pub async fn search_clipboard_items(&self, query: &str) -> Result<Vec<ClipboardItem>> {
        let search_pattern = format!("%{}%", query);
        
        let rows = sqlx::query(
            r#"
            SELECT id, content, content_type, timestamp, app_source, is_favorite, tags
            FROM clipboard_items
            WHERE content LIKE ? OR app_source LIKE ? OR tags LIKE ?
            ORDER BY timestamp DESC
            LIMIT 100
            "#
        )
        .bind(&search_pattern)
        .bind(&search_pattern)
        .bind(&search_pattern)
        .fetch_all(&self.pool)
        .await?;
        
        let mut items = Vec::new();
        for row in rows {
            let tags: String = row.get("tags");
            let tags: Vec<String> = serde_json::from_str(&tags).unwrap_or_default();
            
            items.push(ClipboardItem {
                id: row.get("id"),
                content: row.get("content"),
                content_type: row.get("content_type"),
                timestamp: row.get("timestamp"),
                app_source: row.get("app_source"),
                is_favorite: row.get("is_favorite"),
                tags,
            });
        }
        
        Ok(items)
    }
    
    pub async fn toggle_favorite(&self, item_id: &str) -> Result<bool> {
        let row = sqlx::query(
            "UPDATE clipboard_items SET is_favorite = NOT is_favorite WHERE id = ? RETURNING is_favorite"
        )
        .bind(item_id)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(row.get("is_favorite"))
    }
    
    pub async fn delete_clipboard_item(&self, item_id: &str) -> Result<()> {
        sqlx::query("DELETE FROM clipboard_items WHERE id = ?")
            .bind(item_id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
    
    pub async fn clear_clipboard_history(&self) -> Result<()> {
        sqlx::query("DELETE FROM clipboard_items")
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
    
    pub async fn get_favorite_items(&self) -> Result<Vec<ClipboardItem>> {
        let rows = sqlx::query(
            r#"
            SELECT id, content, content_type, timestamp, app_source, is_favorite, tags
            FROM clipboard_items
            WHERE is_favorite = true
            ORDER BY timestamp DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut items = Vec::new();
        for row in rows {
            let tags: String = row.get("tags");
            let tags: Vec<String> = serde_json::from_str(&tags).unwrap_or_default();
            
            items.push(ClipboardItem {
                id: row.get("id"),
                content: row.get("content"),
                content_type: row.get("content_type"),
                timestamp: row.get("timestamp"),
                app_source: row.get("app_source"),
                is_favorite: row.get("is_favorite"),
                tags,
            });
        }
        
        Ok(items)
    }
    
    pub async fn add_tag_to_item(&self, item_id: &str, tag: &str) -> Result<()> {
        // Get current tags
        let row = sqlx::query("SELECT tags FROM clipboard_items WHERE id = ?")
            .bind(item_id)
            .fetch_one(&self.pool)
            .await?;
        
        let mut tags: Vec<String> = serde_json::from_str(&row.get::<String, _>("tags")).unwrap_or_default();
        
        // Add new tag if it doesn't exist
        if !tags.contains(&tag.to_string()) {
            tags.push(tag.to_string());
            let tags_json = serde_json::to_string(&tags)?;
            
            sqlx::query(
                "UPDATE clipboard_items SET tags = ? WHERE id = ?"
            )
            .bind(&tags_json)
            .bind(item_id)
            .execute(&self.pool)
            .await?;
        }
        
        Ok(())
    }
    
    pub async fn remove_tag_from_item(&self, item_id: &str, tag: &str) -> Result<()> {
        // Get current tags
        let row = sqlx::query("SELECT tags FROM clipboard_items WHERE id = ?")
            .bind(item_id)
            .fetch_one(&self.pool)
            .await?;
        
        let mut tags: Vec<String> = serde_json::from_str(&row.get::<String, _>("tags")).unwrap_or_default();
        
        // Remove tag if it exists
        tags.retain(|t| t != tag);
        let tags_json = serde_json::to_string(&tags)?;
        
        sqlx::query(
            "UPDATE clipboard_items SET tags = ? WHERE id = ?"
        )
        .bind(&tags_json)
        .bind(item_id)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn get_items_by_content_type(&self, content_type: &str) -> Result<Vec<ClipboardItem>> {
        let rows = sqlx::query(
            r#"
            SELECT id, content, content_type, timestamp, app_source, is_favorite, tags
            FROM clipboard_items
            WHERE content_type = ?
            ORDER BY timestamp DESC
            LIMIT 100
            "#
        )
        .bind(content_type)
        .fetch_all(&self.pool)
        .await?;
        
        let mut items = Vec::new();
        for row in rows {
            let tags: String = row.get("tags");
            let tags: Vec<String> = serde_json::from_str(&tags).unwrap_or_default();
            
            items.push(ClipboardItem {
                id: row.get("id"),
                content: row.get("content"),
                content_type: row.get("content_type"),
                timestamp: row.get("timestamp"),
                app_source: row.get("app_source"),
                is_favorite: row.get("is_favorite"),
                tags,
            });
        }
        
        Ok(items)
    }
    
    pub async fn get_database_stats(&self) -> Result<DatabaseStats> {
        let row = sqlx::query(
            r#"
            SELECT 
                COUNT(*) as total_items,
                SUM(CASE WHEN is_favorite = true THEN 1 ELSE 0 END) as favorite_count,
                COUNT(DISTINCT content_type) as unique_content_types,
                MIN(timestamp) as earliest_item,
                MAX(timestamp) as latest_item
            FROM clipboard_items
            "#
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(DatabaseStats {
            total_items: row.get::<i64, _>("total_items") as usize,
            favorite_count: row.get::<i64, _>("favorite_count") as usize,
            unique_content_types: row.get::<i64, _>("unique_content_types") as usize,
            earliest_item: row.get("earliest_item"),
            latest_item: row.get("latest_item"),
        })
    }
    
    pub async fn cleanup_old_items(&self, max_age_days: u32) -> Result<usize> {
        let cutoff_date = Utc::now() - chrono::Duration::days(max_age_days as i64);
        
        let result = sqlx::query("DELETE FROM clipboard_items WHERE timestamp < ?")
            .bind(cutoff_date)
            .execute(&self.pool)
            .await?;
        
        Ok(result.rows_affected() as usize)
    }
    
    // =============================================
    // Connection Pool Monitoring Methods
    // =============================================
    
    pub async fn get_pool_stats(&self) -> Result<ConnectionPoolStats> {
        let pool = &self.pool;
        let size = pool.size();
        let num_idle = pool.num_idle();
        let num_acquire = num_idle; // This is an approximation
        
        Ok(ConnectionPoolStats {
            max_size: size,
            current_size: num_acquire as u32,
            idle_connections: num_idle as u32,
            active_connections: (num_acquire.saturating_sub(num_idle)) as u32,
            config: self.config.clone(),
        })
    }
    
    pub async fn get_database_metrics(&self) -> Result<DatabaseMetrics> {
        let metrics = self.metrics.read().await;
        Ok(metrics.clone())
    }
    
    pub async fn update_query_metrics(&self, _operation: &str, duration_ms: f64, success: bool) {
        let mut metrics = self.metrics.write().await;
        metrics.total_operations += 1;
        metrics.average_query_time_ms = 
            (metrics.average_query_time_ms * (metrics.total_operations - 1) as f64 + duration_ms) / metrics.total_operations as f64;
        
        if !success {
            metrics.error_count += 1;
        }
    }
    
    pub async fn update_cache_metrics(&self, hit: bool) {
        let mut metrics = self.metrics.write().await;
        if hit {
            metrics.cache_hits += 1;
        } else {
            metrics.cache_misses += 1;
        }
    }
    
    pub async fn get_performance_report(&self) -> Result<DatabasePerformanceReport> {
        let pool_stats = self.get_pool_stats().await?;
        let metrics = self.get_database_metrics().await?;
        
        let cache_hit_rate = if metrics.cache_hits + metrics.cache_misses > 0 {
            metrics.cache_hits as f64 / (metrics.cache_hits + metrics.cache_misses) as f64
        } else {
            0.0
        };
        
        let error_rate = if metrics.total_operations > 0 {
            metrics.error_count as f64 / metrics.total_operations as f64
        } else {
            0.0
        };
        
        Ok(DatabasePerformanceReport {
            pool_stats,
            database_metrics: metrics,
            cache_hit_rate,
            error_rate,
            generated_at: Utc::now(),
        })
    }
    
    pub async fn health_check(&self) -> Result<DatabaseHealth> {
        let start = std::time::Instant::now();
        
        // Test basic connectivity
        let result = sqlx::query("SELECT 1")
            .fetch_one(&self.pool)
            .await;
        
        let duration = start.elapsed();
        
        match result {
            Ok(_) => Ok(DatabaseHealth {
                healthy: true,
                response_time_ms: duration.as_millis() as f64,
                pool_size: self.pool.size(),
                last_check: Utc::now(),
                error: None,
            }),
            Err(e) => Ok(DatabaseHealth {
                healthy: false,
                response_time_ms: duration.as_millis() as f64,
                pool_size: self.pool.size(),
                last_check: Utc::now(),
                error: Some(format!("Health check failed: {}", e)),
            }),
        }
    }
    
    pub async fn optimize_database(&self) -> Result<()> {
        log::info!("Starting database optimization");
        
        // Run ANALYZE to update statistics
        sqlx::query("ANALYZE")
            .execute(&self.pool)
            .await?;
        
        // VACUUM if needed (this can be expensive, so we'll check fragmentation first)
        let fragmentation_check = sqlx::query(
            "SELECT COUNT(*) as fragmented_pages FROM dbstat WHERE name='sqlite_master' AND (pages*1.0/aggregate_pages) < 0.8"
        )
        .fetch_one(&self.pool)
        .await?;
        
        let fragmented_pages: i64 = fragmentation_check.get("fragmented_pages");
        if fragmented_pages > 100 {
            log::info!("Database fragmented ({} pages), running VACUUM", fragmented_pages);
            sqlx::query("VACUUM")
                .execute(&self.pool)
                .await?;
        }
        
        // Update PRAGMAs for optimal performance
        let pragmas = vec![
            ("PRAGMA wal_checkpoint(TRUNCATE)", None::<&str>),
            ("PRAGMA optimize", None::<&str>),
            ("PRAGMA shrink_memory", None::<&str>),
        ];
        
        for (pragma, value) in pragmas {
            let query = if let Some(v) = value {
                format!("{} {}", pragma, v)
            } else {
                pragma.to_string()
            };
            
            sqlx::query(&query)
                .execute(&self.pool)
                .await?;
        }
        
        log::info!("Database optimization completed");
        Ok(())
    }
    
    pub async fn close(&self) -> Result<()> {
        log::info!("Closing database connection pool");
        self.pool.close().await;
        Ok(())
    }
    
    // =============================================
    // Backup/Restore Methods
    // =============================================
    
    pub async fn create_backup(&self, backup_path: &std::path::Path) -> Result<BackupRestoreJob> {
        let job_id = Uuid::new_v4().to_string();
        let start_time = Utc::now();
        
        log::info!("Starting database backup to: {:?}", backup_path);
        
        // Create backup directory if it doesn't exist
        if let Some(parent) = backup_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| ClipBookError::DatabaseError(format!("Failed to create backup directory: {}", e)))?;
        }
        
        // Use SQLite backup API
        let backup_result = self.backup_database(backup_path).await;
        
        let (status, error_message) = match backup_result {
            Ok(_) => {
                log::info!("Database backup completed successfully");
                (JobStatus::Completed, None)
            }
            Err(e) => {
                log::error!("Database backup failed: {}", e);
                (JobStatus::Failed, Some(format!("Backup failed: {}", e)))
            }
        };
        
        // Get backup file size if successful
        let file_size = if status == JobStatus::Completed {
            std::fs::metadata(backup_path)
                .map(|m| Some(m.len()))
                .unwrap_or(None)
        } else {
            None
        };
        
        // Get item count for backup verification
        let items_count = if status == JobStatus::Completed {
            let count_result = sqlx::query("SELECT COUNT(*) as count FROM clipboard_items")
                .fetch_one(&self.pool)
                .await;
            
            match count_result {
                Ok(row) => Some(row.get::<i64, _>("count") as u64),
                Err(_) => None,
            }
        } else {
            None
        };
        
        let end_time = Some(Utc::now());
        
        let job = BackupRestoreJob {
            job_id,
            operation_type: OperationType::Backup,
            status,
            file_path: backup_path.to_path_buf(),
            file_size_bytes: file_size,
            items_count,
            start_time,
            end_time,
            error_message,
            metadata: BackupRestoreMetadata::new(),
        };
        
        // Record backup job in database
        self.record_backup_restore_job(&job).await?;
        
        Ok(job)
    }
    
    async fn backup_database(&self, backup_path: &std::path::Path) -> Result<()> {
        // Use SQLite's backup API via ATTACH DATABASE
        let _backup_filename = backup_path.file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| ClipBookError::DatabaseError("Invalid backup filename".to_string()))?;
        
        // Execute backup using SQLite's backup API
        sqlx::query("ATTACH DATABASE ? AS backup_db")
            .bind(backup_path.to_string_lossy().as_ref())
            .execute(&self.pool)
            .await?;
        
        // Backup the main database
        sqlx::query("BEGIN IMMEDIATE TRANSACTION")
            .execute(&self.pool)
            .await?;
        
        sqlx::query("SELECT sql FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| ClipBookError::DatabaseError(format!("Failed to get table schemas: {}", e)))?;
        
        // Copy each table
        let tables = vec!["clipboard_items", "schema_migrations", "system_preferences", "application_state", 
                           "global_shortcuts", "system_tray_menu", "clipboard_monitoring_sessions",
                           "backup_restore_logs", "permission_status", "database_stats"];
        
        for table in tables {
            let result = sqlx::query(&format!(
                "INSERT INTO backup_db.{} SELECT * FROM main.{}",
                table, table
            ))
            .execute(&self.pool)
            .await;
            
            if let Err(e) = result {
                // Log error but continue with other tables
                log::warn!("Failed to backup table {}: {}", table, e);
            }
        }
        
        sqlx::query("COMMIT")
            .execute(&self.pool)
            .await?;
        
        // Detach backup database
        sqlx::query("DETACH DATABASE backup_db")
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
    
    pub async fn restore_from_backup(&self, backup_path: &std::path::Path) -> Result<BackupRestoreJob> {
        let job_id = Uuid::new_v4().to_string();
        let start_time = Utc::now();
        
        log::info!("Starting database restore from: {:?}", backup_path);
        
        // Verify backup file exists
        if !backup_path.exists() {
            return Err(ClipBookError::DatabaseError("Backup file does not exist".to_string()));
        }
        
        // Create backup of current database before restore
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let pre_restore_backup = std::path::PathBuf::from(format!("backup_before_restore_{}.db", timestamp));
        
        if let Err(e) = self.backup_database(&pre_restore_backup).await {
            log::warn!("Failed to create pre-restore backup: {}", e);
        }
        
        // Perform the restore
        let restore_result = self.restore_database(backup_path).await;
        
        let (status, error_message) = match restore_result {
            Ok(_) => {
                log::info!("Database restore completed successfully");
                (JobStatus::Completed, None)
            }
            Err(e) => {
                log::error!("Database restore failed: {}", e);
                (JobStatus::Failed, Some(format!("Restore failed: {}", e)))
            }
        };
        
        // Get restore file size
        let file_size = std::fs::metadata(backup_path)
            .map(|m| Some(m.len()))
            .unwrap_or(None);
        
        // Get item count after restore
        let items_count = if status == JobStatus::Completed {
            let count_result = sqlx::query("SELECT COUNT(*) as count FROM clipboard_items")
                .fetch_one(&self.pool)
                .await;
            
            match count_result {
                Ok(row) => Some(row.get::<i64, _>("count") as u64),
                Err(_) => None,
            }
        } else {
            None
        };
        
        let end_time = Some(Utc::now());
        
        let job = BackupRestoreJob {
            job_id,
            operation_type: OperationType::Restore,
            status,
            file_path: backup_path.to_path_buf(),
            file_size_bytes: file_size,
            items_count,
            start_time,
            end_time,
            error_message,
            metadata: BackupRestoreMetadata::new(),
        };
        
        // Record restore job in database
        self.record_backup_restore_job(&job).await?;
        
        Ok(job)
    }
    
    async fn restore_database(&self, backup_path: &std::path::Path) -> Result<()> {
        // Clear existing data (except schema_migrations and backup_restore_logs)
        let tables_to_clear = vec![
            "clipboard_items", "system_preferences", "application_state", 
            "global_shortcuts", "system_tray_menu", "clipboard_monitoring_sessions",
            "permission_status", "database_stats"
        ];
        
        sqlx::query("BEGIN IMMEDIATE TRANSACTION")
            .execute(&self.pool)
            .await?;
        
        for table in tables_to_clear {
            if let Err(e) = sqlx::query(&format!("DELETE FROM {}", table))
                .execute(&self.pool)
                .await
            {
                log::warn!("Failed to clear table {}: {}", table, e);
            }
        }
        
        sqlx::query("COMMIT")
            .execute(&self.pool)
            .await?;
        
        // Attach backup database
        sqlx::query("ATTACH DATABASE ? AS restore_db")
            .bind(backup_path.to_string_lossy().as_ref())
            .execute(&self.pool)
            .await?;
        
        // Restore data from backup
        sqlx::query("BEGIN IMMEDIATE TRANSACTION")
            .execute(&self.pool)
            .await?;
        
        let tables = vec!["clipboard_items", "system_preferences", "application_state", 
                           "global_shortcuts", "system_tray_menu", "clipboard_monitoring_sessions",
                           "permission_status", "database_stats"];
        
        for table in tables {
            let result = sqlx::query(&format!(
                "INSERT INTO main.{} SELECT * FROM restore_db.{}",
                table, table
            ))
            .execute(&self.pool)
            .await;
            
            if let Err(e) = result {
                log::warn!("Failed to restore table {}: {}", table, e);
            }
        }
        
        sqlx::query("COMMIT")
            .execute(&self.pool)
            .await?;
        
        // Detach restore database
        sqlx::query("DETACH DATABASE restore_db")
            .execute(&self.pool)
            .await?;
        
        // Re-apply any migrations if needed
        Self::run_migrations(&self.pool).await?;
        
        Ok(())
    }
    
    async fn record_backup_restore_job(&self, job: &BackupRestoreJob) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO backup_restore_logs 
            (job_id, operation_type, status, file_path, file_size_bytes, items_count, start_time, end_time, error_message, metadata)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&job.job_id)
        .bind(match job.operation_type {
            OperationType::Backup => "backup",
            OperationType::Restore => "restore",
        })
        .bind(match job.status {
            JobStatus::Pending => "pending",
            JobStatus::InProgress => "in_progress",
            JobStatus::Completed => "completed",
            JobStatus::Failed => "failed",
            JobStatus::Cancelled => "cancelled",
        })
        .bind(job.file_path.to_string_lossy().as_ref())
        .bind(job.file_size_bytes.unwrap_or(0) as i64)
        .bind(job.items_count.unwrap_or(0) as i64)
        .bind(job.start_time)
        .bind(job.end_time)
        .bind(&job.error_message)
        .bind(serde_json::to_string(&job.metadata)?)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn get_backup_restore_history(&self, limit: Option<usize>) -> Result<Vec<BackupRestoreJob>> {
        let limit = limit.unwrap_or(50);
        
        let rows = sqlx::query(
            "SELECT job_id, operation_type, status, file_path, file_size_bytes, items_count, start_time, end_time, error_message, metadata
             FROM backup_restore_logs 
             ORDER BY start_time DESC 
             LIMIT ?"
        )
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await?;
        
        let mut jobs = Vec::new();
        for row in rows {
            let metadata_str: String = row.get("metadata");
            let metadata = serde_json::from_str(&metadata_str)
                .unwrap_or_else(|_| BackupRestoreMetadata::new());
            
            jobs.push(BackupRestoreJob {
                job_id: row.get("job_id"),
                operation_type: match row.get::<&str, _>("operation_type") {
                    "backup" => OperationType::Backup,
                    "restore" => OperationType::Restore,
                    _ => OperationType::Backup, // Default fallback
                },
                status: match row.get::<&str, _>("status") {
                    "pending" => JobStatus::Pending,
                    "in_progress" => JobStatus::InProgress,
                    "completed" => JobStatus::Completed,
                    "failed" => JobStatus::Failed,
                    "cancelled" => JobStatus::Cancelled,
                    _ => JobStatus::Failed, // Default fallback
                },
                file_path: std::path::PathBuf::from(row.get::<String, _>("file_path")),
                file_size_bytes: row.get("file_size_bytes"),
                items_count: row.get("items_count"),
                start_time: row.get("start_time"),
                end_time: row.get("end_time"),
                error_message: row.get("error_message"),
                metadata,
            });
        }
        
        Ok(jobs)
    }
    
    pub async fn schedule_automatic_backup(&self, backup_directory: &std::path::Path) -> Result<BackupRestoreJob> {
        // Create timestamp-based backup filename
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let backup_filename = format!("clipbook_auto_backup_{}.db", timestamp);
        let backup_path = backup_directory.join(backup_filename);
        
        log::info!("Scheduling automatic backup to: {:?}", backup_path);
        
        // Create the backup
        self.create_backup(&backup_path).await
    }
    
    pub async fn cleanup_old_backups(&self, backup_directory: &std::path::Path, max_backups: usize) -> Result<usize> {
        if !backup_directory.exists() {
            return Ok(0);
        }
        
        // Get all backup files sorted by modification time (oldest first)
        let mut backup_files: Vec<_> = std::fs::read_dir(backup_directory)
            .map_err(|e| ClipBookError::DatabaseError(format!("Failed to read backup directory: {}", e)))?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.path().extension().and_then(|s| s.to_str()) == Some("db")
            })
            .collect();
        
        // Sort by modification time (oldest first)
        backup_files.sort_by_key(|entry| {
            entry.metadata().and_then(|m| m.modified()).unwrap_or(std::time::SystemTime::UNIX_EPOCH)
        });
        
        // Calculate how many files to remove
        let files_to_remove = if backup_files.len() > max_backups {
            backup_files.len() - max_backups
        } else {
            0
        };
        
        let mut removed_count = 0;
        
        // Remove oldest files
        for entry in backup_files.iter().take(files_to_remove) {
            if let Err(e) = std::fs::remove_file(entry.path()) {
                log::warn!("Failed to remove old backup file {:?}: {}", entry.path(), e);
            } else {
                removed_count += 1;
                log::info!("Removed old backup file: {:?}", entry.path());
            }
        }
        
        Ok(removed_count)
    }
}

#[derive(Debug, Clone)]
pub struct ConnectionPoolStats {
    pub max_size: u32,
    pub current_size: u32,
    pub idle_connections: u32,
    pub active_connections: u32,
    pub config: DatabaseConfig,
}

#[derive(Debug, Clone)]
pub struct DatabasePerformanceReport {
    pub pool_stats: ConnectionPoolStats,
    pub database_metrics: DatabaseMetrics,
    pub cache_hit_rate: f64,
    pub error_rate: f64,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct DatabaseHealth {
    pub healthy: bool,
    pub response_time_ms: f64,
    pub pool_size: u32,
    pub last_check: DateTime<Utc>,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DatabaseStats {
    pub total_items: usize,
    pub favorite_count: usize,
    pub unique_content_types: usize,
    pub earliest_item: Option<DateTime<Utc>>,
    pub latest_item: Option<DateTime<Utc>>,
}