// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod filereader;
mod database;
mod settings;

#[derive(serde::Serialize)]
struct CustomResponse {
  message: String,
}

enum ApplicationTypes {
    HFM,
    Planning
}

enum ProjestSources {
    Empty,
    File
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(user_name: &str) -> CustomResponse {
    CustomResponse{ 
        message: format!("Hello, {}! You've been greeted from Rust!", user_name)
    }
}


/// Command for import metadata file to system
#[tauri::command]
fn import_metadata_file(){
    todo!()
}

// Command for create metadata project in database
fn create_metadata_project(project_name: &str, app_type: ApplicationTypes, source: ProjestSources) {
    todo!()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
