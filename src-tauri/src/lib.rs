mod docker;

use docker::commands::{version as docker_version, DockerStatus};
use tauri_specta::{collect_commands, Builder};

#[tauri::command]
#[specta::specta]
fn status() -> Result<DockerStatus, String> {
    docker_version()
}

pub fn specta_builder() -> Builder<tauri::Wry> {
    Builder::<tauri::Wry>::new().commands(collect_commands![status])
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = specta_builder();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(builder.invoke_handler())
        .run(tauri::generate_context!())
        .expect("Error while running Tauri app");
}
