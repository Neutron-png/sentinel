#![allow(dead_code)]

use crate::auth::models::AuthState;

pub struct AuthDetector {
    pub known_logout_urls: Vec<String>,
    pub known_login_urls: Vec<String>,
    pub session_cookie_names: Vec<String>,
}

impl AuthDetector {
    pub fn new() -> Self {
        Self {
            known_logout_urls: vec!["/logout".into(), "/signout".into()],
            known_login_urls: vec!["/login".into(), "/signin".into()],
            session_cookie_names: vec!["session".into(), "JSESSIONID".into(), "PHPSESSID".into()],
        }
    }

    pub fn detect_state(
        &self,
        current_url: &str,
        has_session_cookie: bool,
        page_contains_login: bool,
    ) -> AuthState {
        if self
            .known_logout_urls
            .iter()
            .any(|u| current_url.contains(u))
        {
            return AuthState::LoggedOut;
        }
        if !has_session_cookie
            && self
                .known_login_urls
                .iter()
                .any(|u| current_url.contains(u))
            && page_contains_login
        {
            return AuthState::Unknown;
        }
        if has_session_cookie && !page_contains_login {
            return AuthState::Authenticated;
        }
        if !has_session_cookie {
            return AuthState::Expired;
        }
        AuthState::Unknown
    }

    pub fn is_authenticated(&self, current_url: &str, has_session_cookie: bool) -> bool {
        matches!(
            self.detect_state(current_url, has_session_cookie, false),
            AuthState::Authenticated
        )
    }

    pub fn is_login_page(&self, url: &str) -> bool {
        self.known_login_urls.iter().any(|u| url.contains(u))
    }

    pub fn is_logout_page(&self, url: &str) -> bool {
        self.known_logout_urls.iter().any(|u| url.contains(u))
    }
}
