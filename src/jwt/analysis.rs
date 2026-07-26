use super::models::{JwtAlgorithm, JwtAnalysisFinding, JwtToken};

pub fn analyze_token(token: &JwtToken) -> Vec<JwtAnalysisFinding> {
    let mut findings = Vec::new();

    findings.extend(check_none_algorithm(token));
    findings.extend(check_missing_exp(token));
    findings.extend(check_expired(token));
    findings.extend(check_long_expiration(token));
    findings.extend(check_sensitive_claims(token));
    findings.extend(check_weak_algorithm(token));
    findings.extend(check_missing_kid(token));
    findings.extend(check_duplicate_claims(token));

    findings
}

fn check_none_algorithm(token: &JwtToken) -> Option<JwtAnalysisFinding> {
    if token.algorithm == JwtAlgorithm::None {
        Some(JwtAnalysisFinding {
            title: "JWT uses 'none' algorithm".into(),
            description: format!(
                "The JWT at '{}' uses the 'none' algorithm, which means no signature verification is performed. Attackers can modify token claims without detection.",
                token.location.location_type.label()
            ),
            severity: "Critical".into(),
            confidence: "High".into(),
            recommendation: "Switch to a secure signing algorithm such as RS256 or ES256. Never use 'none' in production.".into(),
            cwe: Some("CWE-345".into()),
            owasp_category: Some("A02:2021 - Cryptographic Failures".into()),
        })
    } else {
        None
    }
}

fn check_missing_exp(token: &JwtToken) -> Option<JwtAnalysisFinding> {
    let has_exp = token
        .claims
        .as_object()
        .map(|o| o.contains_key("exp"))
        .unwrap_or(false);

    if !has_exp {
        Some(JwtAnalysisFinding {
            title: "JWT missing expiration claim".into(),
            description: format!(
                "The JWT at '{}' does not contain an 'exp' claim, which means the token will never expire. This increases the impact of token theft.",
                token.location.location_type.label()
            ),
            severity: "High".into(),
            confidence: "High".into(),
            recommendation: "Add an 'exp' (expiration) claim with a reasonable lifetime (e.g., 15-60 minutes for access tokens).".into(),
            cwe: Some("CWE-613".into()),
            owasp_category: Some("A07:2021 - Identification and Authentication Failures".into()),
        })
    } else {
        None
    }
}

fn check_expired(token: &JwtToken) -> Option<JwtAnalysisFinding> {
    if let Some(is_expired) = token.is_expired() {
        if is_expired {
            return Some(JwtAnalysisFinding {
                title: "JWT token is expired".into(),
                description: format!(
                    "The JWT at '{}' has expired but is still being used or accepted by the server. The server may not be properly validating token expiration.",
                    token.location.location_type.label()
                ),
                severity: "Medium".into(),
                confidence: "High".into(),
                recommendation: "Verify that the server validates the 'exp' claim. If an expired token is accepted, this indicates a server-side validation flaw.".into(),
                cwe: Some("CWE-613".into()),
                owasp_category: Some("A07:2021 - Identification and Authentication Failures".into()),
            });
        }
    }
    None
}

fn check_long_expiration(token: &JwtToken) -> Option<JwtAnalysisFinding> {
    if let Some(claims) = token.claims.as_object() {
        if let (Some(exp), Some(iat)) = (
            claims.get("exp").and_then(|v| v.as_i64()),
            claims.get("iat").and_then(|v| v.as_i64()),
        ) {
            let lifetime = exp - iat;
            let one_month = 30 * 24 * 3600;
            if lifetime > one_month {
                return Some(JwtAnalysisFinding {
                    title: "JWT has excessively long expiration".into(),
                    description: format!(
                        "The JWT at '{}' has a lifetime of {} days ({} seconds). Long-lived tokens increase the window of opportunity for token theft and replay attacks.",
                        token.location.location_type.label(),
                        lifetime / 86400,
                        lifetime
                    ),
                    severity: "Medium".into(),
                    confidence: "High".into(),
                    recommendation: "Use short-lived access tokens (15-60 minutes) combined with refresh tokens for longer sessions.".into(),
                    cwe: Some("CWE-613".into()),
                    owasp_category: Some("A07:2021 - Identification and Authentication Failures".into()),
                });
            }
        }
    }
    None
}

fn check_sensitive_claims(token: &JwtToken) -> Vec<JwtAnalysisFinding> {
    let mut findings = Vec::new();
    let sensitive_keys = [
        "password",
        "secret",
        "api_key",
        "apikey",
        "credit_card",
        "ssn",
        "token",
        "private_key",
        "access_key",
        "secret_key",
    ];

    if let Some(claims) = token.claims.as_object() {
        for key in &sensitive_keys {
            if claims.contains_key(*key) || claims.keys().any(|k| k.to_lowercase().contains(key)) {
                findings.push(JwtAnalysisFinding {
                    title: format!("JWT contains sensitive claim: '{}'", key),
                    description: format!(
                        "The JWT at '{}' contains a potentially sensitive claim matching '{}'. JWTs are base64url-encoded, NOT encrypted. Anyone who captures the token can decode and read all claims.",
                        token.location.location_type.label(),
                        key
                    ),
                    severity: "High".into(),
                    confidence: "Medium".into(),
                    recommendation: "Do not store sensitive data in JWT claims. Use opaque references or server-side storage instead. Consider using JWE (encrypted tokens) if sensitive data must be included.".into(),
                    cwe: Some("CWE-312".into()),
                    owasp_category: Some("A04:2021 - Insecure Design".into()),
                });
            }
        }
    }
    findings
}

