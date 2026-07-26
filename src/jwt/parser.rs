use base64::Engine;
use chrono::Utc;
use serde_json::Value;
use uuid::Uuid;

use super::errors::JwtError;
use super::models::{JwtHeader, JwtLocation, JwtToken};

const BASE64URL_ENGINE: base64::engine::GeneralPurpose = base64::engine::GeneralPurpose::new(
    &base64::alphabet::URL_SAFE,
    base64::engine::general_purpose::NO_PAD,
);

fn decode_base64url(input: &str) -> Result<Vec<u8>, JwtError> {
    BASE64URL_ENGINE
        .decode(input)
        .map_err(|e| JwtError::Base64(format!("Failed to decode base64url: {}", e)))
}

fn parse_json_bytes(bytes: &[u8]) -> Result<Value, JwtError> {
    serde_json::from_slice(bytes)
        .map_err(|e| JwtError::Json(format!("Failed to parse JSON: {}", e)))
}

pub fn is_jwt_format(s: &str) -> bool {
    let parts: Vec<&str> = s.splitn(3, '.').collect();
    if parts.len() < 2 {
        return false;
    }
    let all_base64url = parts.iter().all(|p| {
        p.is_empty()
            || p.chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    });
    all_base64url
}

pub fn parse_token(raw: &str, location: JwtLocation) -> Result<JwtToken, JwtError> {
    let parts: Vec<&str> = raw.splitn(3, '.').collect();
    if parts.len() < 2 {
        return Err(JwtError::MalformedToken(
            "Token does not contain enough sections (need at least header.payload)".into(),
        ));
    }

    let header_raw = parts[0].to_string();
    let payload_raw = parts[1].to_string();
    let signature_raw = if parts.len() >= 3 {
        parts[2].to_string()
    } else {
        String::new()
    };

    let header_bytes = decode_base64url(&header_raw)?;
    let payload_bytes = decode_base64url(&payload_raw)
        .or_else(|_| {
            let padded = if !payload_raw.len().is_multiple_of(4) {
                let mut p = payload_raw.to_string();
                while !p.len().is_multiple_of(4) {
                    p.push('=');
                }
                p
            } else {
                payload_raw.to_string()
            };
            base64::Engine::decode(
                &base64::engine::general_purpose::STANDARD,
                padded.as_bytes(),
            )
            .map_err(|e| JwtError::Base64(format!("Failed to decode payload: {}", e)))
        })
        .or_else(|_| {
            let b64 = payload_raw.replace('-', "+").replace('_', "/");
            let padded = if !b64.len().is_multiple_of(4) {
                let mut p = b64;
                while !p.len().is_multiple_of(4) {
                    p.push('=');
                }
                p
            } else {
                b64
            };
            BASE64URL_ENGINE
                .decode(&padded)
                .map_err(|e| JwtError::Base64(format!("Failed to decode payload: {}", e)))
        })?;

    let header_json = parse_json_bytes(&header_bytes)?;
    let payload_json = parse_json_bytes(&payload_bytes)?;

    let header: JwtHeader = serde_json::from_value(header_json.clone())
        .map_err(|e| JwtError::Json(format!("Failed to deserialize JWT header: {}", e)))?;

    let algorithm = header.algorithm();

    let id = Uuid::new_v4();
    let token = JwtToken {
        id,
        raw: raw.to_string(),
        header_raw,
        payload_raw,
        signature_raw,
        header,
        claims: payload_json,
        algorithm,
        location,
        detected_at: Utc::now(),
        is_valid: true,
        validation_errors: Vec::new(),
    };

    Ok(token)
}

pub fn decode_base64url_payload(payload: &str) -> Result<Value, JwtError> {
    let bytes = decode_base64url(payload)?;
    parse_json_bytes(&bytes)
}

#[cfg(test)]
mod tests {
    use super::super::models::JwtLocationType;
    use super::*;
    use crate::jwt::models::JwtAlgorithm;

    fn test_location() -> JwtLocation {
        JwtLocation {
            location_type: JwtLocationType::AuthorizationHeader,
            key: Some("Authorization".into()),
            url: Some("https://example.com/api".into()),
            host: Some("example.com".into()),
        }
    }

    #[test]
    fn test_is_jwt_format_valid() {
        let token = "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxMjMifQ.abc123";
        assert!(is_jwt_format(token));
    }

    #[test]
    fn test_is_jwt_format_invalid() {
        assert!(!is_jwt_format("not-a-jwt"));
        assert!(!is_jwt_format(""));
    }

    #[test]
    fn test_parse_valid_token() {
        let header = base64::Engine::encode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD,
            r#"{"alg":"HS256","typ":"JWT"}"#,
        );
        let payload = base64::Engine::encode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD,
            r#"{"sub":"123","name":"test"}"#,
        );
        let token_str = format!("{}.{}.sig", header, payload);

        let result = parse_token(&token_str, test_location());
        assert!(result.is_ok());
        let token = result.unwrap();
        assert_eq!(token.algorithm, JwtAlgorithm::HS256);
        assert_eq!(token.claims["sub"], "123");
        assert_eq!(token.claims["name"], "test");
    }

    #[test]
    fn test_parse_none_algorithm() {
        let header = base64::Engine::encode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD,
            r#"{"alg":"none","typ":"JWT"}"#,
        );
        let payload = base64::Engine::encode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD,
            r#"{"sub":"admin"}"#,
        );
        let token_str = format!("{}.{}.", header, payload);

        let result = parse_token(&token_str, test_location());
        assert!(result.is_ok());
        let token = result.unwrap();
        assert_eq!(token.algorithm, JwtAlgorithm::None);
    }

    #[test]
    fn test_parse_malformed_token() {
        let result = parse_token("not-a-jwt", test_location());
        assert!(result.is_err());
    }

    #[test]
    fn test_decode_base64url_payload_json() {
        let encoded = base64::Engine::encode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD,
            r#"{"key":"value"}"#,
        );
        let result = decode_base64url_payload(&encoded);
        assert!(result.is_ok());
        assert_eq!(result.unwrap()["key"], "value");
    }
}
