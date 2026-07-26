use crate::models::knowledge::KnowledgeEntry;

const WSTG_JSON: &str = include_str!("wstg_knowledge.json");

pub fn load_all() -> Vec<KnowledgeEntry> {
    serde_json::from_str(WSTG_JSON).unwrap_or_default()
}
