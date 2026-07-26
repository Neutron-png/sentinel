#![allow(dead_code)]

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct KnowledgeEntry {
    pub id: Uuid,
    pub cwe_id: String,
    pub cwe_name: String,
    pub owasp_category: String,
    pub wstg_reference: String,
    pub capec_id: Option<String>,
    pub description: String,
    pub technical_details: String,
    pub impact: String,
    pub attack_scenario: String,
    pub remediation: RemediationInfo,
    pub verification_steps: Vec<String>,
    pub references: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct RemediationInfo {
    pub summary: String,
    pub detailed_fix: String,
    pub secure_example: String,
    pub references: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct EnrichmentResult {
    pub finding_id: Uuid,
    pub knowledge: Option<KnowledgeEntry>,
    pub related_cwe: Vec<String>,
    pub related_findings: Vec<Uuid>,
    pub remediation: Option<RemediationInfo>,
}

#[derive(Debug, Clone)]
pub struct TechnologyStack {
    pub framework: Option<String>,
    pub language: Option<String>,
    pub server: Option<String>,
    pub database: Option<String>,
    pub os: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ReferenceMap {
    pub cwe_to_owasp: Vec<(String, String)>,
    pub cwe_to_wstg: Vec<(String, String)>,
    pub cwe_to_capec: Vec<(String, Vec<String>)>,
}
