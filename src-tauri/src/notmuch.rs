use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::fs;
use std::process::Command;

// --- RAW NOTMUCH MODELS ---

/// Representation of a single result from `notmuch show --format=json`.

pub type NotmuchOutputShow = Vec<Vec<ThreadNode>>;

// --- 1. Les DTOs pour le Frontend (Tauri -> Vue/TS) ---

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")] // Automatiquement camelCase pour TypeScript
pub struct ThreadDto {
    pub roots: Vec<MessageDto>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageDto {
    pub id: String,
    pub subject: String,
    pub from: String,
    pub to: String,
    pub cc: String,
    pub date: String,
    pub timestamp: u64,
    pub tags: Vec<String>, 

    // Le contenu
    pub html_body: Option<String>,
    pub text_body: Option<String>, // Utilisé comme fallback si html_body est None

    // Les pièces jointes et images
    pub inline_images: Vec<AttachmentDto>,
    pub attachments: Vec<AttachmentDto>,

    // La hiérarchie (Récursion)
    pub replies: Vec<MessageDto>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentDto {
    pub part_id: u32, // Le numéro du part (pour requêter "notmuch show --part=X")
    pub filename: String,
    pub content_type: String,
    pub content_id: Option<String>, // Crucial pour faire correspondre <img src="cid:XXX" />
}

/// Le point magique : un Tuple Struct !
/// Serde va automatiquement désérialiser un tableau JSON `[ {message}, [réponses] ]` dans cette structure.
#[derive(Debug, Deserialize, Serialize)]
pub struct ThreadNode(pub MessageInternal, pub Vec<ThreadNode>);

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageInternal {
    pub id: String,

    // "match" est un mot-clé réservé en Rust, on le renomme.
    #[serde(rename = "match")]
    pub is_match: bool,

    pub excluded: bool,

    #[serde(default)]
    pub filename: Vec<String>,

    pub timestamp: u64,
    pub date_relative: String,

    #[serde(default)]
    pub tags: Vec<String>,

    pub duplicate: u32,

    pub headers: HeadersInternal,

    // C'est ici que notmuch met les parts du mail (si vous utilisez --body=true).
    // On le capture sous forme de Value générique (ou de tableau de Value)
    // pour pouvoir facilement fouiller dedans ensuite pour chercher les content-type.
    #[serde(default)]
    pub body: Option<Vec<Value>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HeadersInternal {
    #[serde(rename = "Subject")]
    pub subject: Option<String>,
    #[serde(rename = "From")]
    pub from: Option<String>,
    #[serde(rename = "To")]
    pub to: Option<String>,
    #[serde(rename = "Cc")]
    pub cc: Option<String>,
    #[serde(rename = "Reply-To")]
    pub reply_to: Option<String>,
    #[serde(rename = "Date")]
    pub date: Option<String>,
}

/// Representation of a single result from `notmuch search --format=json`.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessagElement {
    pub thread: String,
    pub timestamp: i64,
    pub date_relative: String,
    pub matched: i64,
    pub total: i64,
    pub authors: String,
    pub subject: String,
    pub query: Vec<Option<String>>,
    pub tags: Vec<String>,
}

pub type Messag = Vec<MessagElement>;

/// Clean representation for the Frontend as defined in docs/arch/data-model.md.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: String,
    pub subject: String,
    pub from: String,
    pub to: String,
    pub date: String,
    pub body: String,
    pub tags: Vec<String>,
    pub is_read: bool,
    pub has_attachments: bool,
}

pub struct NotMuchWrapper;

impl NotMuchWrapper {
    pub fn check_installation() -> bool {
        Command::new("notmuch").arg("--version").output().is_ok()
    }

    pub fn search(
        query: &str,
        limit: Option<u32>,
        sort: Option<&str>,
    ) -> Result<Messag, Box<dyn Error>> {
        let mut cmd = Command::new("notmuch");
        cmd.arg("search").arg("--format=json");

        if let Some(l) = limit {
            cmd.arg("--limit").arg(l.to_string());
        }

        if let Some(s) = sort {
            cmd.arg("--sort").arg(s);
        }

        cmd.arg(query);
        println!("Executing command: {:?}", query);

        let output: std::process::Output = cmd.output()?;

        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            return Err(format!("notmuch search failed: {}", err).into());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.trim().is_empty() {
            return Ok(vec![]);
        }

        let messages: Messag = serde_json::from_str(&stdout)?;
        Ok(messages)
    }

