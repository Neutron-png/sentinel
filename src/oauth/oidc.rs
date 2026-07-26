use serde::{Deserialize, Serialize};

use super::errors::OAuthError;
use super::models::OidcMetadata;

const OIDC_WELL_KNOWN_PATH: &str = "/.well-known/openid-configuration";
const OAUTH_AUTHORIZATION_SERVER_METADATA: &str = "/.well-known/oauth-authorization-server";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OidcDiscoveryDoc {
    issuer: Option<String>,
    authorization_endpoint: Option<String>,
    token_endpoint: Option<String>,
    userinfo_endpoint: Option<String>,
    jwks_uri: Option<String>,
    registration_endpoint: Option<String>,
    scopes_supported: Option<Vec<String>>,
    response_types_supported: Option<Vec<String>>,
    grant_types_supported: Option<Vec<String>>,
    subject_types_supported: Option<Vec<String>>,
    id_token_signing_alg_values_supported: Option<Vec<String>>,
    token_endpoint_auth_methods_supported: Option<Vec<String>>,
    claims_supported: Option<Vec<String>>,
    end_session_endpoint: Option<String>,
    revocation_endpoint: Option<String>,
    introspection_endpoint: Option<String>,
}

pub fn build_discovery_url(base_url: &str) -> String {
    let base = base_url.trim_end_matches('/');
    format!("{}{}", base, OIDC_WELL_KNOWN_PATH)
}

pub fn parse_discovery_document(json: &str) -> Result<OidcMetadata, OAuthError> {
    let doc: OidcDiscoveryDoc =
        serde_json::from_str(json).map_err(|e| OAuthError::Oidc(e.to_string()))?;

    let issuer = doc
        .issuer
        .ok_or_else(|| OAuthError::Oidc("Missing issuer in discovery document".into()))?;
    let auth_endpoint = doc
        .authorization_endpoint
        .ok_or_else(|| OAuthError::Oidc("Missing authorization_endpoint".into()))?;
    let token_endpoint = doc
        .token_endpoint
        .ok_or_else(|| OAuthError::Oidc("Missing token_endpoint".into()))?;

    Ok(OidcMetadata {
        issuer,
        authorization_endpoint: auth_endpoint,
        token_endpoint,
        userinfo_endpoint: doc.userinfo_endpoint,
        jwks_uri: doc.jwks_uri,
        registration_endpoint: doc.registration_endpoint,
        scopes_supported: doc.scopes_supported.unwrap_or_default(),
        response_types_supported: doc.response_types_supported.unwrap_or_default(),
        grant_types_supported: doc.grant_types_supported.unwrap_or_default(),
        subject_types_supported: doc.subject_types_supported.unwrap_or_default(),
        id_token_signing_alg_values_supported: doc
            .id_token_signing_alg_values_supported
            .unwrap_or_default(),
        token_endpoint_auth_methods_supported: doc
            .token_endpoint_auth_methods_supported
            .unwrap_or_default(),
        claims_supported: doc.claims_supported.unwrap_or_default(),
        end_session_endpoint: doc.end_session_endpoint,
        revocation_endpoint: doc.revocation_endpoint,
        introspection_endpoint: doc.introspection_endpoint,
    })
}

pub fn is_oidc_discovery_url(url: &str) -> bool {
    url.contains(OIDC_WELL_KNOWN_PATH) || url.contains(OAUTH_AUTHORIZATION_SERVER_METADATA)
}

pub fn extract_issuer_from_discovery_url(url: &str) -> Option<String> {
    let base = url
        .replace(OIDC_WELL_KNOWN_PATH, "")
        .replace(OAUTH_AUTHORIZATION_SERVER_METADATA, "");
    let trimmed = base.trim_end_matches('/');
    if !trimmed.is_empty() {
        Some(trimmed.to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_discovery_document() {
        let json = r#"{
            "issuer": "https://accounts.example.com",
            "authorization_endpoint": "https://accounts.example.com/authorize",
            "token_endpoint": "https://accounts.example.com/token",
            "userinfo_endpoint": "https://accounts.example.com/userinfo",
            "jwks_uri": "https://accounts.example.com/jwks",
            "scopes_supported": ["openid", "profile", "email"],
            "response_types_supported": ["code", "id_token"],
            "grant_types_supported": ["authorization_code", "refresh_token"],
            "subject_types_supported": ["public"],
            "id_token_signing_alg_values_supported": ["RS256"]
        }"#;

        let metadata = parse_discovery_document(json);
        assert!(metadata.is_ok());
        let m = metadata.unwrap();
        assert_eq!(m.issuer, "https://accounts.example.com");
        assert_eq!(
            m.authorization_endpoint,
            "https://accounts.example.com/authorize"
        );
        assert_eq!(m.token_endpoint, "https://accounts.example.com/token");
        assert_eq!(
            m.userinfo_endpoint.unwrap(),
            "https://accounts.example.com/userinfo"
        );
        assert!(m.scopes_supported.contains(&"openid".to_string()));
    }

    #[test]
    fn test_is_oidc_discovery_url() {
        assert!(is_oidc_discovery_url(
            "https://example.com/.well-known/openid-configuration"
        ));
        assert!(!is_oidc_discovery_url("https://example.com/login"));
    }

    #[test]
    fn test_build_discovery_url() {
        assert_eq!(
            build_discovery_url("https://example.com"),
            "https://example.com/.well-known/openid-configuration"
        );
        assert_eq!(
            build_discovery_url("https://example.com/"),
            "https://example.com/.well-known/openid-configuration"
        );
    }
}
