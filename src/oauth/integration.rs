use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::engine::OAuthEngine;
use super::models::{OAuthAnalysisFinding, OAuthFlowType, OAuthSession, TokenInfo};

pub struct OAuthIntegration {
    engine: OAuthEngine,
}

impl OAuthIntegration {
    pub fn new() -> Self {
        OAuthIntegration {
            engine: OAuthEngine::new(),
        }
    }

    pub fn engine(&self) -> &OAuthEngine {
        &self.engine
    }

    pub fn engine_mut(&mut self) -> &mut OAuthEngine {
        &mut self.engine
    }

    pub fn scan_request(
        &mut self,
        method: &str,
        url_str: &str,
        headers: &[(String, String)],
        body: Option<&str>,
        content_type: Option<&str>,
    ) -> Vec<OAuthScanResult> {
        let ids = self
            .engine
            .detect_in_request(method, url_str, headers, body, content_type);

        ids.into_iter()
            .filter_map(|id| {
                let session = self.engine.get_session(id)?;
                let token_info = self.engine.inspect_token(id);
                Some(OAuthScanResult {
                    session_id: id,
                    flow_type: session.flow_type,
                    client_id: session.client_id.clone(),
                    has_pkce: session.has_pkce(),
                    has_tokens: session.has_tokens(),
                    scopes: session.scopes.clone(),
                    token_info: token_info.map(|t| OAuthTokenScanInfo {
                        token_type: t.token_type,
                        is_jwt: t.is_jwt,
                        lifetime_seconds: t.lifetime_seconds,
                        is_expired: t.is_expired,
                        scopes: t.scopes,
                    }),
                })
            })
            .collect()
    }

    pub fn scan_response(
        &mut self,
        url_str: &str,
        status_code: u16,
        headers: &[(String, String)],
        body: Option<&str>,
        content_type: Option<&str>,
    ) -> Vec<OAuthScanResult> {
        let ids = self
            .engine
            .detect_in_response(url_str, status_code, headers, body, content_type);

        ids.into_iter()
            .filter_map(|id| {
                let session = self.engine.get_session(id)?;
                let token_info = self.engine.inspect_token(id);
                Some(OAuthScanResult {
                    session_id: id,
                    flow_type: session.flow_type,
                    client_id: session.client_id.clone(),
                    has_pkce: session.has_pkce(),
                    has_tokens: session.has_tokens(),
                    scopes: session.scopes.clone(),
                    token_info: token_info.map(|t| OAuthTokenScanInfo {
                        token_type: t.token_type,
                        is_jwt: t.is_jwt,
                        lifetime_seconds: t.lifetime_seconds,
                        is_expired: t.is_expired,
                        scopes: t.scopes,
                    }),
                })
            })
            .collect()
    }

    pub fn analyze_for_scanner(&self, session_id: Uuid) -> Vec<ScannerOAuthFinding> {
        let findings = self.engine.analyze_session(session_id);
        findings
            .into_iter()
            .map(|f| ScannerOAuthFinding {
                title: f.title,
                description: f.description,
                severity: f.severity,
                confidence: f.confidence,
                recommendation: f.recommendation,
                cwe: f.cwe,
                owasp_category: f.owasp_category,
                endpoint: f.endpoint,
                session_id,
            })
            .collect()
    }

    pub fn get_all_findings(&self) -> Vec<ScannerOAuthFinding> {
        self.engine
            .analysis_findings
            .iter()
            .flat_map(|(session_id, findings)| {
                findings.iter().map(move |f| ScannerOAuthFinding {
                    title: f.title.clone(),
                    description: f.description.clone(),
                    severity: f.severity.clone(),
                    confidence: f.confidence.clone(),
                    recommendation: f.recommendation.clone(),
                    cwe: f.cwe.clone(),
                    owasp_category: f.owasp_category.clone(),
                    endpoint: f.endpoint.clone(),
                    session_id: *session_id,
                })
            })
            .collect()
    }
}

impl Default for OAuthIntegration {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthScanResult {
    pub session_id: Uuid,
    pub flow_type: OAuthFlowType,
    pub client_id: Option<String>,
    pub has_pkce: bool,
    pub has_tokens: bool,
    pub scopes: Vec<String>,
    pub token_info: Option<OAuthTokenScanInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthTokenScanInfo {
    pub token_type: String,
    pub is_jwt: bool,
    pub lifetime_seconds: Option<i64>,
    pub is_expired: Option<bool>,
    pub scopes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannerOAuthFinding {
    pub title: String,
    pub description: String,
    pub severity: String,
    pub confidence: String,
    pub recommendation: String,
    pub cwe: Option<String>,
    pub owasp_category: Option<String>,
    pub endpoint: Option<String>,
    pub session_id: Uuid,
}
