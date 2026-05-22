#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use vinaria_core::BottleManager;

#[tauri::command]
fn list_bottles() -> Vec<vinaria_core::Bottle> {
    BottleManager::new().list()
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "vinaria=debug,info".into()),
        )
        .init();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![list_bottles])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
