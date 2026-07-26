#![allow(dead_code)]

use chrono::Utc;

use crate::auth::models::AuthSession;

pub struct SessionExtractor;

impl SessionExtractor {
    pub fn extract_cookies(_cookies: &str) -> Vec<String> {
        vec![]
    }
    pub fn extract_local_storage(_entries: &str) -> Vec<(String, String)> {
        vec![]
    }
    pub fn extract_session_storage(_entries: &str) -> Vec<(String, String)> {
        vec![]
    }

    pub fn build_session(cookies: &str, local_storage: &str, session_storage: &str) -> AuthSession {
        AuthSession {
            cookies: Self::extract_cookies(cookies),
            local_storage: Self::extract_local_storage(local_storage),
            session_storage: Self::extract_session_storage(session_storage),
            extracted_at: Utc::now(),
        }
    }
}
