#![allow(dead_code)]

use crate::http3::engine::ConnectionState;
use crate::network::protocol::{Http3Config, HttpVersion, NegotiatedProtocol};
use uuid::Uuid;

pub struct Http3Connection {
    pub id: Uuid,
    pub authority: String,
    pub server_name: String,
    pub version: HttpVersion,
    pub config: Http3Config,
    pub stream_manager: crate::http3::streams::QuicStreamManager,
    pub qpack_encoder: crate::http3::qpack::QpackContext,
    pub qpack_decoder: crate::http3::qpack::QpackContext,
    pub negotiated: NegotiatedProtocol,
    pub state: ConnectionState,
    pub quic_connection_id: Vec<u8>,
    pub zero_rtt_accepted: bool,
}
