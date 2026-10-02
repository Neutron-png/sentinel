#![allow(dead_code)]

use crate::http3::engine::Http3Engine;
use crate::network::protocol::{Http3Config, HttpVersion, NegotiatedProtocol};
// uuid not used here

pub struct Http3Integration {
    engine: Http3Engine,
}

impl Http3Integration {
    pub fn new(config: Http3Config) -> Self {
        Self {
            engine: Http3Engine::new(config),
        }
    }

    pub fn engine(&self) -> &Http3Engine {
        &self.engine
    }

    pub fn engine_mut(&mut self) -> &mut Http3Engine {
        &mut self.engine
    }

    pub fn negotiate_connection(
        &mut self,
        authority: &str,
        server_name: &str,
    ) -> Result<NegotiatedProtocol, crate::http3::errors::Http3Error> {
        let (_conn, _initial_packet) = self
            .engine
            .create_connection(authority.to_string(), server_name.to_string())?;
        Ok(NegotiatedProtocol {
            version: HttpVersion::Http3,
            alpn: None,
            connection_id: _conn.id,
            streams: Vec::new(),
            frame_log: Vec::new(),
            server_push: Vec::new(),
            quic_connection_id: Some(_conn.quic_connection_id.clone()),
            zero_rtt_accepted: Some(_conn.zero_rtt_accepted),
        })
    }

    pub fn supports_zero_rtt(&self) -> bool {
        self.engine.config().enable_zero_rtt
    }

    pub fn supports_migration(&self) -> bool {
        self.engine.config().enable_migration
    }

    pub fn version(&self) -> u32 {
        self.engine.transport().version()
    }
}

impl Default for Http3Integration {
    fn default() -> Self {
        Self::new(Http3Config::default())
    }
}
