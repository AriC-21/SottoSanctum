use crate::models::JournalPayload;
use crate::AppState;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::ipc::Channel;
use tauri::State;

#[tauri::command]
pub fn save_journal_entry(payload: JournalPayload) -> Result<String, String> {
    let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let journal_dir = PathBuf::from(home_dir).join("Documents").join("JournalData");

    if !journal_dir.exists() {
        fs::create_dir_all(&journal_dir).map_err(|e| e.to_string())?;
    }

    let sanitized_time: String = payload
        .timestamp
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect();
    let file_name = format!("entry_{}.md", sanitized_time);
    let file_path = journal_dir.join(file_name);

    let file_content = payload.to_file_string().map_err(|e| e.to_string())?;
    fs::write(&file_path, file_content).map_err(|e| e.to_string())?;

    Ok(format!("Saved: {}", file_path.display()))
}

#[tauri::command]
pub fn get_journal_entries() -> Result<Vec<JournalPayload>, String> {
    let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let journal_dir = PathBuf::from(home_dir).join("Documents").join("JournalData");

    if !journal_dir.exists() {
        return Ok(Vec::new());
    }

    let mut entries = Vec::new();
    let read_dir = fs::read_dir(&journal_dir).map_err(|e| e.to_string())?;

    for entry in read_dir {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(journal) = JournalPayload::from_file_string(&content) {
                    entries.push(journal);
                }
            }
        }
    }

    entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    Ok(entries)
}

#[tauri::command]
pub async fn analyze_journal_entry_stream(
    content: String,
    on_token: Channel<String>,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let brain = Arc::clone(&state.brain);

    tokio::task::spawn_blocking(move || {
        brain.generate_response_stream(&content, move |token| {
            let _ = on_token.send(token);
        })
    })
    .await
    .map_err(|e| format!("Thread join error: {}", e))?
}