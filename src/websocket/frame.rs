#![allow(dead_code)]

use uuid::Uuid;

use crate::websocket::models::{WsDirection, WsFrame, WsOpcode};
use crate::websocket::parser;

pub struct FrameBuilder;

impl FrameBuilder {
    pub fn text(connection_id: Uuid, text: &str) -> WsFrame {
        WsFrame::new(
            connection_id,
            WsOpcode::Text,
            text.as_bytes().to_vec(),
            WsDirection::ClientToServer,
            true,
            true,
        )
    }

    pub fn binary(connection_id: Uuid, data: &[u8]) -> WsFrame {
        WsFrame::new(
            connection_id,
            WsOpcode::Binary,
            data.to_vec(),
            WsDirection::ClientToServer,
            true,
            true,
        )
    }

    pub fn ping(connection_id: Uuid) -> WsFrame {
        WsFrame::new(
            connection_id,
            WsOpcode::Ping,
            vec![],
            WsDirection::ClientToServer,
            true,
            false,
        )
    }

    pub fn close(connection_id: Uuid) -> WsFrame {
        WsFrame::new(
            connection_id,
            WsOpcode::Close,
            vec![],
            WsDirection::ClientToServer,
            true,
            false,
        )
    }

    pub fn build_bytes(frame: &WsFrame) -> Vec<u8> {
        parser::build_frame(frame.opcode, &frame.payload, frame.fin, frame.masked)
    }
}
