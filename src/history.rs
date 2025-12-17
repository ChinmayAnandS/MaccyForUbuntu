use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Result};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::config::Config;

#[derive(Debug, Clone)]
pub struct ClipboardItem {
    pub id: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}

impl ClipboardItem {
    pub fn new(content: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            content,
            timestamp: Utc::now(),
        }
    }
}

pub struct HistoryManager {
    items: VecDeque<ClipboardItem>,
    config: Arc<Mutex<Config>>,
    db: Connection,
}

impl HistoryManager {
    pub fn new(config: Arc<Mutex<Config>>) -> Self {
        let db_path = dirs::data_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("~/.local/share"))
            .join("clipstack")
            .join("history.db");

        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }

        let db = Connection::open(&db_path).expect("Failed to open database");
        Self::init_db(&db);

        let mut manager = Self {
            items: VecDeque::new(),
            config,
            db,
        };

        manager.load_from_db();
        manager
    }

    fn init_db(db: &Connection) {
        db.execute(
            "CREATE TABLE IF NOT EXISTS clipboard_items (
                id TEXT PRIMARY KEY,
                content TEXT NOT NULL,
                timestamp TEXT NOT NULL
            )",
            [],
        )
        .expect("Failed to create table");
    }

    fn load_from_db(&mut self) {
        let config = self.config.try_lock().unwrap();
        let limit = config.max_items;

        let mut stmt = self.db.prepare(
            "SELECT id, content, timestamp FROM clipboard_items
             ORDER BY timestamp DESC LIMIT ?"
        ).unwrap();

        let items = stmt.query_map([limit], |row| {
            let timestamp_str: String = row.get(2)?;
            let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)
                .unwrap()
                .with_timezone(&Utc);

            Ok(ClipboardItem {
                id: row.get(0)?,
                content: row.get(1)?,
                timestamp,
            })
        }).unwrap();

        self.items.clear();
        for item in items.filter_map(Result::ok) {
            self.items.push_front(item);
        }
    }

    pub async fn add_item(&mut self, content: String) {
        let config = self.config.lock().await;

        // Skip if disabled
        if !config.enabled {
            return;
        }

        // Skip duplicates
        if let Some(last) = self.items.front() {
            if last.content == content {
                return;
            }
        }

        // Skip empty content
        if content.trim().is_empty() {
            return;
        }

        let item = ClipboardItem::new(content);

        // Save to database
        self.db.execute(
            "INSERT INTO clipboard_items (id, content, timestamp) VALUES (?, ?, ?)",
            params![
                &item.id,
                &item.content,
                item.timestamp.to_rfc3339()
            ],
        ).unwrap();

        // Add to memory
        self.items.push_front(item);

        // Enforce limit
        while self.items.len() > config.max_items {
            if let Some(old_item) = self.items.pop_back() {
                self.db.execute(
                    "DELETE FROM clipboard_items WHERE id = ?",
                    [&old_item.id],
                ).unwrap();
            }
        }
    }

    pub fn get_items(&self) -> &VecDeque<ClipboardItem> {
        &self.items
    }

    pub fn search(&self, query: &str) -> Vec<&ClipboardItem> {
        if query.is_empty() {
            return self.items.iter().collect();
        }

        self.items
            .iter()
            .filter(|item| item.content.to_lowercase().contains(&query.to_lowercase()))
            .collect()
    }

    pub async fn clear_all(&mut self) {
        self.db.execute("DELETE FROM clipboard_items", []).unwrap();
        self.items.clear();
    }
}
