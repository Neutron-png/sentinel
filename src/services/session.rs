use chrono::Utc;
use uuid::Uuid;

use crate::db::repository::Repository;
use crate::models::session::Session;

pub fn save_current(repo: &Repository, assessment_id: Uuid, screen: &str) {
    if let Err(e) = repo.save_session(&Session {
        id: Uuid::new_v4(),
        assessment_id,
        current_screen: screen.to_string(),
        last_opened_at: Utc::now(),
    }) {
        eprintln!("Failed to save session: {e}");
    }
}

pub fn clear_session(repo: &Repository) {
    if let Err(e) = repo.delete_all_sessions() {
        eprintln!("Failed to clear session: {e}");
    }
}

pub fn load_last(repo: &Repository) -> Option<Session> {
    repo.load_session().unwrap_or(None)
}
