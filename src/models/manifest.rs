use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectManifest {
    pub project_version: String,
    pub sentinel_version: String,
    pub export_timestamp: String,
    pub methodology: String,
    pub assessment_id: String,
    pub assessment_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectBundle {
    pub manifest: ProjectManifest,
    pub assessment: Option<crate::models::assessment::Assessment>,
    pub sections: Vec<crate::models::workflow::Section>,
    pub activities: Vec<crate::models::workflow::Activity>,
    pub tasks: Vec<crate::models::workflow::Task>,
    pub evidence: Vec<crate::models::evidence::Evidence>,
    pub findings: Vec<crate::models::finding::Finding>,
    pub finding_evidence: Vec<FindingEvidenceLink>,
    pub http_requests: Vec<crate::models::http::HttpRequest>,
    pub http_responses: Vec<crate::models::http::HttpResponse>,
    pub http_transactions: Vec<crate::models::http::HttpTransaction>,
    pub session: Option<SessionData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindingEvidenceLink {
    pub finding_id: String,
    pub evidence_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    pub assessment_id: String,
    pub current_screen: String,
    pub last_opened_at: String,
}
