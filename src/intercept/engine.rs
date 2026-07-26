#![allow(dead_code)]

use uuid::Uuid;

use crate::intercept::editor::{edit_request, edit_response, EditOperation};
use crate::intercept::errors::InterceptError;
use crate::intercept::events::{InterceptEvent, InterceptEventBus};
use crate::intercept::models::{
    InterceptDirection, InterceptStatus, InterceptedItem, InterceptedRequest, InterceptedResponse,
};
use crate::intercept::queue::InterceptQueue;
use crate::intercept::session::InterceptSessionManager;
use crate::network::models::{HttpRequest, HttpResponse};

pub struct InterceptEngine {
    queue: InterceptQueue,
    sessions: InterceptSessionManager,
    event_bus: InterceptEventBus,
    intercept_requests: bool,
    intercept_responses: bool,
}

impl InterceptEngine {
    pub fn new(queue_capacity: usize) -> Self {
        Self {
            queue: InterceptQueue::new(queue_capacity),
            sessions: InterceptSessionManager::new(),
            event_bus: InterceptEventBus::new(1024),
            intercept_requests: true,
            intercept_responses: true,
        }
    }

    pub fn event_bus(&self) -> InterceptEventBus {
        self.event_bus.clone()
    }
    pub fn queue(&self) -> &InterceptQueue {
        &self.queue
    }

    pub fn set_intercept_requests(&mut self, enabled: bool) {
        self.intercept_requests = enabled;
    }
    pub fn set_intercept_responses(&mut self, enabled: bool) {
        self.intercept_responses = enabled;
    }

    // ── Intercept ──

    pub fn intercept_request(
        &mut self,
        request: HttpRequest,
    ) -> Result<Option<InterceptedRequest>, InterceptError> {
        if !self.intercept_requests {
            return Ok(None);
        }

        let conn_id = self.sessions.next_connection_id();
        let txn_id = self.sessions.pair_transaction_id();
        let session =
            self.sessions
                .create_session(InterceptDirection::Request, conn_id, Some(txn_id));

        let item = InterceptedRequest {
            session,
            request: request.clone(),
        };
        let id = item.session.id;
        let url = item.request.url.clone();
        let method = item.request.method.clone();

        self.queue.push(InterceptedItem::Request(item.clone()))?;
        self.event_bus
            .emit(InterceptEvent::RequestIntercepted { id, url, method });

        Ok(Some(item))
    }

    pub fn intercept_response(
        &mut self,
        response: HttpResponse,
        transaction_id: Option<Uuid>,
    ) -> Result<Option<InterceptedResponse>, InterceptError> {
        if !self.intercept_responses {
            return Ok(None);
        }

        let conn_id = self.sessions.next_connection_id();
        let session =
            self.sessions
                .create_session(InterceptDirection::Response, conn_id, transaction_id);

        let item = InterceptedResponse {
            session,
            response: response.clone(),
        };
        let id = item.session.id;
        let url = item.response.url.clone();
        let status = item.response.status_code;

        self.queue.push(InterceptedItem::Response(item.clone()))?;
        self.event_bus
            .emit(InterceptEvent::ResponseIntercepted { id, url, status });

        Ok(Some(item))
    }

    // ── Edit ──

    pub fn edit_request_item(
        &mut self,
        id: Uuid,
        operations: &[EditOperation],
    ) -> Result<(), InterceptError> {
        let item = self
            .queue
            .find_mut(id)
            .ok_or_else(|| InterceptError::NotFound(id.to_string()))?;
        if matches!(
            item.status(),
            InterceptStatus::Forwarded | InterceptStatus::Dropped
        ) {
            return Err(InterceptError::AlreadyProcessed(id.to_string()));
        }
        if let InterceptedItem::Request(ref mut r) = item {
            edit_request(&mut r.request, operations);
            r.session.status = InterceptStatus::Edited;
        }
        self.event_bus.emit(InterceptEvent::RequestModified { id });
        Ok(())
    }

    pub fn edit_response_item(
        &mut self,
        id: Uuid,
        operations: &[EditOperation],
    ) -> Result<(), InterceptError> {
        let item = self
            .queue
            .find_mut(id)
            .ok_or_else(|| InterceptError::NotFound(id.to_string()))?;
        if matches!(
            item.status(),
            InterceptStatus::Forwarded | InterceptStatus::Dropped
        ) {
            return Err(InterceptError::AlreadyProcessed(id.to_string()));
        }
        if let InterceptedItem::Response(ref mut r) = item {
            edit_response(&mut r.response, operations);
            r.session.status = InterceptStatus::Edited;
        }
        self.event_bus.emit(InterceptEvent::ResponseModified { id });
        Ok(())
    }

    // ── Forward ──

    pub fn forward_request(&mut self, id: Uuid) -> Result<Option<HttpRequest>, InterceptError> {
        let item = self
            .queue
            .find_mut(id)
            .ok_or_else(|| InterceptError::NotFound(id.to_string()))?;
        if matches!(
            item.status(),
            InterceptStatus::Forwarded | InterceptStatus::Dropped
        ) {
            return Err(InterceptError::AlreadyProcessed(id.to_string()));
        }
        match item {
            InterceptedItem::Request(ref mut r) => {
                r.session.status = InterceptStatus::Forwarded;
                let req = r.request.clone();
                self.event_bus.emit(InterceptEvent::RequestForwarded { id });
                Ok(Some(req))
            }
            _ => Err(InterceptError::NotFound("Not a request".into())),
        }
    }

    pub fn forward_response(&mut self, id: Uuid) -> Result<Option<HttpResponse>, InterceptError> {
        let item = self
            .queue
            .find_mut(id)
            .ok_or_else(|| InterceptError::NotFound(id.to_string()))?;
        if matches!(
            item.status(),
            InterceptStatus::Forwarded | InterceptStatus::Dropped
        ) {
            return Err(InterceptError::AlreadyProcessed(id.to_string()));
        }
        match item {
            InterceptedItem::Response(ref mut r) => {
                r.session.status = InterceptStatus::Forwarded;
                let resp = r.response.clone();
                self.event_bus
                    .emit(InterceptEvent::ResponseForwarded { id });
                Ok(Some(resp))
            }
            _ => Err(InterceptError::NotFound("Not a response".into())),
        }
    }

    // ── Drop ──

    pub fn drop_request(&mut self, id: Uuid) -> Result<(), InterceptError> {
        let item = self
            .queue
            .remove(id)
            .ok_or_else(|| InterceptError::NotFound(id.to_string()))?;
        self.event_bus.emit(InterceptEvent::RequestDropped { id });
        let _ = item;
        Ok(())
    }

    pub fn drop_response(&mut self, id: Uuid) -> Result<(), InterceptError> {
        let item = self
            .queue
            .remove(id)
            .ok_or_else(|| InterceptError::NotFound(id.to_string()))?;
        self.event_bus.emit(InterceptEvent::ResponseDropped { id });
        let _ = item;
        Ok(())
    }

    // ── Resume ──

    pub fn resume_request(&mut self, id: Uuid) -> Result<Option<HttpRequest>, InterceptError> {
        self.forward_request(id)
    }

    pub fn resume_response(&mut self, id: Uuid) -> Result<Option<HttpResponse>, InterceptError> {
        self.forward_response(id)
    }
}
