use std::sync::Mutex;
use tauri::State;
use crate::db::{Database, Note, Notebook, Tag, NoteLink, FileInfo, GraphData, SearchResult, DailyStats, AppConfig};
use std::path::Path;
use std::fs;

pub struct AppState {
    pub db: Mutex<Database>,
    pub vault_path: Mutex<String>,
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
) -> Result<Note, String> {
    state.db.lock().unwrap()
        .update_note(
            &id,
            title.as_deref(),
            content.as_deref(),
            notebook_id.as_ref().map(|o| o.as_deref()),
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
pub fn list_notes(state: State<AppState>, notebook_id: Option<String>, tag_id: Option<String>) -> Result<Vec<Note>, String> {
    state.db.lock().unwrap()
        .list_notes(notebook_id.as_deref(), tag_id.as_deref())
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
    let vault = state.vault_path.lock().unwrap().clone();
    let src = Path::new(&source_path);
    let file_name = src.file_name()
        .ok_or("Invalid file path")?
        .to_string_lossy()
        .to_string();

    let dest_dir = Path::new(&vault).join("files");
    fs::create_dir_all(&dest_dir).map_err(|e| e.to_string())?;
    let dest = dest_dir.join(&file_name);
    fs::copy(src, &dest).map_err(|e| e.to_string())?;

    let size = fs::metadata(&dest).ok().map(|m| m.len() as i64);
    let mime = infer_mime(&file_name);
    let rel_path = format!("files/{}", file_name);

    state.db.lock().unwrap()
        .create_file(&file_name, &rel_path, mime.as_deref(), size, notebook_id.as_deref())
        .map_err(|e| e.to_string())
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
    let vault = state.vault_path.lock().unwrap().clone();
    let full_path = Path::new(&vault).join(&path);
    let _ = fs::remove_file(full_path); // best-effort
    db.delete_file(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn open_file_external(state: State<AppState>, id: String) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    let path = db.get_file_path(&id).map_err(|e| e.to_string())?;
    let vault = state.vault_path.lock().unwrap().clone();
    let full_path = Path::new(&vault).join(&path);
    opener::open(full_path).map_err(|e| e.to_string())
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