    pub fn get_message_details(id: &str) -> Result<Message, Box<dyn Error>> {
        // 1. Get the raw body using 'notmuch show'
        let body_output = Command::new("notmuch").arg("show").arg(id).output()?;

        if !body_output.status.success() {
            return Err("notmuch show failed: message not found".into());
        }

        let body_content = String::from_utf8_lossy(&body_output.stdout).to_string();

        // 2. Get metadata via a targeted search (format=json)
        let info_output = Command::new("notmuch")
            .arg("search")
            .arg("--format=json")
            .arg(format!("thread:{}", id))
            .output()?;

        if !info_output.status.success() {
            return Err("notmuch metadata retrieval failed".into());
        }

        let info_stdout = String::from_utf8_lossy(&info_output.stdout);
        let raw_msgs: Messag = serde_json::from_str(&info_stdout)?;
        let msg_info = raw_msgs.get(0).ok_or("Message metadata not found")?;

        // 3. Map to our clean Message struct
        Ok(Message {
            id: msg_info.thread.clone(),
            subject: msg_info.subject.clone(),
            from: msg_info.authors.clone(),
            to: "Unknown".to_string(),
            date: msg_info.date_relative.clone(),
            body: body_content,
            tags: msg_info.tags.clone(),
            is_read: false,
            has_attachments: false,
        })
    }

    pub fn get_thread_details(thread_id: &str) -> Result<Vec<ThreadDto>, Box<dyn Error>> {
        let mut cmd = Command::new("notmuch");

        cmd.arg("show")
            .arg("--format=json")
            .arg("--sort=newest-first")
            .arg("--include-html")
            .arg(format!("thread:{}", thread_id));

        let output = cmd.output();
        if (output.is_err()) {
            return Err("notmuch show failed to fetch thread".into());
        }
        let out = output.unwrap();

        let stdout = String::from_utf8_lossy(&out.stdout);
        parse_to_dtos(&stdout)
    }

    pub fn get_message_part(message_id: &str, part_id: u32) -> Result<String, String> {
        // Appel sécurisé à notmuch (pas de risque d'injection shell car on passe un tableau d'arguments)
        let output = Command::new("notmuch")
            .args([
                "show",
                "--format=raw",
                &format!("--part={}", part_id),
                &format!("id:{}", message_id),
            ])
            .output()
            .map_err(|e| format!("Erreur d'exécution notmuch: {}", e))?;

        if output.status.success() {
            // Encodage du binaire brut en chaîne Base64
            let b64 = general_purpose::STANDARD.encode(&output.stdout);
            Ok(b64)
        } else {
            let err = String::from_utf8_lossy(&output.stderr).into_owned();
            Err(format!("Erreur notmuch: {}", err))
        }
    }

