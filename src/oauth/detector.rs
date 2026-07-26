use regex::Regex;
use std::collections::HashMap;
use url::Url;

use super::models::{OAuthEndpoint, OAuthEndpointType, OAuthFlowType, OAuthSession};

const OAUTH_PATH_PATTERNS: &[(&str, OAuthEndpointType)] = &[
    ("/authorize", OAuthEndpointType::Authorization),
    ("/auth", OAuthEndpointType::Authorization),
    ("/oauth/authorize", OAuthEndpointType::Authorization),
    ("/oauth2/authorize", OAuthEndpointType::Authorization),
    ("/token", OAuthEndpointType::Token),
    ("/oauth/token", OAuthEndpointType::Token),
    ("/oauth2/token", OAuthEndpointType::Token),
    ("/userinfo", OAuthEndpointType::UserInfo),
    ("/oauth/userinfo", OAuthEndpointType::UserInfo),
    ("/jwks", OAuthEndpointType::Jwks),
    ("/jwks.json", OAuthEndpointType::Jwks),
    ("/.well-known/jwks.json", OAuthEndpointType::Jwks),
    ("/revoke", OAuthEndpointType::Revocation),
    ("/oauth/revoke", OAuthEndpointType::Revocation),
    ("/introspect", OAuthEndpointType::Introspection),
    ("/oauth/introspect", OAuthEndpointType::Introspection),
    ("/logout", OAuthEndpointType::Logout),
    ("/oauth/logout", OAuthEndpointType::Logout),
    ("/device", OAuthEndpointType::DeviceAuthorization),
    ("/device/code", OAuthEndpointType::DeviceAuthorization),
    ("/oauth/device", OAuthEndpointType::DeviceAuthorization),
    (
        "/.well-known/openid-configuration",
        OAuthEndpointType::Unknown,
    ),
];

const OAUTH_PARAMS: &[&str] = &[
    "client_id",
    "redirect_uri",
    "response_type",
    "scope",
    "state",
    "code_challenge",
    "code_challenge_method",
    "grant_type",
    "code",
    "access_token",
    "refresh_token",
    "id_token",
    "token_type",
    "expires_in",
    "nonce",
    "error",
    "error_description",
];

pub fn classify_endpoint(url_str: &str, _method: &str) -> Option<OAuthEndpointType> {
    if let Ok(parsed) = Url::parse(url_str) {
        let path = parsed.path().to_lowercase();
        for (pattern, ep_type) in OAUTH_PATH_PATTERNS {
            if path.ends_with(pattern) || path == *pattern {
                return Some(*ep_type);
            }
        }
    }
    None
}

pub fn detect_oauth_params(params: &[(String, String)]) -> bool {
    let keys: Vec<&str> = params.iter().map(|(k, _)| k.as_str()).collect();
    let oauth_key_count = OAUTH_PARAMS.iter().filter(|p| keys.contains(p)).count();
    oauth_key_count >= 3
}

pub fn extract_oauth_params(url_str: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();
    if let Ok(parsed) = Url::parse(url_str) {
        for (key, value) in parsed.query_pairs() {
            params.insert(key.to_string(), value.to_string());
        }
    }
    params
}

pub fn detect_flow_type(params: &HashMap<String, String>) -> OAuthFlowType {
    if params.contains_key("code_verifier") || params.contains_key("code_challenge") {
        return OAuthFlowType::AuthorizationCodePkce;
    }
    match params.get("grant_type").map(|s| s.as_str()) {
        Some("authorization_code") => {
            if params.contains_key("code_verifier") {
                OAuthFlowType::AuthorizationCodePkce
            } else {
                OAuthFlowType::AuthorizationCode
            }
        }
        Some("client_credentials") => OAuthFlowType::ClientCredentials,
        Some("refresh_token") => OAuthFlowType::RefreshToken,
        Some("password") => OAuthFlowType::ResourceOwnerPassword,
        Some("urn:ietf:params:oauth:grant-type:device_code") => OAuthFlowType::DeviceAuthorization,
        _ => {
            if params.contains_key("code") && params.contains_key("state") {
                OAuthFlowType::AuthorizationCode
            } else if params.contains_key("access_token") {
                OAuthFlowType::Implicit
            } else if params.contains_key("device_code") {
                OAuthFlowType::DeviceAuthorization
            } else {
                OAuthFlowType::Unknown
            }
        }
    }
}

pub fn detect_oauth_in_url(url_str: &str) -> Option<OAuthSession> {
    let endpoint_type = classify_endpoint(url_str, "GET")?;
    let params = extract_oauth_params(url_str);

    if !detect_oauth_params(
        &params
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<Vec<_>>(),
    ) {
        return None;
    }

    let flow_type = detect_flow_type(&params);

    let mut session = OAuthSession::new(flow_type);
    session.client_id = params.get("client_id").cloned();
    session.redirect_uri = params.get("redirect_uri").cloned();
    session.state = params.get("state").cloned();
    session.nonce = params.get("nonce").cloned();
    session.code_challenge = params.get("code_challenge").cloned();
    session.code_challenge_method = params.get("code_challenge_method").cloned();
    session.authorization_code = params.get("code").cloned();
    session.scopes = params
        .get("scope")
        .map(|s| s.split_whitespace().map(|p| p.to_string()).collect())
        .unwrap_or_default();

    match endpoint_type {
        OAuthEndpointType::Authorization => {
            session.authorization_endpoint = Some(url_str.to_string());
        }
        OAuthEndpointType::Token => {
            session.token_endpoint = Some(url_str.to_string());
        }
        _ => {}
    }

    Some(session)
}

