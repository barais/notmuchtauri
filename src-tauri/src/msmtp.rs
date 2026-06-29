use lettre::message::{Attachment, Message, MultiPart, SinglePart};
use serde::Deserialize;
use tokio::time::interval;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;

// --- RAW NOTMUCH MODELS ---

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentPayload {
    pub path: String,              // Chemin absolu vers le fichier
    pub mime_type: Option<String>, // Optionnel: ex "application/pdf"
    pub is_part: bool,
    pub part_id: i16,
    pub message_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailPayload {
    pub from: String, // Adresse d'expédition
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub bcc: Vec<String>,
    pub subject: String,
    pub body: String,
    pub is_html: bool,
    pub attachments: Vec<AttachmentPayload>,
    pub account: Option<String>, // Optionnel: pour passer `-a <compte>` à msmtp
    pub sent_folder: String,
}

pub struct MSMTPWrapper;

impl MSMTPWrapper {
    pub fn check_installation() -> bool {
        Command::new("msmtp").arg("--version").output().is_ok()
    }

    /// Tente d'envoyer un e-mail brut via msmtp. Retourne true si succès.
    fn send_raw_email(raw_email: &[u8], account:Option<String>) -> bool {
         let mut cmd =  Command::new("msmtp");
        cmd .arg("-t");

                if let Some(account) = account {
            println!("Using msmtp account: {}", account);
            cmd.arg("-a").arg(account);
        }

        let mut child = match cmd // Demande à msmtp de lire le 'To:' et 'From:' dans les headers
            .stdin(Stdio::piped())
            .spawn()
        {
            Ok(c) => c,
            Err(_) => return false,
        };

        if let Some(mut stdin) = child.stdin.take() {
            if stdin.write_all(raw_email).is_err() {
                return false;
            }
        }

        match child.wait_with_output() {
            Ok(out) => out.status.success(),
            Err(_) => false,
        }
    }

    /// Sauvegarde un e-mail brut dans un dossier spécifique via notmuch
    fn insert_to_notmuch(raw_email: &[u8], folder: &str, extra_tags: &[&str]) -> bool {
        let mut args = vec![
            "insert".to_string(),
            format!("--folder={}", folder),
            "--create-folder".to_string(),
        ];
        for tag in extra_tags {
            args.push(tag.to_string());
        }

        let mut child = match Command::new("notmuch")
            .args(&args)
            .stdin(Stdio::piped())
            .spawn()
        {
            Ok(c) => c,
            Err(_) => return false,
        };

        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(raw_email);
        }

        child.wait().map(|s| s.success()).unwrap_or(false)
    }
    pub async fn send_email(payload: EmailPayload) -> Result<(), String> {
        // 1. Initialisation du constructeur de mail
        let mut builder = Message::builder().subject(payload.subject);

        if payload.from != "" {
            builder = builder.from(
                payload
                    .from
                    .parse()
                    .map_err(|e| format!("'From' invalide: {}", e))?,
            )
        }

        for addr in payload.to {
            builder = builder.to(addr.parse().map_err(|e| format!("'To' invalide: {}", e))?);
        }
        for addr in payload.cc {
            builder = builder.cc(addr.parse().map_err(|e| format!("'Cc' invalide: {}", e))?);
        }
        for addr in payload.bcc {
            builder = builder.bcc(addr.parse().map_err(|e| format!("'Bcc' invalide: {}", e))?);
        }

        // 2. Construction du corps (HTML ou texte brut)
        let body_part = if payload.is_html {
            SinglePart::html(payload.body)
        } else {
            SinglePart::plain(payload.body)
        };

        let mut multipart = MultiPart::mixed().singlepart(body_part);

        // 3. Ajout des pièces jointes
        for att in payload.attachments {
            let path = Path::new(&att.path);
            let file_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("fichier_joint")
                .to_string();

            // On utilise std::fs directement en Rust. Cela contourne les restrictions
            // de sécurité du frontend (Tauri fs scope), ce qui est idéal ici.
            if !att.is_part {
                let content = fs::read(path)
                    .map_err(|e| format!("Erreur de lecture de la PJ ({}): {}", file_name, e))?;

                let mime_type = att
                    .mime_type
                    .unwrap_or_else(|| "application/octet-stream".to_string())
                    .parse()
                    .map_err(|_| format!("Type MIME invalide pour {}", file_name))?;

                let attachment = Attachment::new(file_name).body(content, mime_type);
                multipart = multipart.singlepart(attachment);
            } else {
                let output = Command::new("notmuch")
                    .args([
                        "show",
                        "--format=raw",
                        &format!("--part={}", att.part_id),
                        &format!("id:{}", att.message_id),
                    ])
                    .output()
                    .map_err(|e| format!("Erreur lors de l'appel à notmuch: {}", e))?;

                if output.status.success() {
                    let content = output.stdout;
                    let mime_type = att
                        .mime_type
                        .unwrap_or_else(|| "application/octet-stream".to_string())
                        .parse()
                        .map_err(|_| format!("Type MIME invalide pour {}", file_name))?;

                    let attachment = Attachment::new(file_name).body(content, mime_type);
                    multipart = multipart.singlepart(attachment);
                }
            }
        }

        // 4. Génération du message MIME brut
        let email = builder
            .multipart(multipart)
            .map_err(|e| format!("Erreur de construction du mail: {}", e))?;

        let email_bytes = email.formatted();

        // 5. Exécution de msmtp
//        let mut cmd = Command::new("msmtp");
//        cmd.arg("-t"); // Demande à msmtp d'extraire les destinataires (To, Cc, Bcc) des headers MIME

/*         if let Some(account) = payload.account {
            println!("Using msmtp account: {}", account);
            cmd.arg("-a").arg(account);
        }*/

   /*     let mut child = cmd
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Impossible de lancer msmtp: {}", e))?; */

        // Écriture du mail généré dans l'entrée standard de msmtp
/*         if let Some(mut stdin) = child.stdin.take() {
            stdin
                .write_all(&email_bytes)
                .map_err(|e| format!("Erreur d'écriture dans stdin: {}", e))?;
        }

        // Attente du résultat
        let output = child
            .wait_with_output()
            .map_err(|e| format!("Erreur d'attente de msmtp: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Erreur msmtp (code {}): {}", output.status, stderr));
        }

        // 3. Sauvegarde dans Notmuch
        // On récupère le contenu brut de l'e-mail sous forme de Vec<u8> (les octets exacts envoyés)
        let raw_email_bytes = email.formatted();

        println!("Sauvegarde dans Notmuch...");
        MSMTPWrapper::save_to_notmuch(&raw_email_bytes, payload.sent_folder)?;
        println!("Sauvegarde réussie !");
        */
    let sent_successfully = MSMTPWrapper::send_raw_email(&email_bytes,payload.account);

    if sent_successfully {
        // 2a. Succès : Sauvegarde dans Sent
        MSMTPWrapper::insert_to_notmuch(&email_bytes, &payload.sent_folder, &["+sent", "-unread", "-new"]);
        
       println!("Email envoyé avec succès");
    } else {
        // 2b. Échec (pas de réseau) : Sauvegarde dans Outbox
        MSMTPWrapper::insert_to_notmuch(&email_bytes, "Outbox", &["+outbox", "-unread", "-new"]);
        // On retourne quand même "Ok" au frontend pour ne pas bloquer l'UI, 
        // mais on prévient que c'est dans la boîte d'envoi.
        println!("Réseau indisponible. L'e-mail a été placé dans la boîte d'envoi (Outbox).")
    }

        Ok(())
    }

    pub async fn send_ics_email(
        to_addresses: Vec<String>,
        subject: &str,
        body: &str,
        ics_content: &str,
        sentfolder: &str,
    ) -> Result<(), String> {
        // 1. Démarrer la construction de l'e-mail
        // Pensez à remplacer "votre_email" par la lecture de l'adresse de l'utilisateur
        let mut builder = Message::builder()
            .from("votre_email@domaine.com".parse().unwrap())
            .subject(subject);

        // Ajouter tous les destinataires (lettre gère "Nom <email@domaine.com>" ou juste "email@domaine.com")
        for addr in to_addresses {
            let parsed_addr = addr
                .parse()
                .map_err(|_| format!("Adresse invalide: {}", addr))?;
            builder = builder.to(parsed_addr);
        }

        // 2. Créer l'e-mail Multipart (Texte + Fichier ICS)
        // Le Content-Type text/calendar; method=REQUEST permet aux clients mail (Outlook/Gmail)
        // d'afficher directement les boutons "Accepter" ou "Refuser".
        let email = builder
            .multipart(
                MultiPart::mixed()
                    .singlepart(SinglePart::plain(body.to_string()))
                    .singlepart(Attachment::new(String::from("invite.ics")).body(
                        ics_content.to_string(),
                        "text/calendar; method=REQUEST".parse().unwrap(),
                    )),
            )
            .map_err(|e| format!("Erreur de construction de l'email: {}", e))?;

        // 3. Envoi via msmtp

        // 5. Exécution de msmtp
       /* let mut cmd = Command::new("msmtp");
        cmd.arg("-t"); // Demande à msmtp d'extraire les destinataires (To, Cc, Bcc) des headers MIME

        let mut child = cmd
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Impossible de lancer msmtp: {}", e))?;
         */

        let email_bytes = email.formatted();
        let sent_successfully = MSMTPWrapper::send_raw_email(&email_bytes, None);
    if sent_successfully {
        // 2a. Succès : Sauvegarde dans Sent
        MSMTPWrapper::insert_to_notmuch(&email_bytes, sentfolder, &["+sent", "-unread", "-new"]);
        
       println!("Email envoyé avec succès");
    } else {
        // 2b. Échec (pas de réseau) : Sauvegarde dans Outbox
        MSMTPWrapper::insert_to_notmuch(&email_bytes, "Outbox", &["+outbox", "-unread", "-new"]);
        // On retourne quand même "Ok" au frontend pour ne pas bloquer l'UI, 
        // mais on prévient que c'est dans la boîte d'envoi.
        println!("Réseau indisponible. L'e-mail a été placé dans la boîte d'envoi (Outbox).")
    }


        // Écriture du mail généré dans l'entrée standard de msmtp
/*         if let Some(mut stdin) = child.stdin.take() {
            stdin
                .write_all(&email_bytes)
                .map_err(|e| format!("Erreur d'écriture dans stdin: {}", e))?;
        }

        // Attente du résultat
        let output = child
            .wait_with_output()
            .map_err(|e| format!("Erreur d'attente de msmtp: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Erreur msmtp (code {}): {}", output.status, stderr));
        }

        // 3. Sauvegarde dans Notmuch
        // On récupère le contenu brut de l'e-mail sous forme de Vec<u8> (les octets exacts envoyés)
        let raw_email_bytes = email.formatted();

        println!("Sauvegarde dans Notmuch...");
        MSMTPWrapper::save_to_notmuch(&raw_email_bytes, sentfolder.to_string())?;
        println!("Sauvegarde réussie !");
*/
        Ok(())
    }

    /// Fonction utilitaire pour injecter l'e-mail brut dans Notmuch
  /*   fn save_to_notmuch(raw_email: &[u8], folder: String) -> Result<(), String> {
        // On configure la commande `notmuch insert`
        let mut child = Command::new("notmuch")
            .args([
                "insert",
                &format!("--folder={}", folder),
                "--create-folder",
                "-unread", // On retire le tag unread
                "-new",    // On retire le tag new
                "+sent",   // (Optionnel) on ajoute un tag 'sent' pour le retrouver facilement
            ])
            // On indique qu'on va écrire dans l'entrée standard de la commande (stdin)
            .stdin(Stdio::piped())
            // On redirige la sortie d'erreur au cas où pour le débogage
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Erreur lors du lancement de notmuch: {}", e))?;

        // On écrit les octets de l'e-mail brut dans le stdin du processus notmuch
        if let Some(mut stdin) = child.stdin.take() {
            stdin
                .write_all(raw_email)
                .map_err(|e| format!("Erreur d'écriture dans stdin: {}", e))?;
        }

        // On attend que notmuch termine son travail
        let output = child
            .wait_with_output()
            .map_err(|e| format!("Erreur lors de l'attente du processus: {}", e))?;

        if output.status.success() {
            Ok(())
        } else {
            let err_msg = String::from_utf8_lossy(&output.stderr);
            Err(format!(
                "notmuch a échoué avec le code {:?}. Erreur : {}",
                output.status.code(),
                err_msg
            ))
        }
    }*/

    pub async fn process_outbox_daemon() {
    // Vérifie la boîte d'envoi toutes les 5 minutes
    let mut ticker = interval(Duration::from_secs(5 * 60));

    loop {
        ticker.tick().await;
        // println!("Vérification de la boîte d'envoi (Outbox)...");

        // 1. Chercher les IDs des messages ayant le tag outbox
        let search_output = match Command::new("notmuch")
            .args(["search", "--output=messages", "tag:outbox"])
            .output() 
        {
            Ok(out) => out,
            Err(_) => continue,
        };

        let msgs_str = String::from_utf8_lossy(&search_output.stdout);
        
        for msg_id in msgs_str.lines() {
            let msg_id = msg_id.trim();
            if msg_id.is_empty() { continue; }

            // 2. Extraire le contenu brut du message
            let raw_output = Command::new("notmuch")
                .args(["show", "--format=raw", msg_id])
                .output()
                .unwrap_or_else(|_| std::process::Output {
                    status: Default::default(),
                    stdout: Vec::new(),
                    stderr: Vec::new()
                });

            if raw_output.stdout.is_empty() { continue; }

            // 3. Tenter l'envoi
            if MSMTPWrapper::send_raw_email(&raw_output.stdout,None) {
                // SUCCESS ! L'email est parti.
                // println!("Email {} envoyé avec succès depuis l'Outbox !", msg_id);

                // 4. On récupère le(s) chemin(s) physique(s) du fichier dans Outbox
                if let Ok(files_out) = Command::new("notmuch")
                    .args(["search", "--output=files", msg_id])
                    .output() 
                {
                    // 5. On l'insère physiquement dans "Sent"
                    MSMTPWrapper::insert_to_notmuch(&raw_output.stdout, "Sent", &["+sent", "-unread", "-outbox"]);

                    // 6. On supprime les vieux fichiers de Outbox
                    let files_str = String::from_utf8_lossy(&files_out.stdout);
                    for file_path in files_str.lines() {
                        let _ = fs::remove_file(file_path.trim());
                    }

                    // 7. On informe notmuch que la base a changé
                    let _ = Command::new("notmuch").arg("new").output();
                }
            }
        }
    }
}
}


