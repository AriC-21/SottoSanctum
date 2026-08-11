mod ai;
mod commands;
mod models;

use crate::ai::LocalBrain;
use std::sync::Arc;

pub struct AppState {
    pub brain: Arc<LocalBrain>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState {
        brain: Arc::new(LocalBrain::new()),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            commands::save_journal_entry,
            commands::get_journal_entries,
            commands::analyze_journal_entry_stream
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}