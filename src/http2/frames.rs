#![allow(dead_code)]

#[derive(Debug, Clone)]
pub struct FrameHandler {
    frame_count: u64,
    last_frame_timestamp: Option<chrono::DateTime<chrono::Utc>>,
}

impl FrameHandler {
    pub fn new() -> Self {
        Self {
            frame_count: 0,
            last_frame_timestamp: None,
        }
    }

    pub fn parse_frame_type(
        &self,
        data: &[u8],
    ) -> Result<crate::network::protocol::HttpFrameType, crate::http2::errors::Http2Error> {
        if data.len() < 9 {
            return Err(crate::http2::errors::Http2Error::Frame(
                "Frame too short".into(),
            ));
        }
        let frame_type = data[3];
        Ok(crate::network::protocol::HttpFrameType::from_code(
            frame_type,
        ))
    }

    pub fn create_data_frame(stream_id: u32, payload: &[u8], end_stream: bool) -> Vec<u8> {
        let mut flags = 0u8;
        if end_stream {
            flags |= 0x1;
        }
        let length = payload.len() as u32;
        let mut frame = Vec::with_capacity(9 + payload.len());
        frame.extend_from_slice(&length.to_be_bytes()[1..]);
        frame.push(0x0);
        frame.push(flags);
        frame.extend_from_slice(&stream_id.to_be_bytes());
        frame.push(0u8);
        frame.extend_from_slice(payload);
        frame
    }

    pub fn create_headers_frame(
        stream_id: u32,
        header_block: &[u8],
        end_headers: bool,
        end_stream: bool,
    ) -> Vec<u8> {
        let mut flags = 0u8;
        if end_headers {
            flags |= 0x4;
        }
        if end_stream {
            flags |= 0x1;
        }
        let length = header_block.len() as u32;
        let mut frame = Vec::with_capacity(9 + header_block.len());
        frame.extend_from_slice(&length.to_be_bytes()[1..]);
        frame.push(0x1);
        frame.push(flags);
        frame.extend_from_slice(&stream_id.to_be_bytes());
        frame.push(0u8);
        frame.extend_from_slice(header_block);
        frame
    }

    pub fn create_rst_stream_frame(stream_id: u32, error_code: u32) -> Vec<u8> {
        let mut frame = vec![0, 0, 4, 0x3, 0, 0, 0, 0, 0];
        frame[5..9].copy_from_slice(&stream_id.to_be_bytes());
        frame.push(0u8);
        frame.extend_from_slice(&error_code.to_be_bytes());
        frame
    }

    pub fn create_settings_frame(settings: &[(u16, u32)]) -> Vec<u8> {
        let payload_len = settings.len() * 6;
        let mut frame = vec![0u8; 9 + payload_len];
        frame[0] = ((payload_len >> 16) & 0xFF) as u8;
        frame[1] = ((payload_len >> 8) & 0xFF) as u8;
        frame[2] = (payload_len & 0xFF) as u8;
        frame[3] = 0x4;
        let mut offset = 9;
        for (id, value) in settings {
            frame[offset..offset + 2].copy_from_slice(&id.to_be_bytes());
            frame[offset + 2..offset + 6].copy_from_slice(&value.to_be_bytes());
            offset += 6;
        }
        frame
    }

    pub fn create_window_update_frame(stream_id: u32, increment: u32) -> Vec<u8> {
        let mut frame = vec![0, 0, 4, 0x8, 0, 0, 0, 0, 0];
        frame[5..9].copy_from_slice(&stream_id.to_be_bytes());
        frame.push(0u8);
        frame.extend_from_slice(&increment.to_be_bytes());
        frame
    }

    pub fn create_ping_frame(payload: [u8; 8], is_ack: bool) -> Vec<u8> {
        let mut frame = vec![0, 0, 8, 0x6, if is_ack { 0x1 } else { 0 }, 0, 0, 0, 0];
        frame.push(0u8);
        frame.extend_from_slice(&payload);
        frame
    }

    pub fn create_goaway_frame(last_stream_id: u32, error_code: u32, debug_data: &[u8]) -> Vec<u8> {
        let payload_len = 8 + debug_data.len();
        let mut frame = Vec::with_capacity(9 + payload_len);
        let mut header = vec![0u8; 9];
        header[0] = ((payload_len >> 16) & 0xFF) as u8;
        header[1] = ((payload_len >> 8) & 0xFF) as u8;
        header[2] = (payload_len & 0xFF) as u8;
        header[3] = 0x7;
        frame.extend_from_slice(&header);
        frame.push(0u8);
        frame.extend_from_slice(&last_stream_id.to_be_bytes());
        frame.extend_from_slice(&error_code.to_be_bytes());
        frame.extend_from_slice(debug_data);
        frame
    }

    pub fn parse_frame_header(
        data: &[u8],
    ) -> Option<(u32, crate::network::protocol::HttpFrameType, u8, u32)> {
        if data.len() < 9 {
            return None;
        }
        let length = u32::from_be_bytes([0, data[0], data[1], data[2]]);
        let frame_type = crate::network::protocol::HttpFrameType::from_code(data[3]);
        let flags = data[4];
        let stream_id = u32::from_be_bytes([data[5], data[6], data[7], data[8]]) & 0x7FFFFFFF;
        Some((length, frame_type, flags, stream_id))
    }

    pub fn create_frame_metadata(
        frame_type: crate::network::protocol::HttpFrameType,
        stream_id: u32,
        flags: u8,
        length: u32,
    ) -> crate::network::protocol::FrameMetadata {
        crate::network::protocol::FrameMetadata {
            frame_type,
            stream_id,
            flags,
            length,
            timestamp: chrono::Utc::now(),
        }
    }
}

impl Default for FrameHandler {
    fn default() -> Self {
        Self::new()
    }
}
