#![allow(dead_code)]

use std::time::Instant;
use uuid::Uuid;

use crate::network::models::{HttpRequest, HttpResponse};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterceptDirection {
    Request,
    Response,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterceptStatus {
    Waiting,
    Edited,
    Forwarded,
    Dropped,
}

impl InterceptStatus {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Waiting => "Waiting",
            Self::Edited => "Edited",
            Self::Forwarded => "Forwarded",
            Self::Dropped => "Dropped",
        }
    }
}

#[derive(Debug, Clone)]
pub struct InterceptSession {
    pub id: Uuid,
    pub timestamp: Instant,
    pub direction: InterceptDirection,
    pub connection_id: String,
    pub transaction_id: Option<Uuid>,
    pub status: InterceptStatus,
}

#[derive(Debug, Clone)]
pub struct InterceptedRequest {
    pub session: InterceptSession,
    pub request: HttpRequest,
}

#[derive(Debug, Clone)]
pub struct InterceptedResponse {
    pub session: InterceptSession,
    pub response: HttpResponse,
}

#[derive(Debug, Clone)]
pub enum InterceptedItem {
    Request(InterceptedRequest),
    Response(InterceptedResponse),
}

impl InterceptedItem {
    pub fn id(&self) -> Uuid {
        match self {
            Self::Request(r) => r.session.id,
            Self::Response(r) => r.session.id,
        }
    }
    pub fn status(&self) -> InterceptStatus {
        match self {
            Self::Request(r) => r.session.status,
            Self::Response(r) => r.session.status,
        }
    }
    pub fn direction(&self) -> InterceptDirection {
        match self {
            Self::Request(_) => InterceptDirection::Request,
            Self::Response(_) => InterceptDirection::Response,
        }
    }
}
