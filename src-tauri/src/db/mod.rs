pub mod edges;
pub mod mashes;
pub mod search_cache;
pub mod settings;

use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::Connection;

pub fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

pub fn init_db(app_data_dir: &Path) -> Connection {
    std::fs::create_dir_all(app_data_dir).expect("failed to create app data dir");
    let db_path = app_data_dir.join("moonshine.db");
    let conn = Connection::open(db_path).expect("failed to open database");
    conn.pragma_update(None, "journal_mode", "WAL").ok();
    conn.pragma_update(None, "foreign_keys", "ON").ok();
    create_schema(&conn);
    conn
}

fn create_schema(conn: &Connection) {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS mashes (
            id TEXT PRIMARY KEY,
            type TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'MASH_TUN',
            summary TEXT NOT NULL,
            context TEXT NOT NULL DEFAULT '',
            memo TEXT NOT NULL DEFAULT '',
            embedding BLOB,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        );

        CREATE VIRTUAL TABLE IF NOT EXISTS mashes_fts USING fts5(
            summary, context, memo,
            content='mashes',
            content_rowid='rowid',
            tokenize='trigram'
        );

        CREATE TRIGGER IF NOT EXISTS mashes_ai AFTER INSERT ON mashes BEGIN
            INSERT INTO mashes_fts(rowid, summary, context, memo)
            VALUES (new.rowid, new.summary, new.context, new.memo);
        END;

        CREATE TRIGGER IF NOT EXISTS mashes_ad AFTER DELETE ON mashes BEGIN
            INSERT INTO mashes_fts(mashes_fts, rowid, summary, context, memo)
            VALUES ('delete', old.rowid, old.summary, old.context, old.memo);
        END;

        CREATE TRIGGER IF NOT EXISTS mashes_au AFTER UPDATE ON mashes BEGIN
            INSERT INTO mashes_fts(mashes_fts, rowid, summary, context, memo)
            VALUES ('delete', old.rowid, old.summary, old.context, old.memo);
            INSERT INTO mashes_fts(rowid, summary, context, memo)
            VALUES (new.rowid, new.summary, new.context, new.memo);
        END;

        CREATE TABLE IF NOT EXISTS edges (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            source_id TEXT NOT NULL REFERENCES mashes(id) ON DELETE CASCADE,
            target_id TEXT NOT NULL REFERENCES mashes(id) ON DELETE CASCADE,
            relation_type TEXT NOT NULL,
            source TEXT NOT NULL DEFAULT 'ai',
            confidence REAL NOT NULL DEFAULT 0.0,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            UNIQUE(source_id, target_id)
        );

        CREATE INDEX IF NOT EXISTS idx_edges_source ON edges(source_id);
        CREATE INDEX IF NOT EXISTS idx_edges_target ON edges(target_id);

        CREATE TABLE IF NOT EXISTS search_cache (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            query TEXT NOT NULL,
            provider TEXT NOT NULL,
            embedding BLOB NOT NULL,
            result_ids TEXT NOT NULL,
            created_at INTEGER NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_search_cache_query ON search_cache(query, provider);
        ",
    )
    .expect("failed to create schema");
}