    pub fn save_message_part(
        message_id: &str,
        part_id: u32,
        output_path: &str,
    ) -> Result<(), String> {
        // On extrait le binaire brut de la pièce jointe
        let output = Command::new("notmuch")
            .args([
                "show",
                "--format=raw",
                &format!("--part={}", part_id),
                &format!("id:{}", message_id),
            ])
            .output()
            .map_err(|e| format!("Erreur lors de l'appel à notmuch: {}", e))?;

        if output.status.success() {
            // On écrit directement les octets sur le disque
            fs::write(output_path, &output.stdout)
                .map_err(|e| format!("Erreur d'écriture du fichier sur le disque: {}", e))?;
            Ok(())
        } else {
            let err = String::from_utf8_lossy(&output.stderr).into_owned();
            Err(format!("Erreur notmuch: {}", err))
        }
    }

pub fn modify_message_tag(message_id: &str, tag: &str, action: &str) -> Result<(), String> {
    // action doit être "add" ou "remove"
    let prefix = match action {
        "add" => "+",
        "remove" => "-",
        _ => return Err("Action invalide, utilisez 'add' ou 'remove'".to_string()),
    };

    let tag_arg = format!("{}{}", prefix, tag);
    let id_arg = format!("id:{}", message_id);

    // On exécute: notmuch tag +tag_name -- id:message_id
    // Le "--" sécurise la commande au cas où le tag ou l'ID commence par un tiret
    let output = Command::new("notmuch")
        .args(["tag", &tag_arg, "--", &id_arg])
        .output()
        .map_err(|e| format!("Erreur d'exécution notmuch: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        let err = String::from_utf8_lossy(&output.stderr).into_owned();
        Err(format!("Erreur notmuch: {}", err))
    }
}
}

/// Convertit le JSON brut de Notmuch en une liste de ThreadDto prêts pour Tauri/Vue
pub fn parse_to_dtos(raw_json: &str) -> Result<Vec<ThreadDto>, Box<dyn Error>> {
    let threads: NotmuchOutputShow = serde_json::from_str(raw_json)
        .map_err(|e| format!("Erreur parsing JSON notmuch : {}", e))?;

    let mut result = Vec::new();
    for thread_roots in threads {
        let mut root_dtos = Vec::new();
        for root_node in thread_roots {
            root_dtos.push(convert_node_to_dto(&root_node));
        }
        result.push(ThreadDto { roots: root_dtos });
    }

    Ok(result)
}

/// Convertit un nœud récursif interne en DTO
fn convert_node_to_dto(node: &ThreadNode) -> MessageDto {
    let msg = &node.0;
    let replies = &node.1;

    let mut html_bodies = Vec::new();
    let mut text_bodies = Vec::new();
    let mut inline_images = Vec::new();
    let mut attachments = Vec::new();

    // S'il y a un corps de message, on l'analyse
    if let Some(body_parts) = &msg.body {
        for part in body_parts {
            extract_parts(
                part,
                &mut html_bodies,
                &mut text_bodies,
                &mut inline_images,
                &mut attachments,
            );
        }
    }

    // Appel récursif pour les réponses
    let replies_dto = replies.iter().map(convert_node_to_dto).collect();

    MessageDto {
        id: msg.id.clone(),
        subject: msg.headers.subject.clone().unwrap_or_default(),
        from: msg.headers.from.clone().unwrap_or_default(),
        to: msg.headers.to.clone().unwrap_or_default(),
        cc: msg.headers.cc.clone().unwrap_or_default(),
        date: msg.headers.date.clone().unwrap_or_default(),
        timestamp: msg.timestamp,
        tags: msg.tags.clone(), // msg est le ThreadNode.0 (MessageInternal)

        // Si plusieurs parts HTML (ex: mails transférés), on les fusionne. Sinon Some/None
        html_body: if html_bodies.is_empty() {
            None
        } else {
            Some(html_bodies.join("<hr>"))
        },
        text_body: if text_bodies.is_empty() {
            None
        } else {
            Some(text_bodies.join("\n\n---\n\n"))
        },

        inline_images,
        attachments,
        replies: replies_dto,
    }
}

/// Extrait de façon récursive les parts et les dispatche dans les bons vecteurs
fn extract_parts(
    part: &Value,
    html_bodies: &mut Vec<String>,
    text_bodies: &mut Vec<String>,
    inline_images: &mut Vec<AttachmentDto>,
    attachments: &mut Vec<AttachmentDto>,
) {
    let content_type = part
        .get("content-type")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let part_id = part.get("id").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
    let filename = part.get("filename").and_then(|v| v.as_str());
    let content_id = part.get("content-id").and_then(|v| v.as_str());
    let disposition = part
        .get("content-disposition")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    // 1. Est-ce un texte ?
    if content_type == "text/html" {
        if let Some(content) = part.get("content").and_then(|v| v.as_str()) {
            html_bodies.push(content.to_string());
        }
    } else if content_type == "text/plain" {
        // On ignore les text/plain qui sont des pièces jointes explicites
        if filename.is_none() && disposition != "attachment" {
            if let Some(content) = part.get("content").and_then(|v| v.as_str()) {
                text_bodies.push(content.to_string());
            }
        }
    }

    // 2. Est-ce une image "inline" ? (Possède un content-id et est une image)
    if content_id.is_some() && content_type.starts_with("image/") {
        inline_images.push(AttachmentDto {
            part_id,
            filename: filename.unwrap_or("image").to_string(),
            content_type: content_type.to_string(),
            content_id: content_id.map(|s| s.to_string()),
        });
    }
    // 3. Est-ce une vraie pièce jointe ? (Possède un nom de fichier ou une disposition attachment)
    else if filename.is_some() || disposition == "attachment" {
        attachments.push(AttachmentDto {
            part_id,
            filename: filename.unwrap_or("fichier_sans_nom").to_string(),
            content_type: content_type.to_string(),
            content_id: content_id.map(|s| s.to_string()),
        });
    }

    // 4. Récursion pour plonger dans les multipart (alternative, mixed, related...)
    if let Some(sub_parts) = part.get("content").and_then(|v| v.as_array()) {
        for sub_part in sub_parts {
            extract_parts(
                sub_part,
                html_bodies,
                text_bodies,
                inline_images,
                attachments,
            );
        }
    }
}

/// Parcours récursif de l'arbre des messages
fn process_node(node: &ThreadNode, depth: usize) {
    let indent = "  ".repeat(depth);
    let message = &node.0; // Le { message }
    let replies = &node.1; // Le tableau des réponses [ ... ]

    let subject = message.headers.subject.as_deref().unwrap_or("Sans objet");
    let from = message.headers.from.as_deref().unwrap_or("Inconnu");

    println!("{}-> {} (De : {})", indent, subject, from);

    // Si on a des parts (body), on les analyse
    if let Some(body_parts) = &message.body {
        let extracted_texts = find_text_parts(body_parts);
        for text in extracted_texts {
            println!("{}  [+] Part trouvé : {}", indent, text["content-type"]);
            // Vous avez accès au contenu via text["content"]
        }
    }

    // Appel récursif pour parcourir les enfants du thread
    for reply in replies {
        process_node(reply, depth + 1);
    }
}

/// Cherche récursivement dans les `parts` du corps du message
pub fn find_text_parts(parts: &[Value]) -> Vec<&Value> {
    let mut found = Vec::new();

    for part in parts {
        if let Some(content_type) = part.get("content-type").and_then(|v| v.as_str()) {
            if content_type == "text/plain" || content_type == "text/html" {
                found.push(part);
            }
        }

        // Si ce part contient d'autres sous-parts (ex: multipart/alternative)
        if let Some(sub_parts) = part.get("content").and_then(|v| v.as_array()) {
            found.extend(find_text_parts(sub_parts));
        }
    }

    found
}
