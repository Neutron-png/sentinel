#![allow(dead_code)]

use crate::policy::errors::PolicyError;
use crate::policy::models::ScanPolicy;

pub fn load_from_file(path: &str) -> Result<ScanPolicy, PolicyError> {
    let content = std::fs::read_to_string(path).map_err(|e| PolicyError::Load(e.to_string()))?;
    serde_json::from_str(&content).map_err(|e| PolicyError::Load(e.to_string()))
}

pub fn load_from_json(json: &str) -> Result<ScanPolicy, PolicyError> {
    serde_json::from_str(json).map_err(|e| PolicyError::Load(e.to_string()))
}
