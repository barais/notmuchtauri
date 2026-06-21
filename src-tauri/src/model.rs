use serde::Deserialize;

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

pub mod model;