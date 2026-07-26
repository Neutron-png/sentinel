use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OAuthFlowType {
    AuthorizationCode,
    AuthorizationCodePkce,
    ClientCredentials,
    DeviceAuthorization,
    RefreshToken,
    Implicit,
    ResourceOwnerPassword,
    Unknown,
}

impl OAuthFlowType {
    pub fn label(&self) -> &'static str {
        match self {
            OAuthFlowType::AuthorizationCode => "Authorization Code",
            OAuthFlowType::AuthorizationCodePkce => "Authorization Code + PKCE",
            OAuthFlowType::ClientCredentials => "Client Credentials",
            OAuthFlowType::DeviceAuthorization => "Device Authorization",
            OAuthFlowType::RefreshToken => "Refresh Token",
            OAuthFlowType::Implicit => "Implicit (Legacy)",
            OAuthFlowType::ResourceOwnerPassword => "Resource Owner Password (Legacy)",
            OAuthFlowType::Unknown => "Unknown",
        }
    }

    pub fn is_legacy(&self) -> bool {
        matches!(
            self,
            OAuthFlowType::Implicit | OAuthFlowType::ResourceOwnerPassword
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthSession {
    pub id: Uuid,
    pub client_id: Option<String>,
    pub flow_type: OAuthFlowType,
    pub authorization_endpoint: Option<String>,
    pub token_endpoint: Option<String>,
    pub redirect_uri: Option<String>,
    pub scopes: Vec<String>,
    pub state: Option<String>,
    pub nonce: Option<String>,
    pub code_verifier: Option<String>,
    pub code_challenge: Option<String>,
    pub code_challenge_method: Option<String>,
    pub authorization_code: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub id_token: Option<String>,
    pub token_type: Option<String>,
    pub expires_in: Option<i64>,
    pub issuer: Option<String>,
    pub detected_at: DateTime<Utc>,
    pub last_seen_at: DateTime<Utc>,
    pub is_active: bool,
}

impl OAuthSession {
    pub fn new(flow_type: OAuthFlowType) -> Self {
        let now = Utc::now();
        OAuthSession {
            id: Uuid::new_v4(),
            client_id: None,
            flow_type,
            authorization_endpoint: None,
            token_endpoint: None,
            redirect_uri: None,
            scopes: Vec::new(),
            state: None,
            nonce: None,
            code_verifier: None,
            code_challenge: None,
            code_challenge_method: None,
            authorization_code: None,
            access_token: None,
            refresh_token: None,
            id_token: None,
            token_type: None,
            expires_in: None,
            issuer: None,
            detected_at: now,
            last_seen_at: now,
            is_active: true,
        }
    }

    pub fn has_pkce(&self) -> bool {
        self.code_verifier.is_some() || self.code_challenge.is_some()
    }

    pub fn has_tokens(&self) -> bool {
        self.access_token.is_some()
    }

    pub fn has_refresh_token(&self) -> bool {
        self.refresh_token.is_some()
    }

    pub fn has_id_token(&self) -> bool {
        self.id_token.is_some()
    }

    pub fn touch(&mut self) {
        self.last_seen_at = Utc::now();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OidcMetadata {
    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub userinfo_endpoint: Option<String>,
    pub jwks_uri: Option<String>,
    pub registration_endpoint: Option<String>,
    pub scopes_supported: Vec<String>,
    pub response_types_supported: Vec<String>,
    pub grant_types_supported: Vec<String>,
    pub subject_types_supported: Vec<String>,
    pub id_token_signing_alg_values_supported: Vec<String>,
    pub token_endpoint_auth_methods_supported: Vec<String>,
    pub claims_supported: Vec<String>,
    pub end_session_endpoint: Option<String>,
    pub revocation_endpoint: Option<String>,
    pub introspection_endpoint: Option<String>,
}

impl OidcMetadata {
    pub fn supports_pkce(&self) -> bool {
        self.code_challenge_methods_supported()
            .iter()
            .any(|m| m == "S256")
    }

    pub fn code_challenge_methods_supported(&self) -> Vec<String> {
        vec!["S256".to_string(), "plain".to_string()]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    pub token_type: String,
    pub algorithm: Option<String>,
    pub scopes: Vec<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub issuer: Option<String>,
    pub audience: Option<String>,
    pub is_expired: Option<bool>,
    pub lifetime_seconds: Option<i64>,
    pub is_jwt: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthEndpoint {
    pub url: String,
    pub method: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
    pub is_oauth: bool,
    pub endpoint_type: OAuthEndpointType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OAuthEndpointType {
    Authorization,
    Token,
    UserInfo,
    Jwks,
    Revocation,
    Introspection,
    Logout,
    DeviceAuthorization,
    Unknown,
}

impl OAuthEndpointType {
    pub fn label(&self) -> &'static str {
        match self {
            OAuthEndpointType::Authorization => "Authorization",
            OAuthEndpointType::Token => "Token",
            OAuthEndpointType::UserInfo => "UserInfo",
            OAuthEndpointType::Jwks => "JWKS",
            OAuthEndpointType::Revocation => "Revocation",
            OAuthEndpointType::Introspection => "Introspection",
            OAuthEndpointType::Logout => "Logout",
            OAuthEndpointType::DeviceAuthorization => "Device Authorization",
            OAuthEndpointType::Unknown => "Unknown",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthAnalysisFinding {
    pub title: String,
    pub description: String,
    pub severity: String,
    pub confidence: String,
    pub recommendation: String,
    pub cwe: Option<String>,
    pub owasp_category: Option<String>,
    pub endpoint: Option<String>,
    pub evidence: String,
}
