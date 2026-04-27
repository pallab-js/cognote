pub mod db;
pub mod commands;

use commands::AppState;
use db::Database;
use std::sync::Mutex;
use tauri::Manager;

// ANCHOR: COMMANDS_REGISTERED
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let data_dir = app.path().app_data_dir()
                .expect("failed to get app data dir");
            std::fs::create_dir_all(&data_dir)?;
            let db_path = data_dir.join("cognote.db");
            let vault_path = data_dir.to_string_lossy().to_string();
            let db = Database::open(db_path.to_str().unwrap())
                .expect("failed to open database");
            app.manage(AppState {
                db: Mutex::new(db),
                vault_path: Mutex::new(vault_path),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::create_notebook,
            commands::rename_notebook,
            commands::delete_notebook,
            commands::get_notebook_tree,
            commands::create_note,
            commands::get_note,
            commands::update_note,
            commands::delete_note,
            commands::delete_notes,
            commands::list_notes,
            commands::add_tag,
            commands::remove_tag,
            commands::list_tags,
            commands::create_note_link,
            commands::remove_note_link,
            commands::get_backlinks,
            commands::get_knowledge_graph,
            commands::get_mindmap_data,
            commands::import_file,
            commands::list_files,
            commands::delete_file,
            commands::open_file_external,
            commands::search_notes,
            commands::backup_vault,
            commands::get_daily_stats,
            commands::get_app_config,
            commands::update_app_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
