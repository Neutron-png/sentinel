use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Session {
    pub id: Uuid,
    pub assessment_id: Uuid,
    pub current_screen: String,
    pub last_opened_at: DateTime<Utc>,
}
