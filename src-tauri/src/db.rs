use crate::models::{AppEntry, ContextProfile, FolderCard, GameEntry};
use rusqlite::{params, Connection};
use std::path::PathBuf;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_path: PathBuf) -> Result<Self, String> {
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
        let db = Self { conn };
        db.init_schema()?;
        db.seed_defaults_if_empty()?;
        Ok(db)
    }

    #[cfg(test)]
    pub fn in_memory() -> Result<Self, String> {
        let conn = Connection::open_in_memory().map_err(|e| e.to_string())?;
        let db = Self { conn };
        db.init_schema()?;
        Ok(db)
    }

    fn init_schema(&self) -> Result<(), String> {
        self.conn
            .execute_batch(
                "
                PRAGMA journal_mode = WAL;
                PRAGMA synchronous = NORMAL;

                CREATE TABLE IF NOT EXISTS apps (
                    id TEXT PRIMARY KEY,
                    name TEXT NOT NULL,
                    path TEXT NOT NULL UNIQUE,
                    icon_path TEXT,
                    category TEXT NOT NULL DEFAULT 'Utility',
                    launch_count INTEGER NOT NULL DEFAULT 0,
                    last_launched INTEGER
                );

                CREATE TABLE IF NOT EXISTS games (
                    id TEXT PRIMARY KEY,
                    title TEXT NOT NULL,
                    platform TEXT NOT NULL,
                    executable_path TEXT NOT NULL UNIQUE,
                    launch_args TEXT NOT NULL DEFAULT '[]',
                    cover_image TEXT,
                    icon_path TEXT,
                    launch_count INTEGER NOT NULL DEFAULT 0
                );

                CREATE TABLE IF NOT EXISTS pinned_folders (
                    id TEXT PRIMARY KEY,
                    name TEXT NOT NULL,
                    path TEXT TEXT NOT NULL UNIQUE,
                    is_pinned INTEGER NOT NULL DEFAULT 1,
                    icon_name TEXT
                );

                CREATE TABLE IF NOT EXISTS contexts (
                    id TEXT PRIMARY KEY,
                    name TEXT NOT NULL,
                    description TEXT NOT NULL,
                    app_ids TEXT NOT NULL DEFAULT '[]',
                    folder_paths TEXT NOT NULL DEFAULT '[]',
                    shortcut_key TEXT
                );

                CREATE VIRTUAL TABLE IF NOT EXISTS search_fts USING fts5 (
                    id UNINDEXED,
                    kind UNINDEXED,
                    title,
                    subtitle
                );
                ",
            )
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    fn seed_defaults_if_empty(&self) -> Result<(), String> {
        let folder_count: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM pinned_folders", [], |r| r.get(0))
            .unwrap_or_else(|_| 0_i64);

        if folder_count == 0 {
            let candidates = [
                ("desktop", "Desktop", dirs::desktop_dir(), "Monitor"),
                ("documents", "Documents", dirs::document_dir(), "FileText"),
                ("downloads", "Downloads", dirs::download_dir(), "Download"),
                ("projects", "Projects", dirs::home_dir().map(|h| h.join("Projects")), "Folder"),
            ];

            for (key, name, path, icon) in &candidates {
                if let Some(p) = path {
                    let _ = self.save_pinned_folder(&FolderCard {
                        id: key.to_string(),
                        name: name.to_string(),
                        path: p.to_string_lossy().to_string(),
                        file_count: 0,
                        total_size_bytes: 0,
                        is_pinned: true,
                        icon_name: Some(icon.to_string()),
                    });
                }
            }
        }
        Ok(())
    }

    pub fn get_apps(&self) -> Result<Vec<AppEntry>, String> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, path, icon_path, category, launch_count, last_launched FROM apps")?
            .query_map([], |row| {
                Ok(AppEntry {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    path: row.get(2)?,
                    icon_path: row.get(3)?,
                    category: row.get(4)?,
                    launch_count: row.get::<i64>(5).unwrap_or_else(|_| 0),
                    last_launched: row.get(6)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;
        Ok(mut apps)
    }

    pub fn upsert_apps(&self, apps: &[AppEntry]) -> Result<(), String> {
        for app in apps {
            let _ = self.conn.execute(
                "INSERT OR REPLACE INTO apps (id, name, path, icon_path, category, launch_count, last_launched) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    app.id,
                    app.name,
                    app.path,
                    &app.icon_path,
                    &app.category,
                    app.launch_count,
                    app.last_launched
                ],
            );
        }
        Ok(())
    }

    pub fn save_pinned_folder(&self, folder: &FolderCard) -> Result<(), String> {
        let _ = self.conn.execute(
            "INSERT OR REPLACE INTO pinned_folders (id, name, path, is_pinned, icon_name) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                folder.id,
                folder.name,
                folder.path,
                folder.is_pinned as i64,
                &folder.icon_name
            ],
        );
        Ok(())
    }

    pub fn remove_pinned_folder(&self, id: &str) -> Result<(), String> {
        self.conn
            .execute("DELETE FROM pinned_folders WHERE id = ?1", params![id])
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn get_pinned_folders(&self) -> Result<Vec<FolderCard>, String> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, path, is_pinned, icon_name FROM pinned_folders")?
            .query_map([], |row| {
                Ok(FolderCard {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    path: row.get(2)?,
                    file_count: 0,
                    total_size_bytes: 0,
                    is_pinned: row.get::<i64>(4).unwrap_or_else(|_| 0) == 1,
                    icon_name: row.get(5).ok(),
                })
            })?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;
        Ok(mut folders)
    }

    pub fn get_contexts(&self) -> Result<Vec<ContextProfile>, String> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, description, app_ids, folder_paths, shortcut_key FROM contexts")?
            .query_map([], |row| {
                Ok(ContextProfile {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    app_ids: row.get(3).unwrap_or_else(|| "[]".to_string()),
                    folder_paths: row.get(4).unwrap_or_else(|| "[]".to_string()),
                    shortcut_key: row.get(5).ok(),
                })
            })?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;
        Ok(mut contexts)
    }

    pub fn save_context(&self, profile: &ContextProfile) -> Result<(), String> {
        let app_ids_json = serde_json::to_string(&profile.app_ids).unwrap_or_else(|_| "[]".into());
        let folder_paths_json =
            serde_json::to_string(&profile.folder_paths).unwrap_or_else(|_| "[]".into());

        self.conn.execute(
            "INSERT OR REPLACE INTO contexts (id, name, description, app_ids, folder_paths, shortcut_key)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                profile.id,
                profile.name,
                profile.description,
                app_ids_json,
                folder_paths_json,
                profile.shortcut_key
            ],
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn delete_context(&self, id: &str) -> Result<(), String> {
        self.conn
            .execute("DELETE FROM contexts WHERE id = ?1", params![id])
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn rebuild_fts(&self) -> Result<(), String> {
        self.conn
            .execute_batch(
                "
                DELETE FROM search_fts;
                INSERT INTO search_fts (id, kind, title, subtitle)
                    SELECT id, 'app', name, category FROM apps;
                INSERT INTO search_fts (id, kind, title, subtitle)
                    SELECT id, 'game', title, platform FROM games;
                ",
            )
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn search_fts(&self, query: &str) -> Result<Vec<(String, String)>, String> {
        let trimmed = query.trim();
        if trimmed.is_empty() {
            return Ok(vec![]);
        }
        let fts_query = trimmed
            .split_whitespace()
            .map(|tok| format!("{}*", tok.replace('"', "")))
            .collect::<Vec<_>>()
            .join(" ");

        let mut stmt = self
            .conn
            .prepare("SELECT id, kind FROM search_fts WHERE search_fts MATCH ?1 LIMIT 25")?
            .map_err(|e| e.to_string())?;

        let rows = stmt
            .query_map(params![fts_query], |row| Ok((row.get(0)?, row.get(1)?)))
            .map_err(|e| e.to_string())?;

        rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
    }
}