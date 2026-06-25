// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod folder_scan;
mod msmtp;
mod notmuch;
use config::{AppConfig, ConfigManager};
use folder_scan::{FolderNode, FolderScanner};
use msmtp::{EmailPayload, MSMTPWrapper};
use notmuch::{AddressMatch, Message, NotMuchWrapper, ReplyData};

use crate::notmuch::ThreadDto;

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
fn search_messages(
    query: String,
    limit: Option<u32>,
    sort: Option<String>,
) -> Result<Vec<Message>, String> {
    let sort_ref = sort.as_deref();
    let raw_data = NotMuchWrapper::search(&query, limit, sort_ref).map_err(|e| e.to_string())?;

    let flattened_messages = raw_data
        .into_iter()
        .map(|elem| Message {
            id: elem.thread,
            subject: elem.subject,
            from: elem.authors,
            to: "Unknown".to_string(),
            date: elem.date_relative,
            body: "".to_string(),
            tags: elem.tags,
            is_read: false,
            has_attachments: false,
        })
        .collect();

    Ok(flattened_messages)
}

#[tauri::command]
fn get_message_details(id: String) -> Result<Vec<ThreadDto>, String> {
    NotMuchWrapper::get_thread_details(&id).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_message_part(message_id: &str, part_id: u32) -> Result<String, String> {
    NotMuchWrapper::get_message_part(&message_id, part_id).map_err(|e| e.to_string())
}

#[tauri::command]
fn save_message_part(message_id: &str, part_id: u32, output_path: &str) -> Result<(), String> {
    NotMuchWrapper::save_message_part(&message_id, part_id, &output_path).map_err(|e| e.to_string())
}

#[tauri::command]
fn modify_message_tag(message_id: &str, tag: &str, action: &str) -> Result<(), String> {
    NotMuchWrapper::modify_message_tag(&message_id, &tag, &action).map_err(|e| e.to_string())
}

#[tauri::command]
async fn send_email(payload: EmailPayload) -> Result<(), String> {
    MSMTPWrapper::send_email(payload)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_reply_data(
    message_id: String,
    reply_mode: String,
    message: Message,
) -> Result<ReplyData, String> {
    NotMuchWrapper::get_reply_data(message_id, reply_mode, message).map_err(|e| e.to_string())
}
#[tauri::command]
fn lookup_address(query: String, limit: usize) -> Result<Vec<AddressMatch>, String> {
    NotMuchWrapper::lookup_address_limited(&query, limit).map_err(|e| e.to_string())
}

#[tauri::command]
async fn send_ics_email(
    to_addresses: Vec<String>, 
    subject: &str, 
    body: &str, 
    ics_content: &str,
    sentfolder:&str
) -> Result<(), String>{
        MSMTPWrapper::send_ics_email(to_addresses,subject,body,ics_content,sentfolder)
        .await
        .map_err(|e| e.to_string())
}



/*
 fn lookup_address(query: &str) -> Result<Vec<AddressMatch>, String> {
NotMuchWrapper::lookup_address(query).map_err(|e| e.to_string())

 }*/

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(|app, args, cwd| {}))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            search_messages,
            get_message_details,
            get_config,
            save_config,
            scan_mail_folders,
            get_message_part,
            save_message_part,
            modify_message_tag,
            send_email,
            get_reply_data,
            lookup_address,
            send_ics_email
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
