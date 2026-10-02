#![allow(dead_code)]

use crate::http3::errors::Http3Error;
use crate::http3::models::Http3Connection;
use crate::http3::streams::QuicStreamManager;
use crate::http3::transport::QuicTransport;
use crate::network::protocol::{
    Http3Config, HttpVersion, NegotiatedProtocol, StreamDirection, StreamMetadata, StreamState,
};
use uuid::Uuid;

pub struct Http3Engine {
    config: Http3Config,
    connections: Vec<Http3Connection>,
    transport: QuicTransport,
}

impl Http3Engine {
    pub fn new(config: Http3Config) -> Self {
        Self {
            config,
            connections: Vec::new(),
            transport: QuicTransport::new(),
        }
    }

    pub fn config(&self) -> &Http3Config {
        &self.config
    }

    pub fn create_connection(
        &mut self,
        authority: String,
        server_name: String,
    ) -> Result<(&Http3Connection, Vec<u8>), Http3Error> {
        let connection_id = Uuid::new_v4();
        let quic_conn_id = Uuid::new_v4().as_bytes().to_vec();

        let initial_packet = self.transport.build_initial_packet(
            &quic_conn_id,
            &server_name,
            self.config.quic_version,
            self.config.max_data,
            self.config.max_stream_data,
            self.config.idle_timeout_ms,
        );

        let conn = Http3Connection {
            id: connection_id,
            authority,
            server_name,
            version: HttpVersion::Http3,
            config: self.config.clone(),
            stream_manager: QuicStreamManager::new(self.config.max_stream_data),
            qpack_encoder: crate::http3::qpack::QpackContext::new(4096),
            qpack_decoder: crate::http3::qpack::QpackContext::new(4096),
            negotiated: NegotiatedProtocol::new(HttpVersion::Http3, connection_id),
            state: ConnectionState::Connecting,
            quic_connection_id: quic_conn_id,
            zero_rtt_accepted: false,
        };

        let packet = initial_packet;
        self.connections.push(conn);
        Ok((self.connections.last().unwrap(), packet))
    }

    pub fn get_connection(&self, id: Uuid) -> Option<&Http3Connection> {
        self.connections.iter().find(|c| c.id == id)
    }

    pub fn get_connection_mut(&mut self, id: Uuid) -> Option<&mut Http3Connection> {
        self.connections.iter_mut().find(|c| c.id == id)
    }

    pub fn close_connection(&mut self, id: Uuid) -> Result<(), Http3Error> {
        let pos = self
            .connections
            .iter()
            .position(|c| c.id == id)
            .ok_or(Http3Error::ConnectionNotFound(id.to_string()))?;
        self.connections.remove(pos);
        Ok(())
    }

    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }

    pub fn transport(&self) -> &QuicTransport {
        &self.transport
    }

    pub fn create_stream_metadata(
        &self,
        connection_id: Uuid,
        direction: StreamDirection,
    ) -> StreamMetadata {
        StreamMetadata {
            stream_id: 0, // Set by caller
            connection_id,
            protocol: HttpVersion::Http3,
            direction,
            state: StreamState::Idle,
            priority: None,
            dependency: None,
            weight: None,
            window_size: self.config.max_stream_data as u32,
            bytes_sent: 0,
            bytes_received: 0,
            created_at: chrono::Utc::now(),
            closed_at: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Connecting,
    Connected,
    Draining,
    Closed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_connection() {
        let mut engine = Http3Engine::new(Http3Config::default());
        let result = engine.create_connection("example.com".into(), "example.com".into());
        assert!(result.is_ok());
        assert_eq!(engine.connection_count(), 1);
    }

    #[test]
    fn test_close_connection() {
        let mut engine = Http3Engine::new(Http3Config::default());
        let (conn, _) = engine
            .create_connection("example.com".into(), "example.com".into())
            .unwrap();
        let id = conn.id;
        assert!(engine.close_connection(id).is_ok());
        assert_eq!(engine.connection_count(), 0);
    }
}
