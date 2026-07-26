use super::models::{JwtAlgorithm, JwtToken};
use chrono::Utc;

#[derive(Debug, Clone)]
pub struct ValidationPolicy {
    pub require_exp: bool,
    pub require_iat: bool,
    pub require_nbf: bool,
    pub require_sub: bool,
    pub require_iss: bool,
    pub require_aud: bool,
    pub require_jti: bool,
    pub allow_none_algorithm: bool,
    pub allow_weak_algorithms: bool,
    pub max_token_age_seconds: Option<i64>,
    pub max_future_iat_seconds: Option<i64>,
    pub allowed_algorithms: Option<Vec<JwtAlgorithm>>,
}

impl Default for ValidationPolicy {
    fn default() -> Self {
        ValidationPolicy {
            require_exp: true,
            require_iat: false,
            require_nbf: false,
            require_sub: false,
            require_iss: false,
            require_aud: false,
            require_jti: false,
            allow_none_algorithm: false,
            allow_weak_algorithms: false,
            max_token_age_seconds: Some(3600),
            max_future_iat_seconds: Some(300),
            allowed_algorithms: None,
        }
    }
}

impl ValidationPolicy {
    pub fn permissive() -> Self {
        ValidationPolicy {
            require_exp: false,
            require_iat: false,
            require_nbf: false,
            require_sub: false,
            require_iss: false,
            require_aud: false,
            require_jti: false,
            allow_none_algorithm: true,
            allow_weak_algorithms: true,
            max_token_age_seconds: None,
            max_future_iat_seconds: None,
            allowed_algorithms: None,
        }
    }

    pub fn strict() -> Self {
        ValidationPolicy {
            require_exp: true,
            require_iat: true,
            require_nbf: true,
            require_sub: true,
            require_iss: true,
            require_aud: true,
            require_jti: false,
            allow_none_algorithm: false,
            allow_weak_algorithms: false,
            max_token_age_seconds: Some(900),
            max_future_iat_seconds: Some(60),
            allowed_algorithms: Some(vec![
                JwtAlgorithm::RS256,
                JwtAlgorithm::RS512,
                JwtAlgorithm::ES256,
                JwtAlgorithm::ES384,
                JwtAlgorithm::ES512,
            ]),
        }
    }
}

pub fn validate_token(token: &JwtToken, policy: &ValidationPolicy) -> Vec<String> {
    let mut errors = Vec::new();

    if token.algorithm == JwtAlgorithm::None && !policy.allow_none_algorithm {
        errors.push("Algorithm 'none' is not allowed".into());
    }

    if let Some(ref allowed) = policy.allowed_algorithms {
        let algo_allowed = allowed
            .iter()
            .any(|a| std::mem::discriminant(a) == std::mem::discriminant(&token.algorithm));
        if !algo_allowed {
            errors.push(format!(
                "Algorithm '{}' is not in the allowed list",
                token.algorithm
            ));
        }
    }

    if !policy.allow_weak_algorithms && token.algorithm.is_weak() {
        errors.push(format!(
            "Weak algorithm '{}' is not allowed",
            token.algorithm
        ));
    }

    if let Some(claims) = token.claims.as_object() {
        if policy.require_exp && !claims.contains_key("exp") {
            errors.push("Missing required claim: exp".into());
        }
        if policy.require_iat && !claims.contains_key("iat") {
            errors.push("Missing required claim: iat".into());
        }
        if policy.require_nbf && !claims.contains_key("nbf") {
            errors.push("Missing required claim: nbf".into());
        }
        if policy.require_sub && !claims.contains_key("sub") {
            errors.push("Missing required claim: sub".into());
        }
        if policy.require_iss && !claims.contains_key("iss") {
            errors.push("Missing required claim: iss".into());
        }
        if policy.require_aud && !claims.contains_key("aud") {
            errors.push("Missing required claim: aud".into());
        }
        if policy.require_jti && !claims.contains_key("jti") {
            errors.push("Missing required claim: jti".into());
        }
    }

    let now = Utc::now().timestamp();

    if let Some(claims) = token.claims.as_object() {
        if let Some(exp) = claims.get("exp").and_then(|v| v.as_i64()) {
            if now > exp {
                errors.push(format!(
                    "Token expired at {} ({} seconds ago)",
                    exp,
                    now - exp
                ));
            }
        }

        if let Some(nbf) = claims.get("nbf").and_then(|v| v.as_i64()) {
            if now < nbf {
                errors.push(format!(
                    "Token not yet valid until {} ({} seconds from now)",
                    nbf,
                    nbf - now
                ));
            }
        }

        if let Some(iat) = claims.get("iat").and_then(|v| v.as_i64()) {
            if let Some(max_age) = policy.max_token_age_seconds {
                if now - iat > max_age {
                    errors.push(format!(
                        "Token age {}s exceeds maximum allowed {}s",
                        now - iat,
                        max_age
                    ));
                }
            }
            if let Some(max_future) = policy.max_future_iat_seconds {
                if iat - now > max_future {
                    errors.push(format!(
                        "Token issued at is {}s in the future, exceeding max {}s",
                        iat - now,
                        max_future
                    ));
                }
            }
        }
    }

    errors
}

pub fn validate_token_mut(token: &mut JwtToken, policy: &ValidationPolicy) {
    let errors = validate_token(token, policy);
    token.is_valid = errors.is_empty();
    token.validation_errors = errors;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jwt::models::{JwtLocation, JwtLocationType};
    use base64::Engine;

    fn make_token(header_json: &str, payload_json: &str) -> JwtToken {
        let header = base64::Engine::encode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD,
            header_json,
        );
        let payload = base64::Engine::encode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD,
            payload_json,
        );
        let raw = format!("{}.{}.sig", header, payload);
        crate::jwt::parser::parse_token(
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
    fn test_reject_none_algorithm() {
        let token = make_token(r#"{"alg":"none"}"#, r#"{"sub":"test"}"#);
        let policy = ValidationPolicy::default();
        let errors = validate_token(&token, &policy);
        assert!(errors.iter().any(|e| e.contains("none")));
    }

    #[test]
    fn test_require_exp() {
        let token = make_token(r#"{"alg":"HS256"}"#, r#"{"sub":"test"}"#);
        let policy = ValidationPolicy::default();
        let errors = validate_token(&token, &policy);
        assert!(errors.iter().any(|e| e.contains("exp")));
    }

    #[test]
    fn test_permissive_policy() {
        let token = make_token(r#"{"alg":"none"}"#, r#"{"sub":"test"}"#);
        let policy = ValidationPolicy::permissive();
        let errors = validate_token(&token, &policy);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_expired_token() {
        let token = make_token(r#"{"alg":"HS256"}"#, r#"{"sub":"test","exp":1000000000}"#);
        let policy = ValidationPolicy {
            require_exp: false,
            ..ValidationPolicy::default()
        };
        let errors = validate_token(&token, &policy);
        assert!(errors.iter().any(|e| e.contains("expired")));
    }
}
