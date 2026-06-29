use std::fmt::Pointer;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command;
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

#[derive(Debug, Clone)]
struct ImapAccount {
    name: String,
    host: String,
    user: String,
    pass_cmd: Option<String>,
    pass: Option<String>,
}

fn parse_mbsyncrc() -> Vec<ImapAccount> {
    println!("[IDLE] 🔍 Début de la lecture de ~/.mbsyncrc...");
    let mut accounts = Vec::new();
    
    let home_dir = match dirs::home_dir() {
        Some(dir) => dir,
        None => {
            eprintln!("[IDLE] ❌ Impossible de trouver le dossier personnel.");
            return accounts;
        }
    };
    
    let mbsyncrc_path = home_dir.join(".mbsyncrc");

    let file = match File::open(&mbsyncrc_path) {
        Ok(f) => {
            println!("[IDLE] 📄 Fichier ~/.mbsyncrc ouvert avec succès.");
            f
        }
        Err(e) => {
            eprintln!("[IDLE] ❌ Fichier ~/.mbsyncrc introuvable ou illisible : {}", e);
            return accounts;
        }
    };

    let reader = BufReader::new(file);
    let mut current_account: Option<ImapAccount> = None;
    let mut is_imaps = false;

    for line in reader.lines().map_while(Result::ok) {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }

        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        if parts.len() < 2 { continue; }
        
        let key = parts[0];
        let val = parts[1].trim().trim_matches('"');

        match key {
            "IMAPAccount" => {
                if let Some(acc) = current_account.take() {
                    if is_imaps { 
                        println!("[IDLE] ✅ Compte IMAPS validé pour IDLE : {}", acc.name);
                        accounts.push(acc); 
                    } else {
                        println!("[IDLE] ⚠️ Compte ignoré (pas de SSLType IMAPS) : {}", acc.name);
                    }
                }
                current_account = Some(ImapAccount {
                    name: val.to_string(),
                    host: String::new(),
                    user: String::new(),
                    pass_cmd: None,
                    pass: None,
                });
                is_imaps = false;
            }
            "Host" => if let Some(ref mut acc) = current_account { acc.host = val.to_string(); },
            "User" => if let Some(ref mut acc) = current_account { acc.user = val.to_string(); },
            "PassCmd" => if let Some(ref mut acc) = current_account { acc.pass_cmd = Some(val.to_string()); },
            "Pass" => if let Some(ref mut acc) = current_account { acc.pass = Some(val.to_string()); },
            "SSLType" => if val == "IMAPS" { is_imaps = true; },
            _ => {}
        }
    }

    if let Some(acc) = current_account {
        if is_imaps { 
            println!("[IDLE] ✅ Compte IMAPS validé pour IDLE : {}", acc.name);
            accounts.push(acc); 
        }
    }

    println!("[IDLE] 🏁 Parsing terminé. {} compte(s) compatible(s) trouvé(s).", accounts.len());
    accounts
}

fn get_password(account: &ImapAccount) -> Result<String, String> {
    println!("[IDLE: {}] 🔑 Récupération du mot de passe...", account.name);
    
    if let Some(pass) = &account.pass {
        println!("[IDLE: {}] 🔑 Mot de passe trouvé en clair dans la config.", account.name);
        return Ok(pass.clone());
    }

    if let Some(pass_cmd) = &account.pass_cmd {
        println!("[IDLE: {}] 🔑 Exécution de PassCmd : {}", account.name, pass_cmd);
        let output = Command::new("sh")
            .arg("-c")
            .arg(pass_cmd)
            .output()
            .map_err(|e| format!("Erreur exécution shell: {}", e))?;

        if output.status.success() {
            let pass = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("[IDLE: {}] 🔑 PassCmd exécutée avec succès.", account.name);
            return Ok(pass);
        } else {
            let err_str = String::from_utf8_lossy(&output.stderr);
            return Err(format!("PassCmd a échoué avec le code {:?}. Stderr: {}", output.status.code(), err_str));
        }
    }

    Err("Aucune directive Pass ou PassCmd configurée.".to_string())
}

fn trigger_sync(account_name: &str, app_handle: &AppHandle) {
    println!("[IDLE: {}] ⚡ RÉVEIL ! Nouveau message détecté. Lancement de mbsync...", account_name);
    
    let sync_cmd = "sem --fg -j 1 --id mbsync '/usr/bin/mbsync -a' ; sem -j 1 --fg --id notmuch '/usr/bin/notmuch new --quiet'";
    
    let status = Command::new("sh")
        .arg("-c")
        .arg(sync_cmd)
        .status();

    match status {
        Ok(s) => {
            if s.success() {
                println!("[IDLE: {}] ✅ Synchronisation (mbsync + notmuch) terminée avec succès.", account_name);
                if let Err(e) = app_handle.emit("mail-synced", ()) {
                    eprintln!("[IDLE: {}] ❌ Erreur lors de l'émission de l'événement Vue.js: {}", account_name, e);
                } else {
                    println!("[IDLE: {}] 📡 Événement 'mail-synced' envoyé à l'interface.", account_name);
                }
            } else {
                eprintln!("[IDLE: {}] ❌ La commande de synchronisation a échoué avec le code: {:?}", account_name, s.code());
            }
        }
        Err(e) => eprintln!("[IDLE: {}] ❌ Impossible de lancer la commande shell de synchro: {}", account_name, e),
    }
}

