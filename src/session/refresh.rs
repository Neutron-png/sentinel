#![allow(dead_code)]

use uuid::Uuid;

use crate::auth::engine::AuthenticationEngine;
use crate::browser::automation::engine::BrowserAutomationEngine;
use crate::session::errors::SessionError;

pub struct SessionRefresher;

impl SessionRefresher {
    pub async fn refresh(
        auth_engine: &mut AuthenticationEngine,
        automation_engine: &mut BrowserAutomationEngine,
        workflow_id: Uuid,
    ) -> Result<bool, SessionError> {
        auth_engine
            .replay(workflow_id, automation_engine)
            .await
            .map_err(|e| SessionError::Refresh(e.to_string()))
    }

    pub async fn restore_session(
        _auth_engine: &mut AuthenticationEngine,
        _automation_engine: &mut BrowserAutomationEngine,
        _workflow_id: Uuid,
    ) -> Result<bool, SessionError> {
        Ok(true)
    }

    pub async fn retry_failed(
        auth_engine: &mut AuthenticationEngine,
        automation_engine: &mut BrowserAutomationEngine,
        workflow_id: Uuid,
        max_retries: u32,
    ) -> Result<bool, SessionError> {
        for _ in 0..max_retries {
            if Self::refresh(auth_engine, automation_engine, workflow_id).await? {
                return Ok(true);
            }
        }
        Ok(false)
    }
}
