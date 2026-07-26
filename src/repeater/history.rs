#![allow(dead_code)]

use chrono::Utc;
use uuid::Uuid;

use crate::repeater::errors::RepeaterError;
use crate::repeater::models::{RepeaterTab, ResponseEntry};

pub fn record_response(
    tab: &mut RepeaterTab,
    response: crate::network::models::HttpResponse,
    duration_ms: u64,
) {
    let entry = ResponseEntry {
        id: Uuid::new_v4(),
        response_size: response.body.len() as u64,
        response,
        sent_at: Utc::now(),
        duration_ms,
    };
    tab.response_history.push(entry);
    tab.history_index = tab.response_history.len().saturating_sub(1);
    tab.latest_response = tab.response_history.last().cloned();
    tab.updated_at = Utc::now();
}

pub fn clear_history(tab: &mut RepeaterTab) {
    tab.response_history.clear();
    tab.latest_response = None;
    tab.history_index = 0;
}

pub fn delete_response(tab: &mut RepeaterTab, id: Uuid) -> Result<(), RepeaterError> {
    let pos = tab
        .response_history
        .iter()
        .position(|r| r.id == id)
        .ok_or_else(|| RepeaterError::NotFound(id.to_string()))?;
    tab.response_history.remove(pos);
    if tab.history_index >= tab.response_history.len() {
        tab.history_index = tab.response_history.len().saturating_sub(1);
    }
    tab.latest_response = tab.response_history.last().cloned();
    Ok(())
}
