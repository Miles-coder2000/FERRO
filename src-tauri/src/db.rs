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
                    path TEXT NOT NULL UNIQUE,
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

                CREATE VIRTUAL TABLE IF NOT EXISTS search_fts USING fts5(
                    id UNINDEXED,
                    kind UNINDEXED,
                    title,
                    subtitle
                );
                ",
            )
            .map_err(|e| e.to_string())
    }

    fn seed_defaults_if_empty(&self) -> Result<(), String> {
        let folder_count: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM pinned_folders", [], |r| r.get(0))
            .unwrap_or(0);

        if folder_count == 0 {
            let candidates = [
                ("desktop", "Desktop", dirs::desktop_dir(), "Monitor"),
                ("documents", "Documents", dirs::document_dir(), "FileText"),
                ("downloads", "Downloads", dirs::download_dir(), "Download"),
                ("projects", "Projects", dirs::home_dir().map(|h| h.join("Projects")), "Code2"),
            ];

            for (id, name, path_opt, icon) in candidates {
                if let Some(p) = path_opt {
                    if p.exists() {
                        let _ = self.conn.execute(
                            "INSERT OR IGNORE INTO pinned_folders (id, name, path, is_pinned, icon_name) VALUES (?1, ?2, ?3, 1, ?4)",
                            params![id, name, p.to_string_lossy().to_string(), icon],
                        );
                    }
                }
            }
        }

        let ctx_count: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM contexts", [], |r| r.get(0))
            .unwrap_or(0);

        if ctx_count == 0 {
            let defaults = vec![
                ContextProfile {
                    id: "ctx-dev".to_string(),
                    name: "FORGE DEV".to_string(),
                    description: "Launch coding IDE, terminal, and workspace directories.".to_string(),
                    app_ids: vec![],
                    folder_paths: dirs::document_dir()
                        .map(|d| vec![d.to_string_lossy().to_string()])
                        .unwrap_or_default(),
                    shortcut_key: Some("Ctrl+1".to_string()),
                },
                ContextProfile {
                    id: "ctx-gaming".to_string(),
                    name: "ARMORY OPS".to_string(),
                    description: "Prepares gaming environment and Discord comms.".to_string(),
                    app_ids: vec![],
                    folder_paths: vec![],
                    shortcut_key: Some("Ctrl+2".to_string()),
                },
                ContextProfile {
                    id: "ctx-clean".to_string(),
                    name: "SYS TRIAGE".to_string(),
                    description: "Quick access to Downloads & system diagnostic tools.".to_string(),
                    app_ids: vec![],
                    folder_paths: dirs::download_dir()
                        .map(|d| vec![d.to_string_lossy().to_string()])
                        .unwrap_or_default(),
                    shortcut_key: Some("Ctrl+3".to_string()),
                },
            ];

            for ctx in defaults {
                let _ = self.save_context(&ctx);
            }
        }
        Ok(())
    }

    pub fn upsert_apps(&mut self, apps: &[AppEntry]) -> Result<(), String> {
        let tx = self.conn.transaction().map_err(|e| e.to_string())?;
        {
            let mut stmt = tx
                .prepare(
                    "INSERT INTO apps (id, name, path, icon_path, category, launch_count, last_launched)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
                     ON CONFLICT(path) DO UPDATE SET
                        name = excluded.name,
                        icon_path = COALESCE(excluded.icon_path, apps.icon_path),
                        category = excluded.category",
                )
                .map_err(|e| e.to_string())?;

            for app in apps {
                stmt.execute(params![
                    app.id,
                    app.name,
                    app.path,
                    app.icon_path,
                    app.category,
                    app.launch_count,
                    app.last_launched,
                ])
                .map_err(|e| e.to_string())?;
            }
        }
        tx.commit().map_err(|e| e.to_string())?;
        self.rebuild_fts()
    }

    pub fn get_apps(&self) -> Result<Vec<AppEntry>, String> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, name, path, icon_path, category, launch_count, last_launched
                 FROM apps ORDER BY launch_count DESC, name ASC",
            )
            .map_err(|e| e.to_string())?;

        let rows = stmt
            .query_map([], |row| {
                Ok(AppEntry {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    path: row.get(2)?,
                    icon_path: row.get(3)?,
                    category: row.get(4)?,
                    launch_count: row.get(5)?,
                    last_launched: row.get(6)?,
                })
            })
            .map_err(|e| e.to_string())?;

        rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
    }

    pub fn upsert_games(&mut self, games: &[GameEntry]) -> Result<(), String> {
        let tx = self.conn.transaction().map_err(|e| e.to_string())?;
        {
            let mut stmt = tx
                .prepare(
                    "INSERT INTO games (id, title, platform, executable_path, launch_args, cover_image, icon_path, launch_count)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
                     ON CONFLICT(executable_path) DO UPDATE SET
                        title = excluded.title,
                        platform = excluded.platform,
                        launch_args = excluded.launch_args,
                        cover_image = COALESCE(excluded.cover_image, games.cover_image),
                        icon_path = COALESCE(excluded.icon_path, games.icon_path)",
                )
                .map_err(|e| e.to_string())?;

            for game in games {
                let args_json =
                    serde_json::to_string(&game.launch_args).unwrap_or_else(|_| "[]".into());
                stmt.execute(params![
                    game.id,
                    game.title,
                    game.platform,
                    game.executable_path,
                    args_json,
                    game.cover_image,
                    game.icon_path,
                    game.launch_count,
                ])
                .map_err(|e| e.to_string())?;
            }
        }
        tx.commit().map_err(|e| e.to_string())?;
        self.rebuild_fts()
    }

    pub fn get_games(&self) -> Result<Vec<GameEntry>, String> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, title, platform, executable_path, launch_args, cover_image, icon_path, launch_count
                 FROM games ORDER BY launch_count DESC, title ASC",
            )
            .map_err(|e| e.to_string())?;

        let rows = stmt
            .query_map([], |row| {
                let args_str: String = row.get(4)?;
                let launch_args: Vec<String> =
                    serde_json::from_str(&args_str).unwrap_or_default();
                Ok(GameEntry {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    platform: row.get(2)?,
                    executable_path: row.get(3)?,
                    launch_args,
                    cover_image: row.get(5)?,
                    icon_path: row.get(6)?,
                    launch_count: row.get(7)?,
                })
            })
            .map_err(|e| e.to_string())?;

        rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
    }

    pub fn increment_launch_count(&self, target_path: &str) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);

        let _ = self.conn.execute(
            "UPDATE apps SET launch_count = launch_count + 1, last_launched = ?1 WHERE path = ?2",
            params![now, target_path],
        );
        let _ = self.conn.execute(
            "UPDATE games SET launch_count = launch_count + 1 WHERE executable_path = ?1",
            params![target_path],
        );
    }

    pub fn get_pinned_folders(&self) -> Result<Vec<FolderCard>, String> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, path, is_pinned, icon_name FROM pinned_folders ORDER BY name ASC")
            .map_err(|e| e.to_string())?;

        let rows = stmt
            .query_map([], |row| {
                let pinned_int: i64 = row.get(3)?;
                Ok(FolderCard {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    path: row.get(2)?,
                    file_count: 0,
                    total_size_bytes: 0,
                    is_pinned: pinned_int != 0,
                    icon_name: row.get(4)?,
                })
            })
            .map_err(|e| e.to_string())?;

        rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
    }

    pub fn save_pinned_folder(&self, folder: &FolderCard) -> Result<(), String> {
        self.conn
            .execute(
                "INSERT INTO pinned_folders (id, name, path, is_pinned, icon_name)
                 VALUES (?1, ?2, ?3, ?4, ?5)
                 ON CONFLICT(path) DO UPDATE SET name = excluded.name, is_pinned = excluded.is_pinned, icon_name = excluded.icon_name",
                params![
                    folder.id,
                    folder.name,
                    folder.path,
                    if folder.is_pinned { 1 } else { 0 },
                    folder.icon_name
                ],
            )
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn remove_pinned_folder(&self, id: &str) -> Result<(), String> {
        self.conn
            .execute("DELETE FROM pinned_folders WHERE id = ?1", params![id])
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn get_contexts(&self) -> Result<Vec<ContextProfile>, String> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, description, app_ids, folder_paths, shortcut_key FROM contexts ORDER BY name ASC")
            .map_err(|e| e.to_string())?;

        let rows = stmt
            .query_map([], |row| {
                let app_ids_str: String = row.get(3)?;
                let folder_paths_str: String = row.get(4)?;
                Ok(ContextProfile {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    app_ids: serde_json::from_str(&app_ids_str).unwrap_or_default(),
                    folder_paths: serde_json::from_str(&folder_paths_str).unwrap_or_default(),
                    shortcut_key: row.get(5)?,
                })
            })
            .map_err(|e| e.to_string())?;

        rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
    }

    pub fn save_context(&self, ctx: &ContextProfile) -> Result<(), String> {
        let app_ids_json = serde_json::to_string(&ctx.app_ids).unwrap_or_else(|_| "[]".into());
        let folder_paths_json =
            serde_json::to_string(&ctx.folder_paths).unwrap_or_else(|_| "[]".into());

        self.conn
            .execute(
                "INSERT INTO contexts (id, name, description, app_ids, folder_paths, shortcut_key)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                 ON CONFLICT(id) DO UPDATE SET
                    name = excluded.name,
                    description = excluded.description,
                    app_ids = excluded.app_ids,
                    folder_paths = excluded.folder_paths,
                    shortcut_key = excluded.shortcut_key",
                params![
                    ctx.id,
                    ctx.name,
                    ctx.description,
                    app_ids_json,
                    folder_paths_json,
                    ctx.shortcut_key
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

    fn rebuild_fts(&self) -> Result<(), String> {
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
            .map_err(|e| e.to_string())
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
            .prepare("SELECT id, kind FROM search_fts WHERE search_fts MATCH ?1 LIMIT 25")
            .map_err(|e| e.to_string())?;

        let rows = stmt
            .query_map(params![fts_query], |row| Ok((row.get(0)?, row.get(1)?)))
            .map_err(|e| e.to_string())?;

        rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_apps_and_fts() {
        let mut db = Database::in_memory().expect("failed to init memory db");
        let app = AppEntry {
            id: "app-1".into(),
            name: "Visual Studio Code".into(),
            path: "C:\\Program Files\\VSCode\\Code.exe".into(),
            icon_path: None,
            category: "Developer".into(),
            launch_count: 2,
            last_launched: None,
        };
        db.upsert_apps(&[app]).expect("upsert failed");
        let apps = db.get_apps().expect("get_apps failed");
        assert_eq!(apps.len(), 1);
        assert_eq!(apps[0].name, "Visual Studio Code");

        let hits = db.search_fts("visual").expect("fts failed");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].0, "app-1");
        assert_eq!(hits[0].1, "app");
    }
}

}
