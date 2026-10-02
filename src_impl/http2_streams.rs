#![allow(dead_code)]

use crate::network::protocol::Http2Config;

pub struct StreamManager {
    max_concurrent: u32,
    active_streams: u32,
    next_stream_id: u32,
    last_stream_id: u32,
}

impl StreamManager {
    pub fn new(max_concurrent: u32) -> Self {
        Self {
            max_concurrent,
            active_streams: 0,
            next_stream_id: 1,
            last_stream_id: 0,
        }
    }

    pub fn allocate_stream_id(&mut self) -> Result<u32, crate::http2::errors::Http2Error> {
        if self.active_streams >= self.max_concurrent {
            return Err(crate::http2::errors::Http2Error::Stream(
                "Max concurrent streams reached".into(),
            ));
        }
        let id = self.next_stream_id;
        self.next_stream_id += 2;
        self.active_streams += 1;
        self.last_stream_id = id;
        Ok(id)
    }

    pub fn close_stream(&mut self, _stream_id: u32) {
        self.active_streams = self.active_streams.saturating_sub(1);
    }

    pub fn active_stream_count(&self) -> u32 {
        self.active_streams
    }

    pub fn last_stream_id(&self) -> u32 {
        self.last_stream_id
    }

    pub fn can_open_stream(&self) -> bool {
        self.active_streams < self.max_concurrent
    }

    pub fn reset(&mut self) {
        self.active_streams = 0;
        self.next_stream_id = 1;
        self.last_stream_id = 0;
    }
}

#[derive(Debug, Clone)]
pub struct Http2Connection {
    pub id: uuid::Uuid,
    pub authority: String,
    pub version: crate::network::protocol::HttpVersion,
    pub config: Http2Config,
    pub stream_manager: StreamManager,
    pub frame_handler: crate::http2::frames::FrameHandler,
    pub hpack_context: crate::http2::hpack::HpackContext,
    pub alpn_handler: crate::http2::alpn::AlpnHandler,
    pub negotiated: Option<crate::network::protocol::NegotiatedProtocol>,
    pub state: crate::http2::engine::ConnectionState,
}
