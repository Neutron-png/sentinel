use super::models::{OAuthAnalysisFinding, OAuthFlowType, OAuthSession, OidcMetadata};

pub fn analyze_session(session: &OAuthSession) -> Vec<OAuthAnalysisFinding> {
    let mut findings = Vec::new();

    findings.extend(check_missing_pkce(session));
    findings.extend(check_legacy_flow(session));
    findings.extend(check_missing_state(session));
    findings.extend(check_missing_nonce(session));
    findings.extend(check_wildcard_redirect(session));
    findings.extend(check_exposed_client_secret(session));
    findings.extend(check_long_lived_token(session));
    findings.extend(check_insecure_scopes(session));
    findings.extend(check_implicit_flow(session));

    findings
}

pub fn analyze_oidc_metadata(metadata: &OidcMetadata) -> Vec<OAuthAnalysisFinding> {
    let mut findings = Vec::new();

    if !metadata.supports_pkce() {
        findings.push(OAuthAnalysisFinding {
            title: "OIDC Provider does not support PKCE".into(),
            description: format!(
                "The OIDC provider at '{}' does not advertise PKCE (S256) support. PKCE is critical for preventing authorization code interception attacks in public clients.",
                metadata.issuer
            ),
            severity: "High".into(),
            confidence: "High".into(),
            recommendation: "Ensure the OIDC provider enables PKCE support with S256 as the code challenge method. RFC 7636 recommends PKCE for all OAuth clients.".into(),
            cwe: Some("CWE-384".into()),
            owasp_category: Some("A07:2021 - Identification and Authentication Failures".into()),
            endpoint: Some(metadata.issuer.clone()),
            evidence: format!("Discovery document at {}/.well-known/openid-configuration", metadata.issuer),
        });
    }

    if metadata
        .grant_types_supported
        .contains(&"implicit".to_string())
    {
        findings.push(OAuthAnalysisFinding {
            title: "OIDC Provider supports deprecated Implicit flow".into(),
            description: format!(
                "The OIDC provider at '{}' lists the implicit flow as a supported grant type. The implicit flow is deprecated and should not be used due to token leakage risks.",
                metadata.issuer
            ),
            severity: "Medium".into(),
            confidence: "High".into(),
            recommendation: "Disable the implicit grant flow and migrate to Authorization Code with PKCE instead. OAuth 2.1 removes the implicit flow entirely.".into(),
            cwe: Some("CWE-359".into()),
            owasp_category: Some("A07:2021 - Identification and Authentication Failures".into()),
            endpoint: Some(metadata.issuer.clone()),
            evidence: "grant_types_supported includes 'implicit' in discovery document".to_string(),
        });
    }

    findings
}

fn check_missing_pkce(session: &OAuthSession) -> Option<OAuthAnalysisFinding> {
    if session.flow_type == OAuthFlowType::AuthorizationCode && !session.has_pkce() {
        Some(OAuthAnalysisFinding {
            title: "Authorization Code flow without PKCE".into(),
            description: "The OAuth authorization code flow was detected without PKCE (Proof Key for Code Exchange). Without PKCE, authorization codes can be intercepted and misused, especially in mobile and single-page applications.".into(),
            severity: "High".into(),
            confidence: "High".into(),
            recommendation: "Implement PKCE (RFC 7636) with S256 as the code challenge method for all authorization code flows, especially for public clients (SPAs, mobile apps).".into(),
            cwe: Some("CWE-384".into()),
            owasp_category: Some("A07:2021 - Identification and Authentication Failures".into()),
            endpoint: session.authorization_endpoint.clone(),
            evidence: format!("Detected authorization code flow without code_challenge or code_verifier parameters. Client: {}", session.client_id.as_deref().unwrap_or("unknown")),
        })
    } else {
        None
    }
}

