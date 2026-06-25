use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountConfig {
    pub id: String,
    pub label: String,
    pub email: String,
    pub sent_folder: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub root_mail_dir: Option<String>,
    pub default_path: Option<String>,
    pub limit: Option<u16>,
    pub accounts: Option<Vec<AccountConfig>>,
    pub default_sent_folder: Option<String>,
   pub  rmtmmail: Option<String>,
   pub  lthostport: Option<String>,
   pub calendaremail: Option<String>
   
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            root_mail_dir: Some("~/mail".to_string()),
            default_path: Some("".to_string()),
            limit: Some(1000),
            accounts: Some(vec![]),
            default_sent_folder: Some("Sent".to_string()),
            rmtmmail:None,
            lthostport:None,
            calendaremail: Some("barais@irisa.fr".to_string()),

        }
    }
}

pub struct ConfigManager;

impl ConfigManager {
    pub fn load() -> Result<AppConfig, Box<dyn Error>> {
        // In a real Tauri app, we'd use tauri::api::path::app_config_dir()
        // For now, we look for a simple config.json in the current directory or home
        let config_path = PathBuf::from("config.json");
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
