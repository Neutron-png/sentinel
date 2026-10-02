#![allow(dead_code)]

pub struct QuicStreamManager {
    max_stream_data: u64,
    active_streams: u32,
    next_bidirectional: u64,
    next_unidirectional: u64,
}

impl QuicStreamManager {
    pub fn new(max_stream_data: u64) -> Self {
        Self {
            max_stream_data,
            active_streams: 0,
            next_bidirectional: 0,
            next_unidirectional: 2,
        }
    }

    pub fn allocate_bidirectional(&mut self) -> Result<u64, crate::http3::errors::Http3Error> {
        let id = self.next_bidirectional;
        self.next_bidirectional += 4;
        self.active_streams += 1;
        Ok(id)
    }

    pub fn allocate_unidirectional(&mut self) -> Result<u64, crate::http3::errors::Http3Error> {
        let id = self.next_unidirectional;
        self.next_unidirectional += 4;
        self.active_streams += 1;
        Ok(id)
    }

    pub fn close_stream(&mut self, _stream_id: u64) {
        self.active_streams = self.active_streams.saturating_sub(1);
    }

    pub fn active_count(&self) -> u32 {
        self.active_streams
    }

    pub fn max_stream_data(&self) -> u64 {
        self.max_stream_data
    }
}
