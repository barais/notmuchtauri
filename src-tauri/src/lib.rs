// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod notmuch;
mod config;
mod folder_scan;
use notmuch::{NotMuchWrapper, Message, Messag, MessagElement};
use config::{ConfigManager, AppConfig};
use folder_scan::{FolderScanner, FolderNode};

#[tauri::command]
fn get_config() -> Result<AppConfig, String> {
    ConfigManager::load().map_err(|e| e.to_string())
}

#[tauri::command]
fn save_config(config: AppConfig) -> Result<(), String> {
    ConfigManager::save(&config).map_err(|e| e.to_string())
}

#[tauri::command]
fn scan_mail_folders(root_path: String) -> Result<Vec<FolderNode>, String> {
    let path = std::path::Path::new(&root_path);
    FolderScanner::scan(path).map_err(|e| e.to_string())
}

#[tauri::command]
fn search_messages(query: String, limit: Option<u32>, sort: Option<String>) -> Result<Vec<Message>, String> {
    let sort_ref = sort.as_deref();
    let raw_data = NotMuchWrapper::search(&query, limit, sort_ref).map_err(|e| e.to_string())?;

    let flattened_messages = raw_data.into_iter().map(|elem| Message {
        id: elem.thread,
        subject: elem.subject,
        from: elem.authors,
        to: "Unknown".to_string(),
        date: elem.date_relative,
        body: "".to_string(),
        tags: elem.tags,
        is_read: false,
        has_attachments: false,
    }).collect();

    Ok(flattened_messages)
}

#[tauri::command]
fn get_message_details(id: String) -> Result<Message, String> {
    NotMuchWrapper::get_message_details(&id).map_err(|e| e.to_string())
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            search_messages,
            get_message_details,
            get_config,
            save_config,
            scan_mail_folders
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

