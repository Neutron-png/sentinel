use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum JwtAlgorithm {
    HS256,
    HS384,
    HS512,
    RS256,
    RS384,
    RS512,
    ES256,
    ES384,
    ES512,
    None,
    Unknown(String),
}

impl JwtAlgorithm {
    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "HS256" => JwtAlgorithm::HS256,
            "HS384" => JwtAlgorithm::HS384,
            "HS512" => JwtAlgorithm::HS512,
            "RS256" => JwtAlgorithm::RS256,
            "RS384" => JwtAlgorithm::RS384,
            "RS512" => JwtAlgorithm::RS512,
            "ES256" => JwtAlgorithm::ES256,
            "ES384" => JwtAlgorithm::ES384,
            "ES512" => JwtAlgorithm::ES512,
            "NONE" => JwtAlgorithm::None,
            other => JwtAlgorithm::Unknown(other.to_string()),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            JwtAlgorithm::HS256 => "HS256",
            JwtAlgorithm::HS384 => "HS384",
            JwtAlgorithm::HS512 => "HS512",
            JwtAlgorithm::RS256 => "RS256",
            JwtAlgorithm::RS384 => "RS384",
            JwtAlgorithm::RS512 => "RS512",
            JwtAlgorithm::ES256 => "ES256",
            JwtAlgorithm::ES384 => "ES384",
            JwtAlgorithm::ES512 => "ES512",
            JwtAlgorithm::None => "none",
            JwtAlgorithm::Unknown(s) => s.as_str(),
        }
    }

    pub fn is_symmetric(&self) -> bool {
        matches!(
            self,
            JwtAlgorithm::HS256 | JwtAlgorithm::HS384 | JwtAlgorithm::HS512
        )
    }

    pub fn is_asymmetric(&self) -> bool {
        matches!(
            self,
            JwtAlgorithm::RS256
                | JwtAlgorithm::RS384
                | JwtAlgorithm::RS512
                | JwtAlgorithm::ES256
                | JwtAlgorithm::ES384
                | JwtAlgorithm::ES512
        )
    }

    pub fn is_none(&self) -> bool {
        matches!(self, JwtAlgorithm::None)
    }

    pub fn is_weak(&self) -> bool {
        matches!(
            self,
            JwtAlgorithm::None | JwtAlgorithm::HS256 | JwtAlgorithm::RS256 | JwtAlgorithm::ES256
        )
    }
}

impl std::fmt::Display for JwtAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtHeader {
    pub alg: Option<String>,
    pub typ: Option<String>,
    pub kid: Option<String>,
    pub jku: Option<String>,
    pub jwk: Option<serde_json::Value>,
    pub x5u: Option<String>,
    pub x5t: Option<String>,
    pub x5c: Option<Vec<String>>,
    pub cty: Option<String>,
    pub crit: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl JwtHeader {
    pub fn algorithm(&self) -> JwtAlgorithm {
        match &self.alg {
            Some(a) => JwtAlgorithm::from_str(a),
            None => JwtAlgorithm::None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub iss: Option<String>,
    pub sub: Option<String>,
    pub aud: Option<serde_json::Value>,
    pub exp: Option<i64>,
    pub nbf: Option<i64>,
    pub iat: Option<i64>,
    pub jti: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum JwtLocationType {
    AuthorizationHeader,
    Cookie,
    QueryParameter,
    PostBody,
    JsonBody,
    FormBody,
    MultipartForm,
    LocalStorage,
    SessionStorage,
    BrowserStorage,
    HttpHistory,
    ProxyTraffic,
    BrowserRequest,
    BrowserResponse,
    RepeaterRequest,
    CustomHeader,
    WorkflowVariable,
}

impl JwtLocationType {
    pub fn label(&self) -> &'static str {
        match self {
            JwtLocationType::AuthorizationHeader => "Authorization Header",
            JwtLocationType::Cookie => "Cookie",
            JwtLocationType::QueryParameter => "Query Parameter",
            JwtLocationType::PostBody => "POST Body",
            JwtLocationType::JsonBody => "JSON Body",
            JwtLocationType::FormBody => "Form Body",
            JwtLocationType::MultipartForm => "Multipart Form",
            JwtLocationType::LocalStorage => "Local Storage",
            JwtLocationType::SessionStorage => "Session Storage",
            JwtLocationType::BrowserStorage => "Browser Storage",
            JwtLocationType::HttpHistory => "HTTP History",
            JwtLocationType::ProxyTraffic => "Proxy Traffic",
            JwtLocationType::BrowserRequest => "Browser Request",
            JwtLocationType::BrowserResponse => "Browser Response",
            JwtLocationType::RepeaterRequest => "Repeater Request",
            JwtLocationType::CustomHeader => "Custom Header",
            JwtLocationType::WorkflowVariable => "Workflow Variable",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtLocation {
    pub location_type: JwtLocationType,
    pub key: Option<String>,
    pub url: Option<String>,
    pub host: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtToken {
    pub id: Uuid,
    pub raw: String,
    pub header_raw: String,
    pub payload_raw: String,
    pub signature_raw: String,
    pub header: JwtHeader,
    pub claims: serde_json::Value,
    pub algorithm: JwtAlgorithm,
    pub location: JwtLocation,
    pub detected_at: DateTime<Utc>,
    pub is_valid: bool,
    pub validation_errors: Vec<String>,
}

impl JwtToken {
    pub fn has_standard_claim(&self, name: &str) -> bool {
        self.claims.get(name).is_some()
    }

    pub fn claim_count(&self) -> usize {
        self.claims.as_object().map(|o| o.len()).unwrap_or(0)
    }

    pub fn is_expired(&self) -> Option<bool> {
        if let Some(claims) = self.claims.as_object() {
            if let Some(exp) = claims.get("exp").and_then(|v| v.as_i64()) {
                let now = Utc::now().timestamp();
                return Some(now > exp);
            }
        }
        None
    }

    pub fn expiration(&self) -> Option<DateTime<Utc>> {
        self.claims
            .as_object()?
            .get("exp")?
            .as_i64()
            .map(|ts| DateTime::from_timestamp(ts, 0).unwrap_or_default())
    }

    pub fn issued_at(&self) -> Option<DateTime<Utc>> {
        self.claims
            .as_object()?
            .get("iat")?
            .as_i64()
            .map(|ts| DateTime::from_timestamp(ts, 0).unwrap_or_default())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtDetectionResult {
    pub id: Uuid,
    pub token: JwtToken,
    pub source: String,
    pub source_type: JwtLocationType,
    pub request_url: Option<String>,
    pub response_status: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtAnalysisFinding {
    pub title: String,
    pub description: String,
    pub severity: String,
    pub confidence: String,
    pub recommendation: String,
    pub cwe: Option<String>,
    pub owasp_category: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtEditResult {
    pub original: String,
    pub modified: String,
    pub header_raw: String,
    pub payload_raw: String,
    pub claims: serde_json::Value,
}
