use chrono::{DateTime, Utc};

use super::models::{OAuthFlowType, OAuthSession, TokenInfo};

pub fn classify_flow_type(session: &OAuthSession) -> OAuthFlowType {
    if session.flow_type != OAuthFlowType::Unknown {
        return session.flow_type;
    }
    if session.has_pkce() {
        return OAuthFlowType::AuthorizationCodePkce;
    }
    if session.authorization_code.is_some() && session.access_token.is_some() {
        return OAuthFlowType::AuthorizationCode;
    }
    if session.access_token.is_some()
        && session.client_id.is_some()
        && session.authorization_code.is_none()
    {
        return OAuthFlowType::ClientCredentials;
    }
    if session.access_token.is_some() && session.refresh_token.is_some() {
        return OAuthFlowType::RefreshToken;
    }
    OAuthFlowType::Unknown
}

pub fn is_authorization_request(session: &OAuthSession) -> bool {
    session.authorization_endpoint.is_some()
        && session.client_id.is_some()
        && session.redirect_uri.is_some()
}

pub fn is_token_exchange(session: &OAuthSession) -> bool {
    session.token_endpoint.is_some()
        && (session.authorization_code.is_some() || session.access_token.is_some())
}

pub fn is_token_refresh(session: &OAuthSession) -> bool {
    session.flow_type == OAuthFlowType::RefreshToken
        || (session.refresh_token.is_some() && session.access_token.is_some())
}

pub fn is_logout(session: &OAuthSession) -> bool {
    !session.is_active
}

pub fn flow_description(session: &OAuthSession) -> String {
    let flow = classify_flow_type(session);
    let mut desc = format!("OAuth 2.0: {}", flow.label());

    if session.has_pkce() {
        desc.push_str(" with PKCE");
    }
    if let Some(ref client) = session.client_id {
        desc.push_str(&format!(" (Client: {})", client));
    }
    if !session.scopes.is_empty() {
        desc.push_str(&format!(" [Scopes: {}]", session.scopes.join(", ")));
    }
    desc
}

pub fn inspect_access_token(session: &OAuthSession) -> Option<TokenInfo> {
    let token = session.access_token.as_ref()?;
    let mut info = TokenInfo {
        token_type: session
            .token_type
            .clone()
            .unwrap_or_else(|| "Bearer".to_string()),
        algorithm: None,
        scopes: session.scopes.clone(),
        expires_at: session
            .expires_in
            .map(|secs| Utc::now() + chrono::Duration::seconds(secs)),
        issuer: session.issuer.clone(),
        audience: None,
        is_expired: None,
        lifetime_seconds: session.expires_in,
        is_jwt: is_jwt_format(token),
    };

    if info.is_jwt {
        if let Ok(header) = crate::jwt::parser::parse_token(
            token,
            crate::jwt::models::JwtLocation {
                location_type: crate::jwt::models::JwtLocationType::AuthorizationHeader,
                key: None,
                url: None,
                host: None,
            },
        ) {
            info.algorithm = Some(header.algorithm.to_string());
            info.issuer = header
                .claims
                .as_object()
                .and_then(|c| c.get("iss"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .or(info.issuer);
            info.audience = header
                .claims
                .as_object()
                .and_then(|c| c.get("aud"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            info.is_expired = header.is_expired();
            if let (Some(iat), Some(exp)) = (
                header
                    .claims
                    .as_object()
                    .and_then(|c| c.get("iat"))
                    .and_then(|v| v.as_i64()),
                header
                    .claims
                    .as_object()
                    .and_then(|c| c.get("exp"))
                    .and_then(|v| v.as_i64()),
            ) {
                info.lifetime_seconds = Some(exp - iat);
            }
        }
    }

    Some(info)
}

fn is_jwt_format(s: &str) -> bool {
    let parts: Vec<&str> = s.splitn(3, '.').collect();
    if parts.len() < 2 {
        return false;
    }
    parts.iter().all(|p| {
        p.is_empty()
            || p.chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    })
}