pub fn detect_oauth_in_body(
    body: &str,
    content_type: Option<&str>,
    url_str: &str,
) -> Option<OAuthSession> {
    let is_form = content_type
        .map(|ct| ct.contains("application/x-www-form-urlencoded"))
        .unwrap_or(false);
    let is_json = content_type
        .map(|ct| ct.contains("application/json"))
        .unwrap_or(false);

    let params: HashMap<String, String> = if is_form {
        body.split('&')
            .filter_map(|pair| {
                let mut parts = pair.splitn(2, '=');
                let key = parts.next()?.to_string();
                let value = parts.next().map(urlencoding_decode).unwrap_or_default();
                Some((key, value))
            })
            .collect()
    } else if is_json {
        serde_json::from_str::<serde_json::Value>(body)
            .ok()
            .and_then(|v| v.as_object().cloned())
            .map(|obj| {
                obj.into_iter()
                    .map(|(k, v)| {
                        let val = match v {
                            serde_json::Value::String(s) => s,
                            serde_json::Value::Number(n) => n.to_string(),
                            serde_json::Value::Bool(b) => b.to_string(),
                            _ => v.to_string(),
                        };
                        (k, val)
                    })
                    .collect()
            })
            .unwrap_or_default()
    } else {
        return None;
    };

    let oauth_keys = [
        "access_token",
        "token_type",
        "expires_in",
        "refresh_token",
        "id_token",
        "scope",
        "error",
    ];

    let key_count = oauth_keys
        .iter()
        .filter(|k| params.contains_key(*k as &str))
        .count();
    if key_count < 2 {
        return None;
    }

    let flow_type = detect_flow_type(&params);
    let mut session = OAuthSession::new(flow_type);
    session.access_token = params.get("access_token").cloned();
    session.refresh_token = params.get("refresh_token").cloned();
    session.id_token = params.get("id_token").cloned();
    session.token_type = params.get("token_type").cloned();
    session.expires_in = params.get("expires_in").and_then(|v| v.parse().ok());
    session.scopes = params
        .get("scope")
        .map(|s| s.split_whitespace().map(|p| p.to_string()).collect())
        .unwrap_or_default();
    session.token_endpoint = Some(url_str.to_string());

    Some(session)
}

pub fn detect_oauth_in_headers(headers: &[(String, String)]) -> Vec<OAuthSession> {
    let mut sessions = Vec::new();

    for (name, value) in headers {
        let lower = name.to_lowercase();
        if lower == "authorization" {
            if let Some(token) = value.strip_prefix("Bearer ") {
                let mut session = OAuthSession::new(OAuthFlowType::Unknown);
                session.access_token = Some(token.trim().to_string());
                session.token_type = Some("Bearer".to_string());
                sessions.push(session);
            } else if let Some(token) = value.strip_prefix("Basic ") {
                let mut session = OAuthSession::new(OAuthFlowType::ClientCredentials);
                if let Ok(decoded) =
                    base64::Engine::decode(&base64::engine::general_purpose::STANDARD, token.trim())
                {
                    if let Ok(creds) = String::from_utf8(decoded) {
                        if let Some((id, _secret)) = creds.split_once(':') {
                            session.client_id = Some(id.to_string());
                        }
                    }
                }
                sessions.push(session);
            }
        } else if lower == "x-oauth-state" || lower == "oauth-state" {
            let mut session = OAuthSession::new(OAuthFlowType::AuthorizationCode);
            session.state = Some(value.clone());
            sessions.push(session);
        }
    }

    sessions
}

pub fn detect_oauth_in_response(
    url_str: &str,
    status_code: u16,
    headers: &[(String, String)],
    body: Option<&str>,
    content_type: Option<&str>,
) -> Vec<OAuthSession> {
    let mut sessions = Vec::new();

    if status_code == 302
        || status_code == 301
        || status_code == 303
        || status_code == 307
        || status_code == 308
    {
        for (name, value) in headers {
            if name.to_lowercase() == "location" {
                if let Some(session) = detect_oauth_in_url(value) {
                    sessions.push(session);
                }
            }
        }
    }

    if let Some(session) = detect_oauth_in_url(url_str) {
        sessions.push(session);
    }

    if let Some(body) = body {
        if let Some(session) = detect_oauth_in_body(body, content_type, url_str) {
            sessions.push(session);
        }
    }

    sessions.extend(detect_oauth_in_headers(headers));

    sessions
}

pub fn detect_oauth_in_request(
    _method: &str,
    url_str: &str,
    headers: &[(String, String)],
    body: Option<&str>,
    content_type: Option<&str>,
) -> Vec<OAuthSession> {
    let mut sessions = Vec::new();

    if let Some(session) = detect_oauth_in_url(url_str) {
        sessions.push(session);
    }

    if let Some(body) = body {
        if let Some(session) = detect_oauth_in_body(body, content_type, url_str) {
            sessions.push(session);
        }
    }

    sessions.extend(detect_oauth_in_headers(headers));

    sessions
}

fn urlencoding_decode(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'+' => {
                result.push(' ');
                i += 1;
            }
            b'%' if i + 2 < bytes.len() => {
                if let (Some(hi), Some(lo)) = (hex_val(bytes[i + 1]), hex_val(bytes[i + 2])) {
                    result.push((hi << 4 | lo) as char);
                    i += 3;
                } else {
                    result.push('%');
                    i += 1;
                }
            }
            _ => {
                result.push(bytes[i] as char);
                i += 1;
            }
        }
    }
    result
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}
