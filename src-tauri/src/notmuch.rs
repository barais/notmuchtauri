use serde::{Deserialize, Serialize};
use std::process::Command;
use std::error::Error;

// --- RAW NOTMUCH MODELS ---

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
        Command::new("notmuch")
            .arg("--version")
            .output()
            .is_ok()
    }

    pub fn search(query: &str, limit: Option<u32>, sort: Option<&str>) -> Result<Messag, Box<dyn Error>> {
        let mut cmd = Command::new("notmuch");
        cmd.arg("search")
           .arg("--format=json");

                   if let Some(l) = limit {
            cmd.arg("--limit").arg(l.to_string());
        }

        if let Some(s) = sort {
            cmd.arg("--sort").arg(s);
        }

        
          cmd .arg(query);
          println!("Executing command: {:?}", query);


        let output = cmd.output()?;
        
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
        let body_output = Command::new("notmuch")
            .arg("show")
            .arg(id)
            .output()?;

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
}
