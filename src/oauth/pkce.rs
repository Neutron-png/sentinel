use sha2::{Digest, Sha256};

use super::models::OAuthSession;

pub fn verify_pkce(code_verifier: &str, code_challenge: &str, method: &str) -> bool {
    match method.to_uppercase().as_str() {
        "S256" => {
            let mut hasher = Sha256::new();
            hasher.update(code_verifier.as_bytes());
            let hash = hasher.finalize();
            let computed =
                base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, hash);
            computed == code_challenge
        }
        "PLAIN" => code_verifier == code_challenge,
        _ => false,
    }
}

pub fn has_pkce_params(session: &OAuthSession) -> bool {
    session.code_challenge.is_some()
}

pub fn validate_pkce_verifier(code_verifier: &str) -> Result<(), String> {
    if code_verifier.len() < 43 || code_verifier.len() > 128 {
        return Err(format!(
            "code_verifier length {} must be between 43 and 128 characters",
            code_verifier.len()
        ));
    }
    if !code_verifier
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '.' || c == '_' || c == '~')
    {
        return Err(
            "code_verifier contains invalid characters (allowed: A-Z, a-z, 0-9, -, ., _, ~)"
                .to_string(),
        );
    }
    Ok(())
}

pub fn generate_code_challenge(code_verifier: &str) -> (String, String) {
    let mut hasher = Sha256::new();
    hasher.update(code_verifier.as_bytes());
    let hash = hasher.finalize();
    let challenge = base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, hash);
    (challenge, "S256".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_pkce_s256() {
        let verifier = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";
        let challenge = "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM";
        assert!(verify_pkce(verifier, challenge, "S256"));
    }

    #[test]
    fn test_verify_pkce_wrong_method() {
        assert!(!verify_pkce("test", "test", "Unknown"));
    }

    #[test]
    fn test_verify_pkce_plain() {
        assert!(verify_pkce("test_verifier", "test_verifier", "plain"));
    }

    #[test]
    fn test_verify_pkce_mismatch() {
        assert!(!verify_pkce("wrong", "challenge", "S256"));
    }

    #[test]
    fn test_validate_verifier_length() {
        assert!(validate_pkce_verifier("short").is_err());
    }

    #[test]
    fn test_generate_code_challenge() {
        let verifier = "valid_verifier_string_minimum_length_43_chars_";
        let (challenge, method) = generate_code_challenge(verifier);
        assert_eq!(method, "S256");
        assert!(verify_pkce(verifier, &challenge, "S256"));
    }
}
