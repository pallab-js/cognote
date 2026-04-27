// ANCHOR: DB_MODULE_READY
use rusqlite::{Connection, Result, params};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ── Models ────────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Notebook {
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: Option<String>,
    pub notebook_id: Option<String>,
    pub is_pinned: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tag {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoteLink {
    pub source_note_id: String,
    pub target_note_id: String,
    pub context: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileInfo {
    pub id: String,
    pub name: String,
    pub path: String,
    pub mime_type: Option<String>,
    pub size: Option<i64>,
    pub notebook_id: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GraphData {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GraphNode {
    pub id: String,
    pub label: String,
    pub notebook_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GraphEdge {
    pub source: String,
    pub target: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub snippet: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DailyStats {
    pub today_count: i64,
    pub streak: i64,
    pub total_notes: i64,
    pub recent_days: Vec<DayCount>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DayCount {
    pub date: String,
    pub count: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub theme: String,
    pub vault_path: Option<String>,
}

// ── Row helpers ───────────────────────────────────────────────────────────────

fn note_from_row(row: &rusqlite::Row) -> rusqlite::Result<Note> {
    Ok(Note {
        id: row.get(0)?,
        title: row.get(1)?,
        content: row.get(2)?,
        notebook_id: row.get(3)?,
        is_pinned: row.get::<_, i64>(4)? != 0,
        created_at: row.get(5)?,
        updated_at: row.get(6)?,
    })
}

fn file_from_row(row: &rusqlite::Row) -> rusqlite::Result<FileInfo> {
    Ok(FileInfo {
        id: row.get(0)?,
        name: row.get(1)?,
        path: row.get(2)?,
        mime_type: row.get(3)?,
        size: row.get(4)?,
        notebook_id: row.get(5)?,
        created_at: row.get(6)?,
    })
}

// ── Database ──────────────────────────────────────────────────────────────────

pub struct Database {
    pub conn: Connection,
}

impl Database {
    pub fn open(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch("PRAGMA foreign_keys = ON; PRAGMA journal_mode = WAL;")?;
        let db = Database { conn };
        db.migrate()?;
        Ok(db)
    }

    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;
        let db = Database { conn };
        db.migrate()?;
        Ok(db)
    }

    fn migrate(&self) -> Result<()> {
        self.conn.execute_batch("
            CREATE TABLE IF NOT EXISTS notebooks (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                parent_id TEXT REFERENCES notebooks(id) ON DELETE CASCADE,
                created_at TEXT DEFAULT (datetime('now')),
                updated_at TEXT DEFAULT (datetime('now'))
            );

            CREATE TABLE IF NOT EXISTS notes (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                content TEXT,
                notebook_id TEXT REFERENCES notebooks(id) ON DELETE SET NULL,
                is_pinned INTEGER DEFAULT 0,
                created_at TEXT DEFAULT (datetime('now')),
                updated_at TEXT DEFAULT (datetime('now'))
            );

            CREATE TABLE IF NOT EXISTS tags (
                id TEXT PRIMARY KEY,
                name TEXT UNIQUE NOT NULL
            );

            CREATE TABLE IF NOT EXISTS note_tags (
                note_id TEXT REFERENCES notes(id) ON DELETE CASCADE,
                tag_id TEXT REFERENCES tags(id) ON DELETE CASCADE,
                PRIMARY KEY (note_id, tag_id)
            );

            CREATE TABLE IF NOT EXISTS note_links (
                source_note_id TEXT REFERENCES notes(id) ON DELETE CASCADE,
                target_note_id TEXT REFERENCES notes(id) ON DELETE CASCADE,
                context TEXT,
                PRIMARY KEY (source_note_id, target_note_id)
            );

            CREATE TABLE IF NOT EXISTS files (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                path TEXT NOT NULL UNIQUE,
                mime_type TEXT,
                size INTEGER,
                notebook_id TEXT REFERENCES notebooks(id) ON DELETE SET NULL,
                created_at TEXT DEFAULT (datetime('now'))
            );

            CREATE TABLE IF NOT EXISTS app_config (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );

            CREATE VIRTUAL TABLE IF NOT EXISTS notes_fts USING fts5(
                title, content, content='notes', content_rowid='rowid'
            );

            CREATE TRIGGER IF NOT EXISTS notes_fts_insert AFTER INSERT ON notes BEGIN
                INSERT INTO notes_fts(rowid, title, content) VALUES (new.rowid, new.title, COALESCE(new.content, ''));
            END;

            CREATE TRIGGER IF NOT EXISTS notes_fts_update AFTER UPDATE ON notes BEGIN
                INSERT INTO notes_fts(notes_fts, rowid, title, content) VALUES ('delete', old.rowid, old.title, COALESCE(old.content, ''));
                INSERT INTO notes_fts(rowid, title, content) VALUES (new.rowid, new.title, COALESCE(new.content, ''));
            END;

            CREATE TRIGGER IF NOT EXISTS notes_fts_delete AFTER DELETE ON notes BEGIN
                INSERT INTO notes_fts(notes_fts, rowid, title, content) VALUES ('delete', old.rowid, old.title, COALESCE(old.content, ''));
            END;

            CREATE INDEX IF NOT EXISTS idx_notes_notebook ON notes(notebook_id);
            CREATE INDEX IF NOT EXISTS idx_notes_updated ON notes(updated_at DESC);
            CREATE INDEX IF NOT EXISTS idx_note_tags_tag ON note_tags(tag_id);
            CREATE INDEX IF NOT EXISTS idx_note_tags_note ON note_tags(note_id);
            CREATE INDEX IF NOT EXISTS idx_note_links_source ON note_links(source_note_id);
            CREATE INDEX IF NOT EXISTS idx_note_links_target ON note_links(target_note_id);
            CREATE INDEX IF NOT EXISTS idx_files_notebook ON files(notebook_id);
        ")
    }

    // ── Notebooks ─────────────────────────────────────────────────────────────

    pub fn create_notebook(&self, name: &str, parent_id: Option<&str>) -> Result<Notebook> {
        let id = Uuid::new_v4().to_string();
        self.conn.execute(
            "INSERT INTO notebooks (id, name, parent_id) VALUES (?1, ?2, ?3)",
            params![id, name, parent_id],
        )?;
        self.get_notebook(&id)
    }

    pub fn get_notebook(&self, id: &str) -> Result<Notebook> {
        self.conn.query_row(
            "SELECT id, name, parent_id, created_at, updated_at FROM notebooks WHERE id = ?1",
            params![id],
            |row| Ok(Notebook {
                id: row.get(0)?,
                name: row.get(1)?,
                parent_id: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            }),
        )
    }

    pub fn rename_notebook(&self, id: &str, name: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE notebooks SET name = ?1, updated_at = datetime('now') WHERE id = ?2",
            params![name, id],
        )?;
        Ok(())
    }

    pub fn delete_notebook(&self, id: &str) -> Result<()> {
        self.conn.execute("DELETE FROM notebooks WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn get_notebook_tree(&self) -> Result<Vec<Notebook>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, parent_id, created_at, updated_at FROM notebooks ORDER BY name"
        )?;
        let rows = stmt.query_map([], |row| Ok(Notebook {
            id: row.get(0)?,
            name: row.get(1)?,
            parent_id: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
        }))?;
        rows.collect()
    }

    // ── Notes ─────────────────────────────────────────────────────────────────

    pub fn create_note(&self, title: &str, notebook_id: Option<&str>) -> Result<Note> {
        let id = Uuid::new_v4().to_string();
        self.conn.execute(
            "INSERT INTO notes (id, title, notebook_id) VALUES (?1, ?2, ?3)",
            params![id, title, notebook_id],
        )?;
        self.get_note(&id)
    }

    pub fn get_note(&self, id: &str) -> Result<Note> {
        self.conn.query_row(
            "SELECT id, title, content, notebook_id, is_pinned, created_at, updated_at FROM notes WHERE id = ?1",
            params![id],
            |row| Ok(Note {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                notebook_id: row.get(3)?,
                is_pinned: row.get::<_, i64>(4)? != 0,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            }),
        )
    }

    pub fn update_note(&self, id: &str, title: Option<&str>, content: Option<&str>, notebook_id: Option<Option<&str>>, is_pinned: Option<bool>) -> Result<Note> {
        let has_title = title.is_some();
        let has_content = content.is_some();
        let has_notebook = notebook_id.is_some();
        let has_pinned = is_pinned.is_some();

        if has_title || has_content || has_notebook || has_pinned {
            let mut query = "UPDATE notes SET updated_at = datetime('now')".to_string();
            let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

            if let Some(t) = title {
                query.push_str(", title = ?");
                params_vec.push(Box::new(t.to_string()));
            }
            if let Some(c) = content {
                query.push_str(", content = ?");
                params_vec.push(Box::new(c.to_string()));
            }
            if let Some(nb) = notebook_id {
                query.push_str(", notebook_id = ?");
                params_vec.push(Box::new(nb.map(String::from)));
            }
            if let Some(p) = is_pinned {
                query.push_str(", is_pinned = ?");
                params_vec.push(Box::new(if p { 1i64 } else { 0i64 }));
            }
            query.push_str(" WHERE id = ?");
            params_vec.push(Box::new(id.to_string()));

            let mut stmt = self.conn.prepare(&query)?;
            stmt.execute(rusqlite::params_from_iter(params_vec.iter().map(|b| b.as_ref())))?;
        }
        self.get_note(id)
    }

    pub fn delete_note(&self, id: &str) -> Result<()> {
        self.conn.execute("DELETE FROM notes WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn delete_notes(&self, ids: &[String]) -> Result<usize> {
        if ids.is_empty() {
            return Ok(0);
        }
        let placeholders: Vec<&str> = ids.iter().map(|_| "?").collect();
        let query = format!(
            "DELETE FROM notes WHERE id IN ({})",
            placeholders.join(", ")
        );
        let params: Vec<&dyn rusqlite::ToSql> = ids.iter().map(|s| s as &dyn rusqlite::ToSql).collect();
        let count = self.conn.execute(&query, params.as_slice())?;
        Ok(count)
    }

    pub fn list_notes(&self, notebook_id: Option<&str>, tag_id: Option<&str>, search_query: Option<&str>) -> Result<Vec<Note>> {
        // Use FTS5 for search queries for performance
        if let Some(q) = search_query {
            if !q.trim().is_empty() {
                return self.search_notes_inner(q, notebook_id, tag_id);
            }
        }
        
        let mut query = "
            SELECT DISTINCT n.id, n.title, n.content, n.notebook_id, n.is_pinned, n.created_at, n.updated_at 
            FROM notes n
        ".to_string();
        
        let mut conditions = Vec::new();
        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(tid) = tag_id {
            query.push_str(" JOIN note_tags nt ON n.id = nt.note_id");
            conditions.push("nt.tag_id = ?");
            params_vec.push(Box::new(tid.to_string()));
        }

        if let Some(nb) = notebook_id {
            conditions.push("n.notebook_id = ?");
            params_vec.push(Box::new(nb.to_string()));
        }

        if !conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&conditions.join(" AND "));
        }

        query.push_str(" ORDER BY n.is_pinned DESC, n.updated_at DESC");

        let mut stmt = self.conn.prepare(&query)?;
        let rows = stmt.query_map(rusqlite::params_from_iter(params_vec.iter().map(|b| b.as_ref())), note_from_row)?;
        rows.collect()
    }

    // ── Tags ──────────────────────────────────────────────────────────────────

    pub fn add_tag(&self, note_id: &str, tag_name: &str) -> Result<Tag> {
        // Upsert tag
        self.conn.execute(
            "INSERT OR IGNORE INTO tags (id, name) VALUES (?1, ?2)",
            params![Uuid::new_v4().to_string(), tag_name],
        )?;
        let tag: Tag = self.conn.query_row(
            "SELECT id, name FROM tags WHERE name = ?1",
            params![tag_name],
            |row| Ok(Tag { id: row.get(0)?, name: row.get(1)? }),
        )?;
        self.conn.execute(
            "INSERT OR IGNORE INTO note_tags (note_id, tag_id) VALUES (?1, ?2)",
            params![note_id, tag.id],
        )?;
        Ok(tag)
    }

    pub fn remove_tag(&self, note_id: &str, tag_id: &str) -> Result<()> {
        self.conn.execute(
            "DELETE FROM note_tags WHERE note_id = ?1 AND tag_id = ?2",
            params![note_id, tag_id],
        )?;
        Ok(())
    }

    pub fn list_tags(&self) -> Result<Vec<Tag>> {
        let mut stmt = self.conn.prepare("SELECT id, name FROM tags ORDER BY name")?;
        let rows = stmt.query_map([], |row| Ok(Tag { id: row.get(0)?, name: row.get(1)? }))?;
        rows.collect()
    }

    // ── Note Links ────────────────────────────────────────────────────────────

    pub fn create_note_link(&self, source_id: &str, target_id: &str, context: Option<&str>) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO note_links (source_note_id, target_note_id, context) VALUES (?1, ?2, ?3)",
            params![source_id, target_id, context],
        )?;
        Ok(())
    }

    pub fn remove_note_link(&self, source_id: &str, target_id: &str) -> Result<()> {
        self.conn.execute(
            "DELETE FROM note_links WHERE source_note_id = ?1 AND target_note_id = ?2",
            params![source_id, target_id],
        )?;
        Ok(())
    }

    pub fn get_backlinks(&self, note_id: &str) -> Result<Vec<NoteLink>> {
        let mut stmt = self.conn.prepare(
            "SELECT source_note_id, target_note_id, context FROM note_links WHERE target_note_id = ?1"
        )?;
        let rows = stmt.query_map(params![note_id], |row| Ok(NoteLink {
            source_note_id: row.get(0)?,
            target_note_id: row.get(1)?,
            context: row.get(2)?,
        }))?;
        rows.collect()
    }

    pub fn get_knowledge_graph(&self) -> Result<GraphData> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, notebook_id FROM notes"
        )?;
        let nodes: Vec<GraphNode> = stmt.query_map([], |row| Ok(GraphNode {
            id: row.get(0)?,
            label: row.get(1)?,
            notebook_id: row.get(2)?,
        }))?.collect::<Result<_>>()?;

        let mut stmt = self.conn.prepare(
            "SELECT source_note_id, target_note_id FROM note_links"
        )?;
        let edges: Vec<GraphEdge> = stmt.query_map([], |row| Ok(GraphEdge {
            source: row.get(0)?,
            target: row.get(1)?,
        }))?.collect::<Result<_>>()?;

        Ok(GraphData { nodes, edges })
    }

    // ── Files ─────────────────────────────────────────────────────────────────

    pub fn create_file(&self, name: &str, path: &str, mime_type: Option<&str>, size: Option<i64>, notebook_id: Option<&str>) -> Result<FileInfo> {
        let id = Uuid::new_v4().to_string();
        self.conn.execute(
            "INSERT INTO files (id, name, path, mime_type, size, notebook_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![id, name, path, mime_type, size, notebook_id],
        )?;
        self.get_file(&id)
    }

    pub fn get_file(&self, id: &str) -> Result<FileInfo> {
        self.conn.query_row(
            "SELECT id, name, path, mime_type, size, notebook_id, created_at FROM files WHERE id = ?1",
            params![id],
            |row| Ok(FileInfo {
                id: row.get(0)?,
                name: row.get(1)?,
                path: row.get(2)?,
                mime_type: row.get(3)?,
                size: row.get(4)?,
                notebook_id: row.get(5)?,
                created_at: row.get(6)?,
            }),
        )
    }

    pub fn list_files(&self, notebook_id: Option<&str>) -> Result<Vec<FileInfo>> {
        if let Some(nb) = notebook_id {
            let mut stmt = self.conn.prepare("SELECT id, name, path, mime_type, size, notebook_id, created_at FROM files WHERE notebook_id = ?1 ORDER BY created_at DESC")?;
            let rows = stmt.query_map(params![nb], file_from_row)?.collect::<Result<Vec<_>>>()?;
            Ok(rows)
        } else {
            let mut stmt = self.conn.prepare("SELECT id, name, path, mime_type, size, notebook_id, created_at FROM files ORDER BY created_at DESC")?;
            let rows = stmt.query_map([], file_from_row)?.collect::<Result<Vec<_>>>()?;
            Ok(rows)
        }
    }

    pub fn delete_file(&self, id: &str) -> Result<()> {
        self.conn.execute("DELETE FROM files WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn get_file_path(&self, id: &str) -> Result<String> {
        self.conn.query_row(
            "SELECT path FROM files WHERE id = ?1",
            params![id],
            |row| row.get(0),
        )
    }

    // ── Search ────────────────────────────────────────────────────────────────

    pub fn search_notes(&self, query: &str) -> Result<Vec<SearchResult>> {
        let safe_query = self.sanitize_fts_query(query);
        if safe_query.len() <= 2 {
            return Ok(vec![]);
        }

        let mut stmt = self.conn.prepare(
            "SELECT n.id, n.title, snippet(notes_fts, 1, '<mark>', '</mark>', '...', 20) \
             FROM notes_fts \
             JOIN notes n ON notes_fts.rowid = n.rowid \
             WHERE notes_fts MATCH ?1 \
             ORDER BY rank LIMIT 50"
        )?;
        let rows = stmt.query_map(params![safe_query], |row| Ok(SearchResult {
            id: row.get(0)?,
            title: row.get(1)?,
            snippet: row.get(2)?,
        }))?;
        rows.collect()
    }
    
    fn sanitize_fts_query(&self, query: &str) -> String {
        let mut sanitized = String::with_capacity(query.len());
        for c in query.chars() {
            match c {
                '"' => sanitized.push_str("\""),
                '*' | '-' | '(' | ')' | '^' | '~' | '{' | '}' | '[' | ']' => {}
                ':' => sanitized.push(' '),
                _ => sanitized.push(c),
            }
        }
        format!(
            "\"{}\"*",
            sanitized.trim().replace('"', "\"\"")
        )
    }
    
    fn search_notes_inner(&self, query: &str, notebook_id: Option<&str>, tag_id: Option<&str>) -> Result<Vec<Note>> {
        let safe_query = self.sanitize_fts_query(query);
        if safe_query.len() <= 2 {
            return Ok(vec![]);
        }

let mut sql = "
            SELECT n.id, n.title, n.content, n.notebook_id, n.is_pinned, n.created_at, n.updated_at 
            FROM notes_fts 
            JOIN notes n ON notes_fts.rowid = n.rowid
            WHERE notes_fts MATCH ?1
        ".to_string();
        
        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = vec![Box::new(safe_query)];
        
        if let Some(tid) = tag_id {
            sql.push_str(" JOIN note_tags nt ON n.id = nt.note_id AND nt.tag_id = ?");
            params_vec.push(Box::new(tid.to_string()));
        }
        
        if let Some(nb) = notebook_id {
            sql.push_str(" AND n.notebook_id = ?");
            params_vec.push(Box::new(nb.to_string()));
        }

        sql.push_str(" ORDER BY n.is_pinned DESC, n.updated_at DESC");

        let mut stmt = self.conn.prepare(&sql)?;
        let rows = stmt.query_map(rusqlite::params_from_iter(params_vec.iter().map(|b| b.as_ref())), note_from_row)?;
        rows.collect()
    }

    // ── Stats ─────────────────────────────────────────────────────────────────

    pub fn get_daily_stats(&self) -> Result<DailyStats> {
        let today_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM notes WHERE date(created_at) = date('now')",
            [],
            |row| row.get(0),
        )?;

        let total_notes: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM notes",
            [],
            |row| row.get(0),
        )?;

        // Streak: consecutive days with at least one note (create OR update)
        let streak: i64 = {
            let mut stmt = self.conn.prepare(
                "SELECT DISTINCT date(d) as d FROM (
                    SELECT created_at as d FROM notes
                    UNION ALL
                    SELECT updated_at as d FROM notes
                ) ORDER BY d DESC"
            )?;
            let dates: Vec<String> = stmt.query_map([], |row| row.get(0))?
                .filter_map(|r| r.ok())
                .collect();
            
            let today = chrono::Local::now().date_naive();
            let mut s = 0i64;
            let mut expected = today;
            
            for d in dates {
                if let Ok(date) = chrono::NaiveDate::parse_from_str(&d, "%Y-%m-%d") {
                    if date == expected {
                        s += 1;
                        expected = date - chrono::TimeDelta::days(1);
                    } else if date == today - chrono::TimeDelta::days(1) && s == 0 {
                        s = 1;
                        expected = date - chrono::TimeDelta::days(1);
                    } else if date < expected {
                        break;
                    }
                }
            }
            s
        };

        let mut stmt = self.conn.prepare(
            "SELECT date(created_at) as d, COUNT(*) as c FROM notes GROUP BY d ORDER BY d DESC LIMIT 30"
        )?;
        let recent_days: Vec<DayCount> = stmt.query_map([], |row| Ok(DayCount {
            date: row.get(0)?,
            count: row.get(1)?,
        }))?.collect::<Result<_>>()?;

        Ok(DailyStats { today_count, streak, total_notes, recent_days })
    }

    // ── Config ────────────────────────────────────────────────────────────────

    pub fn get_config(&self) -> Result<AppConfig> {
        let theme = self.conn.query_row(
            "SELECT value FROM app_config WHERE key = 'theme'",
            [],
            |row| row.get(0),
        ).unwrap_or_else(|_| "dark".to_string());

        let vault_path = self.conn.query_row(
            "SELECT value FROM app_config WHERE key = 'vault_path'",
            [],
            |row| row.get(0),
        ).ok();

        Ok(AppConfig { theme, vault_path })
    }

    pub fn update_config(&self, config: &AppConfig) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO app_config (key, value) VALUES ('theme', ?1)",
            params![config.theme],
        )?;
        if let Some(ref vp) = config.vault_path {
            self.conn.execute(
                "INSERT OR REPLACE INTO app_config (key, value) VALUES ('vault_path', ?1)",
                params![vp],
            )?;
        }
        Ok(())
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn db() -> Database {
        Database::open_in_memory().unwrap()
    }

    #[test]
    fn test_notebook_crud() {
        let db = db();
        let nb = db.create_notebook("Work", None).unwrap();
        assert_eq!(nb.name, "Work");

        let child = db.create_notebook("Projects", Some(&nb.id)).unwrap();
        assert_eq!(child.parent_id, Some(nb.id.clone()));

        db.rename_notebook(&nb.id, "Work2").unwrap();
        let updated = db.get_notebook(&nb.id).unwrap();
        assert_eq!(updated.name, "Work2");

        let tree = db.get_notebook_tree().unwrap();
        assert_eq!(tree.len(), 2);

        db.delete_notebook(&nb.id).unwrap();
        // child should cascade-delete
        let tree = db.get_notebook_tree().unwrap();
        assert_eq!(tree.len(), 0);
    }

    #[test]
    fn test_note_crud() {
        let db = db();
        let note = db.create_note("Hello", None).unwrap();
        assert_eq!(note.title, "Hello");
        assert!(!note.is_pinned);

        let updated = db.update_note(&note.id, Some("Hello World"), Some("{\"type\":\"doc\"}"), None).unwrap();
        assert_eq!(updated.title, "Hello World");
        assert_eq!(updated.content, Some("{\"type\":\"doc\"}".to_string()));

        let notes = db.list_notes(None, None, None).unwrap();
        assert_eq!(notes.len(), 1);

        db.delete_note(&note.id).unwrap();
        let notes = db.list_notes(None, None, None).unwrap();
        assert_eq!(notes.len(), 0);
    }

    #[test]
    fn test_tags() {
        let db = db();
        let note = db.create_note("Tagged", None).unwrap();
        let tag = db.add_tag(&note.id, "rust").unwrap();
        assert_eq!(tag.name, "rust");

        let tags = db.list_tags().unwrap();
        assert_eq!(tags.len(), 1);

        // Adding same tag again should be idempotent
        db.add_tag(&note.id, "rust").unwrap();
        let tags = db.list_tags().unwrap();
        assert_eq!(tags.len(), 1);

        db.remove_tag(&note.id, &tag.id).unwrap();
        // Tag record stays, junction removed
        let notes_by_tag = db.list_notes(None, Some(&tag.id), None).unwrap();
        assert_eq!(notes_by_tag.len(), 0);
    }

    #[test]
    fn test_note_links_and_graph() {
        let db = db();
        let a = db.create_note("A", None).unwrap();
        let b = db.create_note("B", None).unwrap();

        db.create_note_link(&a.id, &b.id, Some("context")).unwrap();
        let backlinks = db.get_backlinks(&b.id).unwrap();
        assert_eq!(backlinks.len(), 1);
        assert_eq!(backlinks[0].source_note_id, a.id);

        let graph = db.get_knowledge_graph().unwrap();
        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.edges.len(), 1);

        db.remove_note_link(&a.id, &b.id).unwrap();
        let graph = db.get_knowledge_graph().unwrap();
        assert_eq!(graph.edges.len(), 0);
    }

    #[test]
    fn test_files() {
        let db = db();
        let f = db.create_file("doc.pdf", "/vault/doc.pdf", Some("application/pdf"), Some(1024), None).unwrap();
        assert_eq!(f.name, "doc.pdf");

        let files = db.list_files(None).unwrap();
        assert_eq!(files.len(), 1);

        db.delete_file(&f.id).unwrap();
        let files = db.list_files(None).unwrap();
        assert_eq!(files.len(), 0);
    }

    #[test]
    fn test_fts_search() {
        let db = db();
        let n1 = db.create_note("Rust Programming", None).unwrap();
        db.update_note(&n1.id, None, Some("Rust is a systems language"), None).unwrap();
        let n2 = db.create_note("Python Basics", None).unwrap();
        db.update_note(&n2.id, None, Some("Python is easy"), None).unwrap();

        let results = db.search_notes("Rust").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Rust Programming");
    }

    #[test]
    fn test_config() {
        let db = db();
        let cfg = db.get_config().unwrap();
        assert_eq!(cfg.theme, "dark");

        db.update_config(&AppConfig { theme: "light".to_string(), vault_path: Some("/vault".to_string()) }).unwrap();
        let cfg = db.get_config().unwrap();
        assert_eq!(cfg.theme, "light");
        assert_eq!(cfg.vault_path, Some("/vault".to_string()));
    }

    #[test]
    fn test_notebook_filter_notes() {
        let db = db();
        let nb = db.create_notebook("Work", None).unwrap();
        db.create_note("Work Note", Some(&nb.id)).unwrap();
        db.create_note("Personal Note", None).unwrap();

        let work_notes = db.list_notes(Some(&nb.id), None, None).unwrap();
        assert_eq!(work_notes.len(), 1);
        assert_eq!(work_notes[0].title, "Work Note");
    }

    #[test]
    fn test_list_notes_combined_filter() {
        let db = db();
        let nb1 = db.create_notebook("Work", None).unwrap();
        let nb2 = db.create_notebook("Personal", None).unwrap();
        db.create_note("Meeting", Some(&nb1.id)).unwrap();
        db.create_note("Shopping", Some(&nb2.id)).unwrap();
        db.create_note("Work Task", Some(&nb1.id)).unwrap();

        // Search for "Work" in nb1 should return "Work Task"
        // Wait, list_notes current implementation:
        // if let Some(q) = search_query { ... returns matches from all notebooks ... }
        
        let _results = db.list_notes(Some(&nb1.id), None, Some("Work")).unwrap();
        // If current implementation is bugged (ignoring notebook_id), this will fail or return unexpected results
        // Actually, "Work" search query will match "Work Task" in nb1.
        // But if it ignores nb1 and searches globally, and I added "Work Task" in nb2 as well...
        
        db.create_note("Work from home", Some(&nb2.id)).unwrap();
        let results = db.list_notes(Some(&nb1.id), None, Some("Work")).unwrap();
        
        // Expected: only 1 note ("Work Task")
        // Actual (buggy): 2 notes ("Work Task" and "Work from home")
        assert_eq!(results.len(), 1, "Should only return matches within the specified notebook");
    }

    #[test]
    fn test_streak_logic() {
        let db = db();
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let yesterday = (chrono::Local::now() - chrono::Duration::days(1)).format("%Y-%m-%d").to_string();
        
        // Scenario 1: Note today
        db.conn.execute("INSERT INTO notes (id, title, created_at) VALUES (?, ?, ?)", params![Uuid::new_v4().to_string(), "Today", today]).unwrap();
        let stats = db.get_daily_stats().unwrap();
        assert_eq!(stats.streak, 1);

        // Scenario 2: Note today and yesterday
        db.conn.execute("INSERT INTO notes (id, title, created_at) VALUES (?, ?, ?)", params![Uuid::new_v4().to_string(), "Yesterday", yesterday]).unwrap();
        let stats = db.get_daily_stats().unwrap();
        assert_eq!(stats.streak, 2);

        // Reset
        db.conn.execute("DELETE FROM notes", []).unwrap();
        
        // Scenario 3: Note only yesterday (streak should be 1)
        db.conn.execute("INSERT INTO notes (id, title, created_at) VALUES (?, ?, ?)", params![Uuid::new_v4().to_string(), "Yesterday", yesterday]).unwrap();
        let stats = db.get_daily_stats().unwrap();
        assert_eq!(stats.streak, 1);
    }
}
