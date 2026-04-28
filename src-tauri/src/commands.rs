use std::sync::{Mutex, RwLock};
use tauri::State;
use crate::db::{Database, Note, Notebook, Tag, NoteLink, FileInfo, GraphData, SearchResult, DailyStats, AppConfig, Task};
use std::path::Path;
use std::fs;

const MAX_TITLE_LENGTH: usize = 500;
const MAX_TASK_LENGTH: usize = 2000;
const MAX_FILE_SIZE: i64 = 100 * 1024 * 1024; // 100MB

fn validate_title(title: &str) -> Result<(), String> {
    let trimmed = title.trim();
    if trimmed.is_empty() {
        return Err("Title cannot be empty".to_string());
    }
    if trimmed.len() > MAX_TITLE_LENGTH {
        return Err(format!("Title cannot exceed {} characters", MAX_TITLE_LENGTH));
    }
    Ok(())
}

fn validate_task_content(content: &str) -> Result<(), String> {
    let trimmed = content.trim();
    if trimmed.is_empty() {
        return Err("Task content cannot be empty".to_string());
    }
    if trimmed.len() > MAX_TASK_LENGTH {
        return Err(format!("Task content cannot exceed {} characters", MAX_TASK_LENGTH));
    }
    Ok(())
}

pub struct AppState {
    pub db: Mutex<Database>,
    pub vault_path: RwLock<String>,
}

// ── Notebooks ─────────────────────────────────────────────────────────────────

