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
    #[serde(default)]
    pub is_default: Option<bool>, // <-- NOUVEAU: Permet au frontend de savoir lequel sélectionner par défaut
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShortcuConfig {
    pub shortcut: String,
    pub text: String
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub root_mail_dir: Option<String>,
    pub default_path: Option<String>,
    pub limit: Option<u16>,
    pub accounts: Option<Vec<AccountConfig>>,
    pub default_sent_folder: Option<String>,
    pub rmtmmail: Option<String>,
    pub lthostport: Option<String>,
    pub calendaremail: Option<String>,
    pub llm: Option<LlmConfig>,
    pub shortcuts: Option<Vec<ShortcuConfig>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LlmConfig {
    pub api_url: Option<String>,
    pub api_key: Option<String>,
    pub model: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            root_mail_dir: Some("~/mail".to_string()),
            default_path: Some("".to_string()),
            limit: Some(1000),
            accounts: Some(vec![]),
            default_sent_folder: Some("Sent".to_string()),
            rmtmmail: None,
            lthostport: None,
            calendaremail: Some("barais@irisa.fr".to_string()),
            llm: None,
            shortcuts:None
        }
    }
}

pub struct ConfigManager;

impl ConfigManager {

    /// Lit et parse le fichier ~/.msmtprc pour extraire les comptes et le compte par défaut
    fn parse_msmtprc() -> (Vec<AccountConfig>, Option<String>) {
        let mut accounts = Vec::new();
        let mut default_account = None;

        if let Some(mut home) = dirs::home_dir() {
            home.push(".msmtprc");

            if let Ok(content) = fs::read_to_string(home) {
                let mut current_id = String::new();
                let mut current_email = String::new();

                // Fonction utilitaire pour sauvegarder le compte en cours de lecture
                let mut save_current_account = |id: &mut String, email: &mut String| {
                    if !id.is_empty() && !email.is_empty() {
                        accounts.push(AccountConfig {
                            id: id.clone(),
                            label: id.clone(), // Le label est l'ID par défaut
                            email: email.clone(),
                            sent_folder: None,
                            is_default: None,
                        });
                        *id = String::new();
                        *email = String::new();
                    }
                };

                for line in content.lines() {
                    let line = line.trim();
                    if line.is_empty() || line.starts_with('#') {
                        continue;
                    }

                    // Cherche la déclaration du compte par défaut "account default : irisa"
                    if line.starts_with("account default") {
                        if let Some(idx) = line.find(':') {
                            default_account = Some(line[idx + 1..].trim().to_string());
                        }
                    } 
                    // Cherche un nouveau compte "account irisa"
                    else if line.starts_with("account ") {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 2 {
                            save_current_account(&mut current_id, &mut current_email);
                            current_id = parts[1].to_string();
                        }
                    } 
                    // Cherche l'adresse email d'expédition "from barais@irisa.fr"
                    else if line.starts_with("from ") {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 2 {
                            current_email = parts[1].to_string();
                        }
                    }
                }
                // N'oublie pas d'enregistrer le tout dernier bloc de compte lu
                save_current_account(&mut current_id, &mut current_email);
            }
        }

        (accounts, default_account)
    }

    pub fn load() -> Result<AppConfig, Box<dyn Error>> {
        let config_path = PathBuf::from("config.json");
        
        // 1. Charger la config JSON de base (ou prendre celle par défaut)
        let mut config = if config_path.exists() {
            let content = fs::read_to_string(&config_path)
                .map_err(|e| format!("Failed to read config file: {}", e))?;
            serde_json::from_str(&content)?
        } else {
            AppConfig::default()
        };

        // 2. Extraire les comptes directement depuis ~/.msmtprc
        let (msmtp_accounts, default_id) = Self::parse_msmtprc();
        let mut existing_accounts = config.accounts.unwrap_or_default();

        // 3. Fusionner les comptes MSMPT avec ceux du JSON (pour garder les labels customs)
        for msmtp_acc in msmtp_accounts {
            let is_this_default = default_id.as_ref() == Some(&msmtp_acc.id);

            // Si le compte existe déjà dans le JSON, on met juste à jour l'email et l'état par défaut
            if let Some(existing) = existing_accounts.iter_mut().find(|a| a.id == msmtp_acc.id) {
                existing.email = msmtp_acc.email;
                existing.is_default = Some(is_this_default);
            } else {
                // Sinon, c'est un nouveau compte trouvé dans msmtp, on l'ajoute
                let mut new_acc = msmtp_acc.clone();
                new_acc.is_default = Some(is_this_default);
                existing_accounts.push(new_acc);
            }
        }

        // Si aucun compte n'est défini comme par défaut explicitement dans msmtprc,
        // on assigne le premier par précaution pour le frontend.
        if !existing_accounts.is_empty() && !existing_accounts.iter().any(|a| a.is_default == Some(true)) {
            existing_accounts[0].is_default = Some(true);
        }

        config.accounts = Some(existing_accounts);

        Ok(config)
    }

    pub fn save(config: &AppConfig) -> Result<(), Box<dyn Error>> {
        let content = serde_json::to_string_pretty(config)?;
        fs::write("config.json", content)?;
        Ok(())
    }
}
