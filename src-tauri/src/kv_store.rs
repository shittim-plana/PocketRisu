use rusqlite::{params, Connection};
use std::path::Path;
use std::sync::Mutex;

pub struct KvStore {
    conn: Mutex<Connection>,
}

unsafe impl Send for KvStore {}
unsafe impl Sync for KvStore {}

impl KvStore {
    pub fn open(data_dir: &Path) -> Result<Self, String> {
        std::fs::create_dir_all(data_dir).map_err(|e| e.to_string())?;
        let db_path = data_dir.join("pocketrisu.db");
        let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;

        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA cache_size = -64000;
             PRAGMA temp_store = MEMORY;
             PRAGMA busy_timeout = 5000;
             PRAGMA mmap_size = 268435456;
             PRAGMA journal_size_limit = 268435456;"
        ).map_err(|e| e.to_string())?;

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS kv (
                key TEXT PRIMARY KEY,
                value BLOB NOT NULL,
                updated_at INTEGER NOT NULL DEFAULT (strftime('%s','now') * 1000)
            );"
        ).map_err(|e| e.to_string())?;

        Ok(Self { conn: Mutex::new(conn) })
    }

    pub fn get(&self, key: &str) -> Result<Option<Vec<u8>>, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn.prepare("SELECT value FROM kv WHERE key = ?1")
            .map_err(|e| e.to_string())?;
        let result = stmt.query_row(params![key], |row| row.get(0))
            .optional()
            .map_err(|e| e.to_string())?;
        Ok(result)
    }

    pub fn set(&self, key: &str, value: &[u8]) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        conn.execute(
            "INSERT OR REPLACE INTO kv (key, value, updated_at) VALUES (?1, ?2, strftime('%s','now') * 1000)",
            params![key, value],
        ).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn delete(&self, key: &str) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        conn.execute("DELETE FROM kv WHERE key = ?1", params![key])
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn list(&self, prefix: Option<&str>) -> Result<Vec<String>, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        match prefix {
            Some(p) if !p.is_empty() => {
                let pattern = format!("{}%", p.replace('%', "\\%").replace('_', "\\_"));
                let mut stmt = conn.prepare("SELECT key FROM kv WHERE key LIKE ?1 ESCAPE '\\'")
                    .map_err(|e| e.to_string())?;
                let rows = stmt.query_map(params![pattern], |row| row.get::<_, String>(0))
                    .map_err(|e| e.to_string())?;
                let keys: Vec<String> = rows.filter_map(|r| r.ok()).collect();
                Ok(keys)
            }
            _ => {
                let mut stmt = conn.prepare("SELECT key FROM kv")
                    .map_err(|e| e.to_string())?;
                let rows = stmt.query_map([], |row| row.get::<_, String>(0))
                    .map_err(|e| e.to_string())?;
                let keys: Vec<String> = rows.filter_map(|r| r.ok()).collect();
                Ok(keys)
            }
        }
    }

    pub fn size(&self, key: &str) -> Result<Option<i64>, String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn.prepare("SELECT LENGTH(value) FROM kv WHERE key = ?1")
            .map_err(|e| e.to_string())?;
        let result = stmt.query_row(params![key], |row| row.get(0))
            .optional()
            .map_err(|e| e.to_string())?;
        Ok(result)
    }

    pub fn checkpoint_wal(&self) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| e.to_string())?;
        conn.execute_batch("PRAGMA wal_checkpoint(TRUNCATE);")
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

use rusqlite::OptionalExtension;
