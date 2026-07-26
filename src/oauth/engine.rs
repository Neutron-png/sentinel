use uuid::Uuid;

use super::analysis;
use super::detector;
use super::flows;
use super::models::{OAuthAnalysisFinding, OAuthFlowType, OAuthSession, OidcMetadata, TokenInfo};
use super::oidc;
use super::pkce;
use super::sessions::OAuthSessionTracker;
use super::tokens;

pub struct OAuthEngine {
    pub session_tracker: OAuthSessionTracker,
    pub oidc_metadata: Vec<OidcMetadata>,
    pub analysis_findings: Vec<(Uuid, Vec<OAuthAnalysisFinding>)>,
}

impl OAuthEngine {
    pub fn new() -> Self {
        OAuthEngine {
            session_tracker: OAuthSessionTracker::new(),
            oidc_metadata: Vec::new(),
            analysis_findings: Vec::new(),
        }
    }

    pub fn detect_in_request(
        &mut self,
        method: &str,
        url_str: &str,
        headers: &[(String, String)],
        body: Option<&str>,
        content_type: Option<&str>,
    ) -> Vec<Uuid> {
        let sessions =
            detector::detect_oauth_in_request(method, url_str, headers, body, content_type);
        sessions
            .into_iter()
            .map(|s| self.session_tracker.track_session(s))
            .collect()
    }

    pub fn detect_in_response(
        &mut self,
        url_str: &str,
        status_code: u16,
        headers: &[(String, String)],
        body: Option<&str>,
        content_type: Option<&str>,
    ) -> Vec<Uuid> {
        let sessions =
            detector::detect_oauth_in_response(url_str, status_code, headers, body, content_type);
        let mut ids = Vec::new();

        for session in sessions {
            let id = self.session_tracker.track_session(session);

            if let Some(s) = self.session_tracker.get_session(id) {
                if s.access_token.is_some() {
                    let analysis = analysis::analyze_session(s);
                    if !analysis.is_empty() {
                        self.analysis_findings.push((id, analysis));
                    }
                }
            }

            ids.push(id);
        }

        ids
    }

    pub fn process_discovery_document(
        &mut self,
        json: &str,
        _url: &str,
    ) -> Result<&OidcMetadata, super::errors::OAuthError> {
        let metadata = oidc::parse_discovery_document(json)?;
        self.oidc_metadata.push(metadata);
        let idx = self.oidc_metadata.len() - 1;

        let metadata_ref = &self.oidc_metadata[idx];
        let oidc_findings = analysis::analyze_oidc_metadata(metadata_ref);
        if !oidc_findings.is_empty() {
            self.analysis_findings.push((Uuid::new_v4(), oidc_findings));
        }

        Ok(&self.oidc_metadata[idx])
    }

    pub fn inspect_token(&self, session_id: Uuid) -> Option<TokenInfo> {
        let session = self.session_tracker.get_session(session_id)?;
        flows::inspect_access_token(session)
    }

    pub fn verify_pkce(&self, code_verifier: &str, code_challenge: &str, method: &str) -> bool {
        pkce::verify_pkce(code_verifier, code_challenge, method)
    }

    pub fn generate_code_challenge(&self, code_verifier: &str) -> (String, String) {
        pkce::generate_code_challenge(code_verifier)
    }

    pub fn get_session(&self, id: Uuid) -> Option<&OAuthSession> {
        self.session_tracker.get_session(id)
    }

    pub fn get_active_sessions(&self) -> Vec<&OAuthSession> {
        self.session_tracker.get_active_sessions()
    }

    pub fn get_pkce_sessions(&self) -> Vec<&OAuthSession> {
        self.session_tracker.get_pkce_sessions()
    }

    pub fn get_token_sessions(&self) -> Vec<&OAuthSession> {
        self.session_tracker.get_token_sessions()
    }

    pub fn analyze_session(&self, session_id: Uuid) -> Vec<OAuthAnalysisFinding> {
        self.session_tracker
            .get_session(session_id)
            .map(analysis::analyze_session)
            .unwrap_or_default()
    }

    pub fn analyze_all_sessions(&mut self) {
        self.analysis_findings.clear();
        for session in self.session_tracker.all_sessions() {
            let findings = analysis::analyze_session(session);
            if !findings.is_empty() {
                self.analysis_findings.push((session.id, findings));
            }
        }
    }

    pub fn all_sessions(&self) -> Vec<&OAuthSession> {
        self.session_tracker.all_sessions()
    }

    pub fn classify_flow(&self, session_id: Uuid) -> OAuthFlowType {
        self.session_tracker
            .get_session(session_id)
            .map(flows::classify_flow_type)
            .unwrap_or(OAuthFlowType::Unknown)
    }

    pub fn is_oidc_discovery_url(&self, url: &str) -> bool {
        oidc::is_oidc_discovery_url(url)
    }

    pub fn build_discovery_url(&self, base_url: &str) -> String {
        oidc::build_discovery_url(base_url)
    }

    pub fn clear(&mut self) {
        self.session_tracker.clear();
        self.oidc_metadata.clear();
        self.analysis_findings.clear();
    }
}

impl Default for OAuthEngine {
    fn default() -> Self {
        Self::new()
    }
}
