#![allow(dead_code)]

use crate::http2::alpn::AlpnHandler;
use crate::http2::errors::Http2Error;
use crate::http2::frames::FrameHandler;
use crate::http2::hpack::HpackContext;
use crate::http2::models::Http2Connection;
use crate::http2::streams::StreamManager;
use crate::network::protocol::{Http2Config, HttpVersion, NegotiatedProtocol};
use uuid::Uuid;

pub struct Http2Engine {
    config: Http2Config,
    connections: Vec<Http2Connection>,
}

impl Http2Engine {
    pub fn new(config: Http2Config) -> Self {
        Self {
            config,
            connections: Vec::new(),
        }
    }

    pub fn config(&self) -> &Http2Config {
        &self.config
    }

    pub fn create_connection(
        &mut self,
        authority: String,
        negotiated_protocol: &NegotiatedProtocol,
    ) -> &Http2Connection {
        let conn = Http2Connection {
            id: Uuid::new_v4(),
            authority,
            version: HttpVersion::Http2,
            config: self.config.clone(),
            stream_manager: StreamManager::new(self.config.max_concurrent_streams),
            frame_handler: FrameHandler::new(),
            hpack_context: HpackContext::new(self.config.max_header_list_size),
            alpn_handler: AlpnHandler::new(),
            negotiated: Some(negotiated_protocol.clone()),
            state: ConnectionState::Connected,
        };
        self.connections.push(conn);
        self.connections.last().unwrap()
    }

    pub fn get_connection(&self, id: Uuid) -> Option<&Http2Connection> {
        self.connections.iter().find(|c| c.id == id)
    }

    pub fn get_connection_mut(&mut self, id: Uuid) -> Option<&mut Http2Connection> {
        self.connections.iter_mut().find(|c| c.id == id)
    }

    pub fn close_connection(&mut self, id: Uuid) -> Result<(), Http2Error> {
        let pos = self
            .connections
            .iter()
            .position(|c| c.id == id)
            .ok_or(Http2Error::ConnectionNotFound(id.to_string()))?;
        self.connections.remove(pos);
        Ok(())
    }

    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Connecting,
    Connected,
    GoAway,
    Closed,
}
