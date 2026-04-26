use std::fs;
use std::path::Path;

use specta_typescript::Typescript;

fn main() {
    let output = "../src/tauri/bindings.gen.ts";
    if let Some(parent) = Path::new(output).parent() {
        fs::create_dir_all(parent).expect("Failed to create output dir");
    }

    telakka_lib::specta_builder()
        .export(Typescript::default(), output)
        .expect("Failed to generate TypeScript bindings");
}
