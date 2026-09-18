use tauri::plugin::TauriPlugin;
use tauri::Runtime;

const LEVEL_ENV: &str = "TELAKKA_LOG_LEVEL";
const LEVEL_DEFAULT: ::log::LevelFilter = ::log::LevelFilter::Info;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    tauri_plugin_log::Builder::new().level(level()).build()
}

fn level() -> ::log::LevelFilter {
    match std::env::var(LEVEL_ENV) {
        Ok(value) => value.parse().unwrap_or(LEVEL_DEFAULT),
        Err(_) => LEVEL_DEFAULT,
    }
}
