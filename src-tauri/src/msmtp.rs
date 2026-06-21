use serde::{Deserialize};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use lettre::message::{Attachment, MultiPart, SinglePart, Message};



// --- RAW NOTMUCH MODELS ---

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentPayload {
    pub path: String,          // Chemin absolu vers le fichier
    pub mime_type: Option<String>, // Optionnel: ex "application/pdf"
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailPayload {
    pub from: String,          // Adresse d'expédition
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub bcc: Vec<String>,
    pub subject: String,
    pub body: String,
    pub is_html: bool,
    pub attachments: Vec<AttachmentPayload>,
    pub account: Option<String>, // Optionnel: pour passer `-a <compte>` à msmtp
}



pub struct MSMTPWrapper;

impl MSMTPWrapper {
    pub fn check_installation() -> bool {
        Command::new("msmtp").arg("--version").output().is_ok()
    }


pub async  fn send_email(payload: EmailPayload) -> Result<(), String> {
    // 1. Initialisation du constructeur de mail
    let mut builder = Message::builder()
        .from(payload.from.parse().map_err(|e| format!("'From' invalide: {}", e))?)
        .subject(payload.subject);

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
        let content = fs::read(path).map_err(|e| format!("Erreur de lecture de la PJ ({}): {}", file_name, e))?;

        let mime_type = att.mime_type
            .unwrap_or_else(|| "application/octet-stream".to_string())
            .parse()
            .map_err(|_| format!("Type MIME invalide pour {}", file_name))?;

        let attachment = Attachment::new(file_name).body(content, mime_type);
        multipart = multipart.singlepart(attachment);
    }

    // 4. Génération du message MIME brut
    let email = builder
        .multipart(multipart)
        .map_err(|e| format!("Erreur de construction du mail: {}", e))?;

    let email_bytes = email.formatted();

    // 5. Exécution de msmtp
    let mut cmd = Command::new("msmtp");
    cmd.arg("-t"); // Demande à msmtp d'extraire les destinataires (To, Cc, Bcc) des headers MIME

    if let Some(account) = payload.account {
        println!("Using msmtp account: {}", account);
        cmd.arg("-a").arg(account);
    }

    let mut child = cmd
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Impossible de lancer msmtp: {}", e))?;

    // Écriture du mail généré dans l'entrée standard de msmtp
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(&email_bytes)
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

    Ok(())
}

}

