use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceType {
    #[serde(rename = "Note")]
    Note,
    #[serde(rename = "HTTP Request")]
    HttpRequest,
    #[serde(rename = "HTTP Response")]
    HttpResponse,
    #[serde(rename = "Screenshot")]
    Screenshot,
    #[serde(rename = "File Attachment")]
    FileAttachment,
    #[serde(rename = "URL")]
    Url,
    #[serde(rename = "Command Output")]
    CommandOutput,
}

impl EvidenceType {
    pub const ALL: &'static [EvidenceType] = &[
        Self::Note,
        Self::HttpRequest,
        Self::HttpResponse,
        Self::Screenshot,
        Self::FileAttachment,
        Self::Url,
        Self::CommandOutput,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            Self::Note => "Note",
            Self::HttpRequest => "HTTP Request",
            Self::HttpResponse => "HTTP Response",
            Self::Screenshot => "Screenshot",
            Self::FileAttachment => "File Attachment",
            Self::Url => "URL",
            Self::CommandOutput => "Command Output",
        }
    }

    pub fn from_label(label: &str) -> Option<Self> {
        match label {
            "Note" => Some(Self::Note),
            "HTTP Request" => Some(Self::HttpRequest),
            "HTTP Response" => Some(Self::HttpResponse),
            "Screenshot" => Some(Self::Screenshot),
            "File Attachment" => Some(Self::FileAttachment),
            "URL" => Some(Self::Url),
            "Command Output" => Some(Self::CommandOutput),
            _ => None,
        }
    }

    pub fn next(&self) -> Self {
        let all = Self::ALL;
        let pos = all.iter().position(|e| e == self).unwrap_or(0);
        all[(pos + 1) % all.len()]
    }

    pub fn prev(&self) -> Self {
        let all = Self::ALL;
        let pos = all.iter().position(|e| e == self).unwrap_or(0);
        all[(pos + all.len() - 1) % all.len()]
    }
}

impl std::fmt::Display for EvidenceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub id: Uuid,
    pub assessment_id: Uuid,
    pub task_id: Uuid,
    pub evidence_type: EvidenceType,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
