use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub root_mail_dir: String,
    pub default_path: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            root_mail_dir: "".to_string(),
            default_path: "".to_string(),
        }
    }
}

pub struct ConfigManager;

impl ConfigManager {
    pub fn load() -> Result<AppConfig, Box<dyn Error>> {
        // In a real Tauri app, we'd use tauri::api::path::app_config_dir()
        // For now, we look for a simple config.json in the current directory or home
        let config_path = PathBuf::from("/home/barais/git/notmuchtauri/notmuchtauri/config.json");
        println!("Loading config from: {:?}", config_path.as_path().display());
        if !config_path.exists() {
            return Ok(AppConfig::default());
        }
        let content = fs::read_to_string(config_path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;
        let config: AppConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn save(config: &AppConfig) -> Result<(), Box<dyn Error>> {
        let content = serde_json::to_string_pretty(config)?;
        fs::write("config.json", content)?;
        Ok(())
    }
}