fn check_weak_algorithm(token: &JwtToken) -> Option<JwtAnalysisFinding> {
    if token.algorithm == JwtAlgorithm::HS256 || token.algorithm == JwtAlgorithm::RS256 {
        return Some(JwtAnalysisFinding {
            title: format!("JWT uses weak algorithm: {}", token.algorithm),
            description: format!(
                "The JWT at '{}' uses {}, which is considered weak. This algorithm may be susceptible to brute-force attacks or algorithm confusion.",
                token.location.location_type.label(),
                token.algorithm
            ),
            severity: "Medium".into(),
            confidence: "Medium".into(),
            recommendation: "Upgrade to HS384/HS512 for HMAC or RS384/RS512/ES256+ for asymmetric signing.".into(),
            cwe: Some("CWE-327".into()),
            owasp_category: Some("A02:2021 - Cryptographic Failures".into()),
        });
    }
    None
}

fn check_missing_kid(token: &JwtToken) -> Option<JwtAnalysisFinding> {
    let uses_asymmetric = token.algorithm.is_asymmetric();
    let has_kid = token.header.kid.is_some();

    if uses_asymmetric && !has_kid {
        Some(JwtAnalysisFinding {
            title: "Asymmetric JWT missing Key ID (kid)".into(),
            description: format!(
                "The JWT at '{}' uses an asymmetric algorithm ({}) but does not include a 'kid' header. Without a key identifier, the server may accept tokens signed with any key in its keystore, potentially enabling algorithm confusion attacks.",
                token.location.location_type.label(),
                token.algorithm
            ),
            severity: "Low".into(),
            confidence: "Medium".into(),
            recommendation: "Include a 'kid' (Key ID) header to specify which key was used for signing. This prevents key confusion attacks.".into(),
            cwe: None,
            owasp_category: Some("A02:2021 - Cryptographic Failures".into()),
        })
    } else {
        None
    }
}

fn check_duplicate_claims(_token: &JwtToken) -> Option<JwtAnalysisFinding> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jwt::models::{JwtLocation, JwtLocationType};
    use crate::jwt::parser::parse_token;
    use base64::Engine;

    fn make_token(alg: &str, exp: Option<i64>, iat: Option<i64>) -> JwtToken {
        let header_json = format!(r#"{{"alg":"{}"}}"#, alg);
        let payload_json = match (exp, iat) {
            (Some(e), Some(i)) => {
                format!(r#"{{"sub":"test","exp":{},"iat":{}}}"#, e, i)
            }
            (Some(e), None) => {
                format!(r#"{{"sub":"test","exp":{}}}"#, e)
            }
            (None, Some(i)) => {
                format!(r#"{{"sub":"test","iat":{}}}"#, i)
            }
            (None, None) => r#"{"sub":"test"}"#.to_string(),
        };

        let header = base64::Engine::encode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD,
            &header_json,
        );
        let payload = base64::Engine::encode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD,
            &payload_json,
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
    fn test_none_algorithm_detection() {
        let token = make_token("none", None, None);
        let findings = analyze_token(&token);
        assert!(findings.iter().any(|f| f.title.contains("none")));
    }

    #[test]
    fn test_missing_exp_detection() {
        let token = make_token("HS256", None, None);
        let findings = analyze_token(&token);
        assert!(findings.iter().any(|f| f.title.contains("expiration")));
    }

    #[test]
    fn test_expired_detection() {
        let token = make_token("HS256", Some(1000000000), None);
        let findings = analyze_token(&token);
        assert!(findings.iter().any(|f| f.title.contains("expired")));
    }

    #[test]
    fn test_weak_algorithm_detection() {
        let token = make_token("HS256", Some(2000000000), None);
        let findings = analyze_token(&token);
        assert!(findings.iter().any(|f| f.title.contains("weak algorithm")));
    }

    #[test]
    fn test_sensitive_claim_detection() {
        let header = base64::Engine::encode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD,
            r#"{"alg":"HS256"}"#,
        );
        let payload = base64::Engine::encode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD,
            r#"{"sub":"test","password":"secret123"}"#,
        );
        let raw = format!("{}.{}.sig", header, payload);
        let token = parse_token(
            &raw,
            JwtLocation {
                location_type: JwtLocationType::Cookie,
                key: None,
                url: None,
                host: None,
            },
        )
        .unwrap();
        let findings = analyze_token(&token);
        assert!(findings.iter().any(|f| f.title.contains("password")));
    }
}
