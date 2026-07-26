#![allow(dead_code)]

use crate::session::models::SessionState;

pub struct ExpirationDetector {
    pub expired_keywords: Vec<String>,
    pub login_redirect_urls: Vec<String>,
    pub cookie_names_to_check: Vec<String>,
}

impl ExpirationDetector {
    pub fn new() -> Self {
        Self {
            expired_keywords: vec![
                "expired".into(),
                "session expired".into(),
                "please log in".into(),
                "unauthorized".into(),
            ],
            login_redirect_urls: vec!["/login".into(), "/signin".into(), "/auth".into()],
            cookie_names_to_check: vec!["session".into(), "token".into()],
        }
    }

    pub fn detect_expiration(
        &self,
        status_code: u16,
        body: &str,
        current_url: &str,
        has_session_cookie: bool,
    ) -> SessionState {
        if status_code == 401 || status_code == 403 {
            return SessionState::Expired;
        }
        if self
            .login_redirect_urls
            .iter()
            .any(|u| current_url.contains(u))
            && !has_session_cookie
        {
            return SessionState::Expired;
        }
        let lower = body.to_lowercase();
        if self
            .expired_keywords
            .iter()
            .any(|k| lower.contains(k.as_str()))
        {
            return SessionState::Expired;
        }
        if !has_session_cookie {
            return SessionState::Expired;
        }
        SessionState::Authenticated
    }

    pub fn is_expired(
        &self,
        status_code: u16,
        body: &str,
        current_url: &str,
        has_cookie: bool,
    ) -> bool {
        matches!(
            self.detect_expiration(status_code, body, current_url, has_cookie),
            SessionState::Expired
        )
    }
}
