use base64::Engine;
use serde_json::Value;

use super::errors::JwtError;
use super::models::{JwtEditResult, JwtToken};

const B64_ENGINE: base64::engine::GeneralPurpose = base64::engine::GeneralPurpose::new(
    &base64::alphabet::URL_SAFE,
    base64::engine::general_purpose::NO_PAD,
);

pub fn edit_header(token: &JwtToken, new_header: &Value) -> Result<String, JwtError> {
    let header_str = serde_json::to_string(new_header)
        .map_err(|e| JwtError::Editing(format!("Failed to serialize header: {}", e)))?;
    let header_b64 = B64_ENGINE.encode(header_str.as_bytes());
    Ok(format!(
        "{}.{}.{}",
        header_b64, token.payload_raw, token.signature_raw
    ))
}

pub fn edit_claims(token: &JwtToken, new_claims: &Value) -> Result<String, JwtError> {
    let claims_str = serde_json::to_string(new_claims)
        .map_err(|e| JwtError::Editing(format!("Failed to serialize claims: {}", e)))?;
    let claims_b64 = B64_ENGINE.encode(claims_str.as_bytes());
    Ok(format!(
        "{}.{}.{}",
        token.header_raw, claims_b64, token.signature_raw
    ))
}

pub fn add_claim(token: &JwtToken, key: &str, value: &Value) -> Result<String, JwtError> {
    let mut claims = token.claims.clone();
    if let Some(obj) = claims.as_object_mut() {
        obj.insert(key.to_string(), value.clone());
    } else {
        return Err(JwtError::Editing(
            "Claims payload is not a JSON object".into(),
        ));
    }
    edit_claims(token, &claims)
}

pub fn remove_claim(token: &JwtToken, key: &str) -> Result<String, JwtError> {
    let mut claims = token.claims.clone();
    if let Some(obj) = claims.as_object_mut() {
        obj.remove(key);
    } else {
        return Err(JwtError::Editing(
            "Claims payload is not a JSON object".into(),
        ));
    }
    edit_claims(token, &claims)
}

pub fn set_algorithm(token: &JwtToken, algorithm: &str) -> Result<String, JwtError> {
    let mut header = serde_json::to_value(&token.header)
        .map_err(|e| JwtError::Editing(format!("Failed to serialize header: {}", e)))?;
    if let Some(obj) = header.as_object_mut() {
        obj.insert("alg".to_string(), Value::String(algorithm.to_string()));
    }
    edit_header(token, &header)
}

pub fn strip_signature(token: &JwtToken) -> String {
    format!("{}.{}.", token.header_raw, token.payload_raw)
}

pub fn rebuild_token(
    header_json: &str,
    payload_json: &str,
    signature: Option<&str>,
) -> Result<String, JwtError> {
    let parsed_header: Value = serde_json::from_str(header_json)
        .map_err(|e| JwtError::Json(format!("Invalid header JSON: {}", e)))?;
    let parsed_payload: Value = serde_json::from_str(payload_json)
        .map_err(|e| JwtError::Json(format!("Invalid payload JSON: {}", e)))?;

    let header_encoded = B64_ENGINE.encode(
        serde_json::to_string(&parsed_header)
            .map_err(|e| JwtError::Editing(format!("Failed to serialize header: {}", e)))?
            .as_bytes(),
    );
    let payload_encoded = B64_ENGINE.encode(
        serde_json::to_string(&parsed_payload)
            .map_err(|e| JwtError::Editing(format!("Failed to serialize payload: {}", e)))?
            .as_bytes(),
    );

    Ok(format!(
        "{}.{}.{}",
        header_encoded,
        payload_encoded,
        signature.unwrap_or("")
    ))
}

pub fn preview_edit(token: &JwtToken, new_claims: &Value) -> JwtEditResult {
    let modified = edit_claims(token, new_claims).unwrap_or_else(|_| token.raw.clone());
    let claims_str = serde_json::to_string(new_claims).unwrap_or_default();
    let payload_b64 = B64_ENGINE.encode(claims_str.as_bytes());
    JwtEditResult {
        original: token.raw.clone(),
        modified,
        header_raw: token.header_raw.clone(),
        payload_raw: payload_b64,
        claims: new_claims.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jwt::models::{JwtLocation, JwtLocationType};
    use crate::jwt::parser::parse_token;
    use base64::Engine;

    fn sample_token() -> JwtToken {
        let header = base64::Engine::encode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD,
            r#"{"alg":"HS256","typ":"JWT"}"#,
        );
        let payload = base64::Engine::encode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD,
            r#"{"sub":"123","name":"test"}"#,
        );
        let raw = format!("{}.{}.sig", header, payload);
        parse_token(
            &raw,
            JwtLocation {
                location_type: JwtLocationType::AuthorizationHeader,
                key: None,
                url: None,
                host: None,
            },
        )
        .unwrap()
    }

    #[test]
    fn test_add_claim() {
        let token = sample_token();
        let result = add_claim(&token, "role", &Value::String("admin".into()));
        assert!(result.is_ok());
        let new_token = result.unwrap();
        let parsed = parse_token(
            &new_token,
            JwtLocation {
                location_type: JwtLocationType::AuthorizationHeader,
                key: None,
                url: None,
                host: None,
            },
        )
        .unwrap();
        assert_eq!(parsed.claims["role"], "admin");
        assert_eq!(parsed.claims["sub"], "123");
    }

    #[test]
    fn test_remove_claim() {
        let token = sample_token();
        let result = remove_claim(&token, "name");
        assert!(result.is_ok());
        let new_token = result.unwrap();
        let parsed = parse_token(
            &new_token,
            JwtLocation {
                location_type: JwtLocationType::AuthorizationHeader,
                key: None,
                url: None,
                host: None,
            },
        )
        .unwrap();
        assert!(!parsed.claims.as_object().unwrap().contains_key("name"));
    }

    #[test]
    fn test_set_algorithm() {
        let token = sample_token();
        let result = set_algorithm(&token, "none");
        assert!(result.is_ok());
        let new_token = result.unwrap();
        let parsed = parse_token(
            &new_token,
            JwtLocation {
                location_type: JwtLocationType::AuthorizationHeader,
                key: None,
                url: None,
                host: None,
            },
        )
        .unwrap();
        assert_eq!(parsed.algorithm, crate::jwt::models::JwtAlgorithm::None);
    }

    #[test]
    fn test_strip_signature() {
        let token = sample_token();
        let result = strip_signature(&token);
        assert!(result.ends_with('.'));
        assert_eq!(result.matches('.').count(), 2);
    }
}