fn check_legacy_flow(session: &OAuthSession) -> Option<OAuthAnalysisFinding> {
    if session.flow_type.is_legacy() {
        Some(OAuthAnalysisFinding {
            title: format!("Legacy OAuth flow detected: {}", session.flow_type.label()),
            description: format!(
                "The {} flow was detected. This flow type is deprecated by OAuth 2.1 due to security concerns including token leakage and lack of client authentication.",
                session.flow_type.label()
            ),
            severity: "Medium".into(),
            confidence: "High".into(),
            recommendation: "Migrate to the Authorization Code flow with PKCE for public clients, or Client Credentials for server-to-server communication. OAuth 2.1 removes Implicit and Resource Owner Password flows.".into(),
            cwe: Some("CWE-359".into()),
            owasp_category: Some("A07:2021 - Identification and Authentication Failures".into()),
            endpoint: session.token_endpoint.clone().or_else(|| session.authorization_endpoint.clone()),
            evidence: format!("Detected {} OAuth flow. Client: {}", session.flow_type.label(), session.client_id.as_deref().unwrap_or("unknown")),
        })
    } else {
        None
    }
}

fn check_missing_state(session: &OAuthSession) -> Option<OAuthAnalysisFinding> {
    if session.state.is_none()
        && (session.flow_type == OAuthFlowType::AuthorizationCode
            || session.flow_type == OAuthFlowType::AuthorizationCodePkce)
    {
        Some(OAuthAnalysisFinding {
            title: "OAuth authorization request missing state parameter".into(),
            description: "The OAuth authorization request does not include a 'state' parameter. The state parameter prevents CSRF attacks by binding the authorization request to the user's session. Without it, attackers can trick users into authorizing malicious clients.".into(),
            severity: "Medium".into(),
            confidence: "High".into(),
            recommendation: "Always include a cryptographically random 'state' parameter in authorization requests and validate it upon callback. The state should be unguessable and tied to the user's session.".into(),
            cwe: Some("CWE-352".into()),
            owasp_category: Some("A07:2021 - Identification and Authentication Failures".into()),
            endpoint: session.authorization_endpoint.clone(),
            evidence: "Authorization request detected without 'state' query parameter.".into(),
        })
    } else {
        None
    }
}

fn check_missing_nonce(session: &OAuthSession) -> Option<OAuthAnalysisFinding> {
    if session.nonce.is_none() && session.id_token.is_some() {
        Some(OAuthAnalysisFinding {
            title: "OIDC authorization request missing nonce parameter".into(),
            description: "An OpenID Connect authorization with ID token was detected without a 'nonce' parameter. The nonce parameter mitigates replay attacks on ID tokens by binding the token to the original authorization request.".into(),
            severity: "Medium".into(),
            confidence: "Medium".into(),
            recommendation: "Include a 'nonce' parameter in OIDC authorization requests and verify it matches the 'nonce' claim in the returned ID token. This prevents token replay attacks.".into(),
            cwe: Some("CWE-294".into()),
            owasp_category: Some("A07:2021 - Identification and Authentication Failures".into()),
            endpoint: session.authorization_endpoint.clone(),
            evidence: "ID token detected without corresponding nonce in authorization request.".into(),
        })
    } else {
        None
    }
}

fn check_wildcard_redirect(session: &OAuthSession) -> Option<OAuthAnalysisFinding> {
    let uri = session.redirect_uri.as_deref()?;
    if uri.contains("*") || uri.contains("localhost") || uri.starts_with("http://") {
        Some(OAuthAnalysisFinding {
            title: "Potentially insecure redirect URI detected".into(),
            description: format!(
                "The redirect URI '{}' may be insecure. Wildcards, localhost URIs, and non-HTTPS URIs can be exploited by attackers to intercept authorization codes or tokens. Redirect URIs must be validated against a strict allowlist.",
                uri
            ),
            severity: "High".into(),
            confidence: "Medium".into(),
            recommendation: "Use exact-match HTTPS redirect URIs with no wildcards. For native apps, use claimed HTTPS schemes or app-specific custom schemes with PKCE. Never use wildcard patterns in production.".into(),
            cwe: Some("CWE-601".into()),
            owasp_category: Some("A07:2021 - Identification and Authentication Failures".into()),
            endpoint: Some(uri.to_string()),
            evidence: format!("Redirect URI: {}", uri),
        })
    } else {
        None
    }
}