fn start_idle_worker(account: ImapAccount, app_handle: AppHandle) {
    let acc_name = account.name.clone();
    println!("[IDLE: {}] 🚀 Démarrage du worker IDLE...", acc_name);

    loop {
        println!("[IDLE: {}] 🔄 Nouvelle boucle de connexion...", acc_name);

        let password = match get_password(&account) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("[IDLE: {}] ❌ Échec mot de passe: {}. Pause 60s...", acc_name, e);
                thread::sleep(Duration::from_secs(60));
                continue;
            }
        };

        println!("[IDLE: {}] 🌐 Connexion TLS à {}:993...", acc_name, account.host);
        let tls = native_tls::TlsConnector::builder().build().unwrap();
        let client = match imap::connect((account.host.as_str(), 993), &account.host, &tls) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("[IDLE: {}] ❌ Échec de la connexion TCP/TLS: {:?}. Pause 30s...", acc_name, e);
                thread::sleep(Duration::from_secs(30));
                continue;
            }
        };

        println!("[IDLE: {}] 🔐 Tentative de Login...", acc_name);
        let mut session = match client.login(&account.user, &password) {
            Ok(s) => s,
            Err((e, _unauth_client)) => {
                eprintln!("[IDLE: {}] ❌ Erreur de Login: {:?}. Pause 60s...", acc_name, e);
                thread::sleep(Duration::from_secs(60));
                continue;
            }
        };

        // --- NOUVEAU : Vérification des capacités du serveur ---
        println!("[IDLE: {}] 🛠️ Vérification du support IDLE par le serveur...", acc_name);
        match session.capabilities() {
            Ok(capabilities) => {
                if !capabilities.has_str("IDLE") {
                    eprintln!("[IDLE: {}] 🚨 ATTENTION : Ce serveur IMAP ne supporte PAS l'extension IDLE !", acc_name);
                    for  cap in capabilities.iter(){
                    eprintln!("[IDLE: {}] Capacités disponibles : {:?}", acc_name, cap);

                    }
                    // Si vous voyez ce message, vous ne POURREZ PAS utiliser IDLE pour ce compte.
                    // Il faudra utiliser un système de "polling" (vérification toutes les X minutes).
                } else {
                    println!("[IDLE: {}] ✅ Le serveur supporte IDLE.", acc_name);
                }
            }
            Err(e) => eprintln!("[IDLE: {}] ⚠️ Impossible de lire les capacités du serveur: {:?}", acc_name, e),
        }

        println!("[IDLE: {}] 📂 Sélection de la boîte INBOX...", acc_name);
        if let Err(e) = session.select("INBOX") {
            eprintln!("[IDLE: {}] ❌ Erreur select INBOX: {:?}. Pause 30s...", acc_name, e);
            let _ = session.logout();
            thread::sleep(Duration::from_secs(30));
            continue;
        }

        println!("[IDLE: {}] 🎧 Passage en mode IDLE (Écoute continue)...", acc_name);
        loop {
            // C'est ici que ça bloquait ou échouait pour vous.
            match session.idle() {
                Ok(mut idle) => {
                    println!("[IDLE: {}] ⏳ En attente de notifications du serveur...", acc_name);
                    
                    match idle.wait_keepalive() {
                        Ok(_) => {
                            println!("[IDLE: {}] 🔔 Sortie de wait_keepalive() ! (Message, changement ou KeepAlive 29min)", acc_name);
                            trigger_sync(&acc_name, &app_handle);
                        }
                        Err(e) => {
                            eprintln!("[IDLE: {}] ❌ Erreur/Déconnexion pendant wait_keepalive: {:?}", acc_name, e);
                            break; 
                        }
                    }
                }
                Err(e) => {
                    // L'erreur exacte sera affichée ici grâce au `{:?}`
                    eprintln!("[IDLE: {}] ❌ Impossible de passer en mode IDLE: {:?}", acc_name, e);
                    break;
                }
            }
        }

        println!("[IDLE: {}] 🔌 Déconnexion propre de la session...", acc_name);
        let _ = session.logout();
        println!("[IDLE: {}] 💤 Pause 10s avant de relancer le cycle de reconnexion...", acc_name);
        thread::sleep(Duration::from_secs(10));
    }
}
pub fn start_imap_idle_daemons(app_handle: AppHandle) {
    let accounts = parse_mbsyncrc();
    println!("[IDLE-MANAGER] Démarrage des threads pour {} compte(s).", accounts.len());

    for account in accounts {
        let handle_clone = app_handle.clone();
        thread::spawn(move || {
            start_idle_worker(account, handle_clone);
        });
    }
}