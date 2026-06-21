use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct HistoryRecord {
    pub id: i64,
    pub source_lang: String,
    pub target_lang: String,
    pub source_text: String,
    pub target_text: String,
    pub timestamp: i64,
    pub favorite: bool,
}

fn db_path() -> PathBuf {
    let name = if cfg!(debug_assertions) {
        "history-dev.db"
    } else {
        "history.db"
    };
    crate::config::default_db_dir_inner().join(name)
}

fn open() -> Result<Connection, String> {
    let p = db_path();
    if let Some(parent) = p.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("create db dir: {}", e))?;
    }
    Connection::open(&p).map_err(|e| format!("open db: {}", e))
}

fn init() -> Result<(), String> {
    let conn = open()?;
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            source_lang TEXT NOT NULL,
            target_lang TEXT NOT NULL,
            source_text TEXT NOT NULL,
            target_text TEXT NOT NULL,
            timestamp INTEGER NOT NULL,
            favorite INTEGER NOT NULL DEFAULT 0
        )",
    )
    .map_err(|e| format!("init history db: {}", e))
}

// ---- Tauri commands ----

#[tauri::command]
pub fn save_history(
    source_lang: String,
    target_lang: String,
    source_text: String,
    target_text: String,
    timestamp: i64,
) -> Result<(), String> {
    init()?;
    let conn = open()?;
    conn.execute(
        "INSERT INTO history (source_lang, target_lang, source_text, target_text, timestamp, favorite)
         VALUES (?1, ?2, ?3, ?4, ?5, 0)",
        params![source_lang, target_lang, source_text, target_text, timestamp],
    )
    .map_err(|e| format!("save history: {}", e))?;
    Ok(())
}

#[tauri::command]
pub fn load_history() -> Result<String, String> {
    init()?;
    let conn = open()?;
    let mut stmt = conn
        .prepare("SELECT id, source_lang, target_lang, source_text, target_text, timestamp, favorite FROM history ORDER BY timestamp DESC")
        .map_err(|e| format!("query history: {}", e))?;
    let rows = stmt
        .query_map([], |row| {
            Ok(HistoryRecord {
                id: row.get(0)?,
                source_lang: row.get(1)?,
                target_lang: row.get(2)?,
                source_text: row.get(3)?,
                target_text: row.get(4)?,
                timestamp: row.get(5)?,
                favorite: row.get::<_, i64>(6)? != 0,
            })
        })
        .map_err(|e| format!("query history: {}", e))?;
    let mut records: Vec<HistoryRecord> = Vec::new();
    for row in rows {
        records.push(row.map_err(|e| format!("read history row: {}", e))?);
    }
    serde_json::to_string(&records).map_err(|e| format!("serialize history: {}", e))
}

#[tauri::command]
pub fn delete_history(id: i64) -> Result<(), String> {
    init()?;
    let conn = open()?;
    conn.execute("DELETE FROM history WHERE id = ?1", params![id])
        .map_err(|e| format!("delete history: {}", e))?;
    Ok(())
}

#[tauri::command]
pub fn delete_history_batch(ids: Vec<i64>) -> Result<(), String> {
    init()?;
    let conn = open()?;
    for id in ids {
        conn.execute("DELETE FROM history WHERE id = ?1", params![id])
            .map_err(|e| format!("delete history batch: {}", e))?;
    }
    Ok(())
}

#[tauri::command]
pub fn delete_history_all() -> Result<(), String> {
    init()?;
    let conn = open()?;
    conn.execute("DELETE FROM history", [])
        .map_err(|e| format!("delete all history: {}", e))?;
    Ok(())
}

#[tauri::command]
pub fn toggle_history_favorite(id: i64) -> Result<(), String> {
    init()?;
    let conn = open()?;
    conn.execute(
        "UPDATE history SET favorite = CASE WHEN favorite = 0 THEN 1 ELSE 0 END WHERE id = ?1",
        params![id],
    )
    .map_err(|e| format!("toggle favorite: {}", e))?;
    Ok(())
}
