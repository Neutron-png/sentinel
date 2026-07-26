use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeEntry {
    #[serde(rename = "ref")]
    pub reference: String,
    pub title: String,
    pub objective: String,
    pub description: String,
    pub prerequisites: String,
    pub testing_steps: String,
    pub expected_result: String,
    pub required_evidence: String,
    pub references: String,
}
