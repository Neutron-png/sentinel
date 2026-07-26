use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::models::{JwtAlgorithm, JwtLocation, JwtToken};
use super::JwtEngine;

pub struct JwtIntegration {
    engine: JwtEngine,
}

impl JwtIntegration {
    pub fn new() -> Self {
        JwtIntegration {
            engine: JwtEngine::new(),
        }
    }

    pub fn engine(&self) -> &JwtEngine {
        &self.engine
    }

    pub fn engine_mut(&mut self) -> &mut JwtEngine {
        &mut self.engine
    }

    pub fn scan_request(
        &mut self,
        method: &str,
        url_str: &str,
        headers: &[(String, String)],
        body: Option<&str>,
        content_type: Option<&str>,
    ) -> Vec<JwtScanResult> {
        let tokens = self
            .engine
            .detect_in_request(method, url_str, headers, body, content_type);

        tokens
            .iter()
            .map(|token| JwtScanResult {
                token_id: token.id,
                algorithm: token.algorithm.clone(),
                location: token.location.clone(),
                claim_count: token.claim_count(),
                has_exp: token.has_standard_claim("exp"),
                is_expired: token.is_expired(),
                is_valid: token.is_valid,
            })
            .collect()
    }

    pub fn scan_response(
        &mut self,
        url_str: &str,
        headers: &[(String, String)],
        body: Option<&str>,
        content_type: Option<&str>,
    ) -> Vec<JwtScanResult> {
        let tokens = self
            .engine
            .detect_in_response(url_str, headers, body, content_type);

        tokens
            .iter()
            .map(|token| JwtScanResult {
                token_id: token.id,
                algorithm: token.algorithm.clone(),
                location: token.location.clone(),
                claim_count: token.claim_count(),
                has_exp: token.has_standard_claim("exp"),
                is_expired: token.is_expired(),
                is_valid: token.is_valid,
            })
            .collect()
    }

    pub fn analyze_for_scanner(&self, token: &JwtToken) -> Vec<ScannerJwtFinding> {
        let analysis_findings = self.engine.analyze(token);
        analysis_findings
            .into_iter()
            .map(|f| ScannerJwtFinding {
                title: f.title,
                description: f.description,
                severity: f.severity,
                confidence: f.confidence,
                recommendation: f.recommendation,
                cwe: f.cwe,
                owasp_category: f.owasp_category,
                token_id: token.id,
                algorithm: token.algorithm.as_str().to_string(),
                location: token.location.location_type.label().to_string(),
            })
            .collect()
    }
}

impl Default for JwtIntegration {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtScanResult {
    pub token_id: Uuid,
    pub algorithm: JwtAlgorithm,
    pub location: JwtLocation,
    pub claim_count: usize,
    pub has_exp: bool,
    pub is_expired: Option<bool>,
    pub is_valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannerJwtFinding {
    pub title: String,
    pub description: String,
    pub severity: String,
    pub confidence: String,
    pub recommendation: String,
    pub cwe: Option<String>,
    pub owasp_category: Option<String>,
    pub token_id: Uuid,
    pub algorithm: String,
    pub location: String,
}
