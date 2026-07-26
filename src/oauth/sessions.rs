use chrono::Utc;
use std::collections::HashMap;
use uuid::Uuid;

use super::models::{OAuthFlowType, OAuthSession};

pub struct OAuthSessionTracker {
    sessions: HashMap<Uuid, OAuthSession>,
    sessions_by_client: HashMap<String, Vec<Uuid>>,
    sessions_by_endpoint: HashMap<String, Vec<Uuid>>,
}

impl OAuthSessionTracker {
    pub fn new() -> Self {
        OAuthSessionTracker {
            sessions: HashMap::new(),
            sessions_by_client: HashMap::new(),
            sessions_by_endpoint: HashMap::new(),
        }
    }

    pub fn track_session(&mut self, session: OAuthSession) -> Uuid {
        let id = session.id;

        if let Some(ref client_id) = session.client_id {
            self.sessions_by_client
                .entry(client_id.clone())
                .or_default()
                .push(id);
        }

        if let Some(ref endpoint) = session.authorization_endpoint {
            self.sessions_by_endpoint
                .entry(endpoint.clone())
                .or_default()
                .push(id);
        }
        if let Some(ref endpoint) = session.token_endpoint {
            self.sessions_by_endpoint
                .entry(endpoint.clone())
                .or_default()
                .push(id);
        }

        self.sessions.insert(id, session);
        id
    }

    pub fn update_session(&mut self, id: Uuid, session: OAuthSession) {
        let mut s = session;
        s.touch();
        self.sessions.insert(id, s);
    }

    pub fn get_session(&self, id: Uuid) -> Option<&OAuthSession> {
        self.sessions.get(&id)
    }

    pub fn get_session_mut(&mut self, id: Uuid) -> Option<&mut OAuthSession> {
        self.sessions.get_mut(&id)
    }

    pub fn get_sessions_by_client(&self, client_id: &str) -> Vec<&OAuthSession> {
        self.sessions_by_client
            .get(client_id)
            .map(|ids| ids.iter().filter_map(|id| self.sessions.get(id)).collect())
            .unwrap_or_default()
    }

    pub fn get_active_sessions(&self) -> Vec<&OAuthSession> {
        self.sessions.values().filter(|s| s.is_active).collect()
    }

    pub fn get_authorization_sessions(&self) -> Vec<&OAuthSession> {
        self.sessions
            .values()
            .filter(|s| s.authorization_endpoint.is_some() && s.authorization_code.is_some())
            .collect()
    }

    pub fn get_token_sessions(&self) -> Vec<&OAuthSession> {
        self.sessions.values().filter(|s| s.has_tokens()).collect()
    }

    pub fn get_pkce_sessions(&self) -> Vec<&OAuthSession> {
        self.sessions.values().filter(|s| s.has_pkce()).collect()
    }

    pub fn deactivate_session(&mut self, id: Uuid) {
        if let Some(session) = self.sessions.get_mut(&id) {
            session.is_active = false;
        }
    }

    pub fn link_token_to_authorization(&mut self, auth_session_id: Uuid, token_session_id: Uuid) {
        let token_data = self.sessions.get(&token_session_id).map(|ts| {
            (
                ts.access_token.clone(),
                ts.refresh_token.clone(),
                ts.id_token.clone(),
                ts.token_type.clone(),
                ts.expires_in,
                ts.token_endpoint.clone(),
            )
        });

        if let Some((
            access_token,
            refresh_token,
            id_token,
            token_type,
            expires_in,
            token_endpoint,
        )) = token_data
        {
            if let Some(auth_session) = self.sessions.get_mut(&auth_session_id) {
                auth_session.access_token = access_token;
                auth_session.refresh_token = refresh_token;
                auth_session.id_token = id_token;
                auth_session.token_type = token_type;
                auth_session.expires_in = expires_in;
                auth_session.token_endpoint = token_endpoint;
                auth_session.touch();
            }
        }
    }

    pub fn all_sessions(&self) -> Vec<&OAuthSession> {
        self.sessions.values().collect()
    }

    pub fn clear(&mut self) {
        self.sessions.clear();
        self.sessions_by_client.clear();
        self.sessions_by_endpoint.clear();
    }
}

impl Default for OAuthSessionTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_track_and_retrieve_session() {
        let mut tracker = OAuthSessionTracker::new();
        let mut session = OAuthSession::new(OAuthFlowType::AuthorizationCodePkce);
        session.client_id = Some("test-client".to_string());
        session.authorization_endpoint = Some("https://example.com/authorize".to_string());

        let id = tracker.track_session(session);
        let retrieved = tracker.get_session(id);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().client_id.as_deref(), Some("test-client"));
    }

    #[test]
    fn test_sessions_by_client() {
        let mut tracker = OAuthSessionTracker::new();
        let mut s1 = OAuthSession::new(OAuthFlowType::AuthorizationCode);
        s1.client_id = Some("client-a".to_string());
        tracker.track_session(s1);

        let mut s2 = OAuthSession::new(OAuthFlowType::ClientCredentials);
        s2.client_id = Some("client-a".to_string());
        tracker.track_session(s2);

        let sessions = tracker.get_sessions_by_client("client-a");
        assert_eq!(sessions.len(), 2);
    }

    #[test]
    fn test_deactivate_session() {
        let mut tracker = OAuthSessionTracker::new();
        let session = OAuthSession::new(OAuthFlowType::Implicit);
        let id = tracker.track_session(session);

        tracker.deactivate_session(id);
        let s = tracker.get_session(id).unwrap();
        assert!(!s.is_active);
    }
}
