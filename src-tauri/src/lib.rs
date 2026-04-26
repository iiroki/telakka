mod docker;
mod utils;

use docker::commands as DockerCommands;
use docker::types::{DockerContainerCommand, DockerContainerResult, DockerStatus};
use tauri_specta::{collect_commands, Builder};

#[tauri::command]
#[specta::specta]
fn get_status() -> Result<DockerStatus, String> {
    DockerCommands::get_status()
}

#[tauri::command]
#[specta::specta]
fn get_containers() -> Result<DockerContainerResult, String> {
    DockerCommands::get_containers()
}

#[tauri::command]
#[specta::specta]
fn run_container_command(command: DockerContainerCommand, id: String) -> Result<(), String> {
    DockerCommands::run_container_command(&command, &id)
}

pub fn specta_builder() -> Builder<tauri::Wry> {
    Builder::<tauri::Wry>::new().commands(collect_commands![
        get_status,
        get_containers,
        run_container_command
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
