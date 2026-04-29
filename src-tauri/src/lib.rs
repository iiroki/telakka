mod docker;
mod utils;

use docker::commands as DockerCommands;
use docker::types::{
    DockerContainerAction, DockerContainerResult, DockerContainerStats, DockerProjectAction,
    DockerStatus,
};
use tauri_specta::{collect_commands, Builder};

#[tauri::command]
#[specta::specta]
async fn get_status() -> Result<DockerStatus, String> {
    tauri::async_runtime::spawn_blocking(DockerCommands::get_status)
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
#[specta::specta]
async fn get_containers() -> Result<DockerContainerResult, String> {
    tauri::async_runtime::spawn_blocking(DockerCommands::get_containers)
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
#[specta::specta]
async fn run_container_action(command: DockerContainerAction, id: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        DockerCommands::run_container_action(&command, &id)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
#[specta::specta]
async fn run_project_action(command: DockerProjectAction, project: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        DockerCommands::run_project_action(&command, &project)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
#[specta::specta]
async fn get_container_stats() -> Result<Vec<DockerContainerStats>, String> {
    tauri::async_runtime::spawn_blocking(DockerCommands::get_container_stats)
        .await
        .map_err(|e| e.to_string())?
}

pub fn specta_builder() -> Builder<tauri::Wry> {
    Builder::<tauri::Wry>::new().commands(collect_commands![
        get_status,
        get_containers,
        get_container_stats,
        run_container_action,
        run_project_action,
    ])
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = specta_builder();
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(tauri_plugin_log::log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(builder.invoke_handler())
        .run(tauri::generate_context!())
        .expect("Error while running Tauri app");
}
