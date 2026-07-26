use uuid::Uuid;

use super::analysis;
use super::detector;
use super::editor;
use super::errors::JwtError;
use super::models::{JwtAnalysisFinding, JwtEditResult, JwtLocation, JwtToken};
use super::parser;
use super::validator::{validate_token, validate_token_mut, ValidationPolicy};

pub struct JwtEngine {
    pub detected_tokens: Vec<JwtToken>,
    pub analysis_results: Vec<(Uuid, Vec<JwtAnalysisFinding>)>,
}

impl JwtEngine {
    pub fn new() -> Self {
        JwtEngine {
            detected_tokens: Vec::new(),
            analysis_results: Vec::new(),
        }
    }

    pub fn detect_in_request(
        &mut self,
        method: &str,
        url_str: &str,
        headers: &[(String, String)],
        body: Option<&str>,
        content_type: Option<&str>,
    ) -> &[JwtToken] {
        let prev_len = self.detected_tokens.len();
        let new_tokens = detector::detect_in_request(method, url_str, headers, body, content_type);
        self.detected_tokens.extend(new_tokens);
        &self.detected_tokens[prev_len..]
    }

    pub fn detect_in_response(
        &mut self,
        url_str: &str,
        headers: &[(String, String)],
        body: Option<&str>,
        content_type: Option<&str>,
    ) -> &[JwtToken] {
        let prev_len = self.detected_tokens.len();
        let new_tokens = detector::detect_in_response(url_str, headers, body, content_type);
        self.detected_tokens.extend(new_tokens);
        &self.detected_tokens[prev_len..]
    }

    pub fn detect_in_string(&mut self, input: &str, location: JwtLocation) -> Vec<&JwtToken> {
        let tokens = detector::detect_in_string(input, location);
        let start = self.detected_tokens.len();
        self.detected_tokens.extend(tokens);
        self.detected_tokens[start..].iter().collect()
    }

    pub fn parse(&self, raw: &str, location: JwtLocation) -> Result<JwtToken, JwtError> {
        parser::parse_token(raw, location)
    }

    pub fn validate(&self, token: &JwtToken, policy: &ValidationPolicy) -> Vec<String> {
        validate_token(token, policy)
    }

    pub fn validate_all(&mut self, policy: &ValidationPolicy) {
        for token in &mut self.detected_tokens {
            validate_token_mut(token, policy);
        }
    }

    pub fn analyze(&self, token: &JwtToken) -> Vec<JwtAnalysisFinding> {
        analysis::analyze_token(token)
    }

    pub fn analyze_all(&mut self) {
        self.analysis_results = self
            .detected_tokens
            .iter()
            .map(|t| (t.id, analysis::analyze_token(t)))
            .collect();
    }

    pub fn edit_header(
        &self,
        token: &JwtToken,
        new_header: &serde_json::Value,
    ) -> Result<String, JwtError> {
        editor::edit_header(token, new_header)
    }

    pub fn edit_claims(
        &self,
        token: &JwtToken,
        new_claims: &serde_json::Value,
    ) -> Result<String, JwtError> {
        editor::edit_claims(token, new_claims)
    }

    pub fn add_claim(
        &self,
        token: &JwtToken,
        key: &str,
        value: &serde_json::Value,
    ) -> Result<String, JwtError> {
        editor::add_claim(token, key, value)
    }

    pub fn remove_claim(&self, token: &JwtToken, key: &str) -> Result<String, JwtError> {
        editor::remove_claim(token, key)
    }

    pub fn set_algorithm(&self, token: &JwtToken, algorithm: &str) -> Result<String, JwtError> {
        editor::set_algorithm(token, algorithm)
    }

    pub fn strip_signature(&self, token: &JwtToken) -> String {
        editor::strip_signature(token)
    }

    pub fn preview_edit(&self, token: &JwtToken, new_claims: &serde_json::Value) -> JwtEditResult {
        editor::preview_edit(token, new_claims)
    }

    pub fn rebuild_token(
        &self,
        header_json: &str,
        payload_json: &str,
        signature: Option<&str>,
    ) -> Result<String, JwtError> {
        editor::rebuild_token(header_json, payload_json, signature)
    }

    pub fn get_token(&self, id: &Uuid) -> Option<&JwtToken> {
        self.detected_tokens.iter().find(|t| t.id == *id)
    }

    pub fn clear(&mut self) {
        self.detected_tokens.clear();
        self.analysis_results.clear();
    }
}

impl Default for JwtEngine {
    fn default() -> Self {
        Self::new()
    }
}
