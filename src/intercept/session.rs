#![allow(dead_code)]

use uuid::Uuid;

use crate::intercept::models::{InterceptDirection, InterceptSession};

#[derive(Default)]
pub struct InterceptSessionManager {
    connection_counter: u64,
}

impl InterceptSessionManager {
    pub fn new() -> Self {
        Self {
            connection_counter: 0,
        }
    }

    pub fn next_connection_id(&mut self) -> String {
        self.connection_counter += 1;
        format!("conn-{}", self.connection_counter)
    }

    pub fn create_session(
        &self,
        direction: InterceptDirection,
        connection_id: String,
        transaction_id: Option<Uuid>,
    ) -> InterceptSession {
        InterceptSession {
            id: Uuid::new_v4(),
            timestamp: std::time::Instant::now(),
            direction,
            connection_id,
            transaction_id,
            status: super::models::InterceptStatus::Waiting,
        }
    }

    pub fn pair_transaction_id(&self) -> Uuid {
        Uuid::new_v4()
    }
}