#[tauri::command]
pub fn create_notebook(state: State<AppState>, name: String, parent_id: Option<String>) -> Result<Notebook, String> {
    state.db.lock().unwrap()
        .create_notebook(&name, parent_id.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn rename_notebook(state: State<AppState>, id: String, name: String) -> Result<(), String> {
    let name = name.trim().to_string();
    validate_title(&name)?;
    state.db.lock().unwrap()
        .rename_notebook(&id, &name)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_notebook(state: State<AppState>, id: String) -> Result<(), String> {
    state.db.lock().unwrap()
        .delete_notebook(&id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_notebook_tree(state: State<AppState>) -> Result<Vec<Notebook>, String> {
    state.db.lock().unwrap()
        .get_notebook_tree()
        .map_err(|e| e.to_string())
}

// ── Notes ─────────────────────────────────────────────────────────────────────

#[tauri::command]
pub fn create_note(state: State<AppState>, title: String, notebook_id: Option<String>) -> Result<Note, String> {
    let title = title.trim().to_string();
    validate_title(&title)?;
    state.db.lock().unwrap()
        .create_note(&title, notebook_id.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_note(state: State<AppState>, id: String) -> Result<Note, String> {
    state.db.lock().unwrap()
        .get_note(&id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_note(
    state: State<AppState>,
    id: String,
    title: Option<String>,
    content: Option<String>,
    notebook_id: Option<Option<String>>,
    is_pinned: Option<bool>,
) -> Result<Note, String> {
    if let Some(ref t) = title {
        let t = t.trim().to_string();
        validate_title(&t)?;
        return state.db.lock().unwrap()
            .update_note(
                &id,
                Some(&t),
                content.as_deref(),
                notebook_id.as_ref().map(|o| o.as_deref()),
                is_pinned,
            )
            .map_err(|e| e.to_string());
    }
    state.db.lock().unwrap()
        .update_note(
            &id,
            title.as_deref(),
            content.as_deref(),
            notebook_id.as_ref().map(|o| o.as_deref()),
            is_pinned,
        )
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_note(state: State<AppState>, id: String) -> Result<(), String> {
    state.db.lock().unwrap()
        .delete_note(&id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_notes(state: State<AppState>, ids: Vec<String>) -> Result<usize, String> {
    if ids.len() > 1000 {
        return Err("Cannot delete more than 1000 notes at once".to_string());
    }
    state.db.lock().unwrap()
        .delete_notes(&ids)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_notes(state: State<AppState>, notebook_id: Option<String>, tag_id: Option<String>, search_query: Option<String>) -> Result<Vec<Note>, String> {
    state.db.lock().unwrap()
        .list_notes(notebook_id.as_deref(), tag_id.as_deref(), search_query.as_deref())
        .map_err(|e| e.to_string())
}

// ── Tags ──────────────────────────────────────────────────────────────────────

#[tauri::command]
pub fn add_tag(state: State<AppState>, note_id: String, tag_name: String) -> Result<Tag, String> {
    state.db.lock().unwrap()
        .add_tag(&note_id, &tag_name)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn remove_tag(state: State<AppState>, note_id: String, tag_id: String) -> Result<(), String> {
    state.db.lock().unwrap()
        .remove_tag(&note_id, &tag_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_tags(state: State<AppState>) -> Result<Vec<Tag>, String> {
    state.db.lock().unwrap()
        .list_tags()
        .map_err(|e| e.to_string())
}

// ── Note Links ────────────────────────────────────────────────────────────────

#[tauri::command]
pub fn create_note_link(state: State<AppState>, source_id: String, target_id: String, context: Option<String>) -> Result<(), String> {
    state.db.lock().unwrap()
        .create_note_link(&source_id, &target_id, context.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn remove_note_link(state: State<AppState>, source_id: String, target_id: String) -> Result<(), String> {
    state.db.lock().unwrap()
        .remove_note_link(&source_id, &target_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_backlinks(state: State<AppState>, note_id: String) -> Result<Vec<NoteLink>, String> {
    state.db.lock().unwrap()
        .get_backlinks(&note_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_knowledge_graph(state: State<AppState>) -> Result<GraphData, String> {
    state.db.lock().unwrap()
        .get_knowledge_graph()
        .map_err(|e| e.to_string())
}

// ── Files ─────────────────────────────────────────────────────────────────────

#[tauri::command]
pub fn import_file(state: State<AppState>, source_path: String, notebook_id: Option<String>) -> Result<FileInfo, String> {
    let vault = state.vault_path.read().unwrap().clone();

    // Fix 1: canonicalize source and verify it's a real file (prevents path traversal)
    let src = fs::canonicalize(&source_path).map_err(|_| "Source file not found")?;
    if !src.is_file() {
        return Err("Source path is not a file".into());
    }

    // Check file size limit
    let file_size = fs::metadata(&src).map_err(|e| e.to_string())?.len() as i64;
    if file_size > MAX_FILE_SIZE {
        return Err(format!("File exceeds maximum size of {} MB", MAX_FILE_SIZE / 1024 / 1024));
    }

    let orig_name = src.file_name()
        .ok_or("Invalid file path")?
        .to_string_lossy()
        .to_string();

    // Fix 3: UUID-prefix the destination filename to avoid collisions
    let file_name = format!("{}_{}", uuid::Uuid::new_v4(), orig_name);

    let dest_dir = Path::new(&vault).join("files");
    fs::create_dir_all(&dest_dir).map_err(|e| e.to_string())?;
    let dest = dest_dir.join(&file_name);
    
    // Verify destination stays within vault (prevent path traversal via symlinks)
    let dest_canonical = fs::canonicalize(&dest_dir).map_err(|e| e.to_string())?;
    let vault_canonical = fs::canonicalize(&vault).map_err(|e| e.to_string())?;
    if !dest_canonical.starts_with(&vault_canonical) {
        return Err("Invalid destination path".to_string());
    }
    
    fs::copy(&src, &dest).map_err(|e| e.to_string())?;

    let size = fs::metadata(&dest).ok().map(|m| m.len() as i64);
    let mime = infer_mime(&orig_name);
    let rel_path = format!("files/{}", file_name);

    state.db.lock().unwrap()
        .create_file(&orig_name, &rel_path, mime.as_deref(), size, notebook_id.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_image(state: State<AppState>, filename: String, bytes: Vec<u8>) -> Result<String, String> {
    let vault = state.vault_path.read().unwrap().clone();
    
    // Extract only the base file name to prevent path traversal
    let safe_base_name = Path::new(&filename)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("image.png");
        
    let safe_filename = format!("{}_{}", uuid::Uuid::new_v4(), safe_base_name);
    let dest_dir = Path::new(&vault).join("files");
    fs::create_dir_all(&dest_dir).map_err(|e| e.to_string())?;
    let dest = dest_dir.join(&safe_filename);
    
    // Verify destination stays within vault
    let dest_canonical = fs::canonicalize(&dest_dir).map_err(|e| e.to_string())?;
    let vault_canonical = fs::canonicalize(&vault).map_err(|e| e.to_string())?;
    if !dest_canonical.starts_with(&vault_canonical) {
        return Err("Invalid destination path".to_string());
    }
    
    fs::write(&dest, &bytes).map_err(|e| e.to_string())?;
    
    // Return absolute path so frontend can use convertFileSrc
    let absolute_path = dest.to_string_lossy().to_string();
    Ok(absolute_path)
}

fn infer_mime(name: &str) -> Option<String> {
    let ext = Path::new(name).extension()?.to_str()?;
    Some(match ext.to_lowercase().as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "pdf" => "application/pdf",
        "txt" => "text/plain",
        "md" => "text/markdown",
        _ => return None,
    }.to_string())
}

#[tauri::command]
pub fn list_files(state: State<AppState>, notebook_id: Option<String>) -> Result<Vec<FileInfo>, String> {
    state.db.lock().unwrap()
        .list_files(notebook_id.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_file(state: State<AppState>, id: String) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    let path = db.get_file_path(&id).map_err(|e| e.to_string())?;
    let vault = state.vault_path.read().unwrap().clone();
    let full_path = Path::new(&vault).join(&path);
    
    // Security: canonicalize and verify the resolved path stays within the vault
    let canonical = fs::canonicalize(&full_path).map_err(|e| e.to_string())?;
    let vault_canonical = fs::canonicalize(&vault).map_err(|e| e.to_string())?;
    
    // Verify path doesn't escape vault (handles symlinks, relative paths, etc.)
    if !canonical.starts_with(&vault_canonical) {
        return Err("Invalid file path: path escapes vault directory".to_string());
    }
    
    fs::remove_file(&canonical).map_err(|e| format!("Failed to delete file: {}", e))?;
    db.delete_file(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn open_file_external(state: State<AppState>, id: String) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    let path = db.get_file_path(&id).map_err(|e| e.to_string())?;
    let vault = state.vault_path.read().unwrap().clone();
    let full_path = Path::new(&vault).join(&path);
    
    // Security: canonicalize and verify the resolved path stays within the vault
    let canonical = fs::canonicalize(&full_path).map_err(|e| e.to_string())?;
    let vault_canonical = fs::canonicalize(&vault).map_err(|e| e.to_string())?;
    
    // Verify path doesn't escape vault (handles symlinks, relative paths, etc.)
    if !canonical.starts_with(&vault_canonical) {
        return Err("Invalid file path: path escapes vault directory".to_string());
    }
    
    opener::open(canonical).map_err(|e| e.to_string())
}

// ── Mind Map ──────────────────────────────────────────────────────────────────

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct MindMapNode {
    pub id: String,
    pub label: String,
    pub children: Vec<MindMapNode>,
}

#[tauri::command]
pub fn get_mindmap_data(state: State<AppState>, note_id: String) -> Result<MindMapNode, String> {
    let db = state.db.lock().unwrap();
    let note = db.get_note(&note_id).map_err(|e| e.to_string())?;
    let content = note.content.unwrap_or_default();
    let root = parse_headings_to_tree(&note_id, &note.title, &content);
    Ok(root)
}

fn parse_headings_to_tree(note_id: &str, title: &str, content: &str) -> MindMapNode {
    let mut root = MindMapNode { id: note_id.to_string(), label: title.to_string(), children: vec![] };
    if let Ok(doc) = serde_json::from_str::<serde_json::Value>(content) {
        if let Some(nodes) = doc["content"].as_array() {
            let mut stack: Vec<(u64, usize)> = vec![]; // (level, index into children)
            for node in nodes {
                if node["type"] == "heading" {
                    let level = node["attrs"]["level"].as_u64().unwrap_or(1);
                    let text = node["content"].as_array()
                        .and_then(|c| c.first())
                        .and_then(|t| t["text"].as_str())
                        .unwrap_or("").to_string();
                    if text.is_empty() { continue; }
                    let child = MindMapNode { id: uuid::Uuid::new_v4().to_string(), label: text, children: vec![] };
                    // Pop stack until parent level < current level
                    while stack.len() > 1 && stack.last().map(|(l, _)| *l >= level).unwrap_or(false) {
                        stack.pop();
                    }
                    // Find parent (either root or last child at lower level)
                    let parent_idx = if stack.len() == 0 { 0 } else { stack.len() - 1 };
                    let parent_children = if parent_idx == 0 { &mut root.children } else { 
                        // Traverse to find the right parent
                        let mut parent = &mut root;
                        for (_, idx) in stack.iter().skip(1) {
                            parent = &mut parent.children[*idx];
                        }
                        &mut parent.children
                    };
                    parent_children.push(child);
                    let new_idx = parent_children.len() - 1;
                    stack.push((level, new_idx));
                }
            }
        }
    }
    root
}

// ── Backup ────────────────────────────────────────────────────────────────────

#[tauri::command]
pub fn backup_vault(state: State<AppState>) -> Result<String, String> {
    let vault = state.vault_path.read().unwrap().clone();
    let vault_path = Path::new(&vault);

    // Fix 4: write backup to a sibling directory so it's never inside the vault
    let backup_dir = vault_path.parent()
        .ok_or("Vault has no parent directory")?
        .join("cognote-backups");
    fs::create_dir_all(&backup_dir).map_err(|e| e.to_string())?;
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let backup_path = backup_dir.join(format!("backup_{}.zip", timestamp));

    let file = fs::File::create(&backup_path).map_err(|e| e.to_string())?;
    let mut zip = zip::ZipWriter::new(file);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    for entry in walkdir::WalkDir::new(vault_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let rel = entry.path().strip_prefix(vault_path).map_err(|e| e.to_string())?;
        zip.start_file(rel.to_string_lossy(), options).map_err(|e| e.to_string())?;
        let mut f = fs::File::open(entry.path()).map_err(|e| e.to_string())?;
        std::io::copy(&mut f, &mut zip).map_err(|e| e.to_string())?;
    }
    zip.finish().map_err(|e| e.to_string())?;
    Ok(backup_path.to_string_lossy().to_string())
}

// ── Search ────────────────────────────────────────────────────────────────────

#[tauri::command]
pub fn search_notes(state: State<AppState>, query: String) -> Result<Vec<SearchResult>, String> {
    state.db.lock().unwrap()
        .search_notes(&query)
        .map_err(|e| e.to_string())
}

// ── Stats ─────────────────────────────────────────────────────────────────────

#[tauri::command]
pub fn get_daily_stats(state: State<AppState>) -> Result<DailyStats, String> {
    state.db.lock().unwrap()
        .get_daily_stats()
        .map_err(|e| e.to_string())
}

// ── Config ────────────────────────────────────────────────────────────────────

#[tauri::command]
pub fn get_app_config(state: State<AppState>) -> Result<AppConfig, String> {
    state.db.lock().unwrap()
        .get_config()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_app_config(state: State<AppState>, theme: Option<String>, vault_path: Option<String>) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    let mut cfg = db.get_config().map_err(|e| e.to_string())?;
    if let Some(t) = theme { cfg.theme = t; }
    if vault_path.is_some() { cfg.vault_path = vault_path; }
    db.update_config(&cfg).map_err(|e| e.to_string())
}

// ── Tasks ─────────────────────────────────────────────────────────────────────

#[tauri::command]
pub fn create_task(state: State<AppState>, content: String, note_id: Option<String>, due_date: Option<String>) -> Result<Task, String> {
    let content = content.trim().to_string();
    validate_task_content(&content)?;
    state.db.lock().unwrap()
        .create_task(&content, note_id.as_deref(), due_date.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_task(state: State<AppState>, id: String) -> Result<Task, String> {
    state.db.lock().unwrap()
        .get_task(&id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_task(
    state: State<AppState>,
    id: String,
    content: Option<String>,
    is_completed: Option<bool>,
    due_date: Option<Option<String>>,
) -> Result<Task, String> {
    if let Some(ref c) = content {
        validate_task_content(c)?;
    }
    state.db.lock().unwrap()
        .update_task(&id, content.as_deref(), is_completed, due_date.as_ref().map(|o| o.as_deref()))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_task(state: State<AppState>, id: String) -> Result<(), String> {
    state.db.lock().unwrap()
        .delete_task(&id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_tasks(state: State<AppState>, note_id: Option<String>, completed: Option<bool>) -> Result<Vec<Task>, String> {
    state.db.lock().unwrap()
        .list_tasks(note_id.as_deref(), completed)
        .map_err(|e| e.to_string())
}
