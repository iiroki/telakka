use std::fs;
use tauri::Manager;

#[tauri::command]
#[specta::specta]
pub async fn storage_read_json(
    app: tauri::AppHandle,
    key: String,
) -> Result<Option<String>, String> {
    let path = storage_path_json(&app, &key);
    if !path.exists() {
        return Ok(None);
    }
    fs::read_to_string(&path)
        .map(Some)
        .map_err(|e| e.to_string())
}

#[tauri::command]
#[specta::specta]
pub async fn storage_write_json(
    app: tauri::AppHandle,
    key: String,
    value: String,
) -> Result<(), String> {
    let path = storage_path_json(&app, &key);
    fs::create_dir_all(path.parent().unwrap()).map_err(|e| e.to_string())?;
    fs::write(&path, value).map_err(|e| e.to_string())
}

fn storage_path_json(app: &tauri::AppHandle, key: &str) -> std::path::PathBuf {
    app.path()
        .app_data_dir()
        .unwrap()
        .join(format!("{key}.json"))
}