fn check_exposed_client_secret(_session: &OAuthSession) -> Option<OAuthAnalysisFinding> {
    None
}

fn check_long_lived_token(session: &OAuthSession) -> Option<OAuthAnalysisFinding> {
    if let Some(expires_in) = session.expires_in {
        let _one_hour = 3600;
        let twenty_four_hours = 86400;

        if expires_in > twenty_four_hours {
            return Some(OAuthAnalysisFinding {
                title: "OAuth access token with excessive lifetime".into(),
                description: format!(
                    "The OAuth access token has a lifetime of {} seconds ({} hours). Long-lived access tokens increase the window of opportunity for token theft and replay attacks.",
                    expires_in,
                    expires_in / 3600
                ),
                severity: "Medium".into(),
                confidence: "High".into(),
                recommendation: "Set access token lifetime to a maximum of 1 hour (3600 seconds). Use refresh tokens with rotation for longer sessions. This limits the impact of token compromise.".into(),
                cwe: Some("CWE-613".into()),
                owasp_category: Some("A07:2021 - Identification and Authentication Failures".into()),
                endpoint: session.token_endpoint.clone(),
                evidence: format!("Access token expires_in: {} seconds from token response", expires_in),
            });
        }
    }
    None
}

fn check_insecure_scopes(session: &OAuthSession) -> Option<OAuthAnalysisFinding> {
    if session
        .scopes
        .iter()
        .any(|s| s == "admin" || s == "administrator" || s == "root" || s == "*")
    {
        Some(OAuthAnalysisFinding {
            title: "Excessive or privileged OAuth scopes requested".into(),
            description: format!(
                "The OAuth session requests privileged scopes: {}. Overly broad scopes grant excessive permissions to clients, violating the principle of least privilege. If the token is compromised, the attacker gains full administrative access.",
                session.scopes.join(", ")
            ),
            severity: "High".into(),
            confidence: "High".into(),
            recommendation: "Limit OAuth scopes to the minimum required for the application's functionality. Avoid wildcard (*) scopes and administrative scopes. Implement scope validation on the resource server.".into(),
            cwe: Some("CWE-285".into()),
            owasp_category: Some("A01:2021 - Broken Access Control".into()),
            endpoint: session.authorization_endpoint.clone(),
            evidence: format!("Requested scopes: {}", session.scopes.join(", ")),
        })
    } else {
        None
    }
}

fn check_implicit_flow(session: &OAuthSession) -> Option<OAuthAnalysisFinding> {
    if session.flow_type == OAuthFlowType::Implicit {
        Some(OAuthAnalysisFinding {
            title: "Implicit flow detected - tokens exposed in URL fragment".into(),
            description: "The Implicit OAuth flow returns access tokens in the URL fragment, which can be exposed through browser history, referrer headers, and JavaScript. This flow is deprecated by OAuth 2.1 due to inherent security risks.".into(),
            severity: "High".into(),
            confidence: "High".into(),
            recommendation: "Migrate to the Authorization Code flow with PKCE. If the application is a SPA, use a backend-for-frontend (BFF) pattern or a secure token handler to keep tokens out of the browser.".into(),
            cwe: Some("CWE-359".into()),
            owasp_category: Some("A07:2021 - Identification and Authentication Failures".into()),
            endpoint: session.authorization_endpoint.clone(),
            evidence: "Implicit flow OAuth session detected with access token in URL/response.".into(),
        })
    } else {
        None
    }
}
