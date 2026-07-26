#![allow(dead_code)]

use uuid::Uuid;

use crate::websocket::events::{WsEvent, WsEventBus};
use crate::websocket::models::{
    WsConnection, WsConversation, WsDirection, WsFrame, WsHandshake, WsOpcode, WsState,
};
use crate::websocket::parser;
use crate::websocket::replay::ReplayManager;

pub struct WebSocketEngine {
    connections: Vec<WsConnection>,
    frames: Vec<WsFrame>,
    handshakes: Vec<WsHandshake>,
    conversations: Vec<WsConversation>,
    replay: ReplayManager,
    event_bus: WsEventBus,
}

impl WebSocketEngine {
    pub fn new() -> Self {
        Self {
            connections: Vec::new(),
            frames: Vec::new(),
            handshakes: Vec::new(),
            conversations: Vec::new(),
            replay: ReplayManager::new(),
            event_bus: WsEventBus::new(1024),
        }
    }

    pub fn event_bus(&self) -> WsEventBus {
        self.event_bus.clone()
    }

    // ── Connections ──

    pub fn open_connection(&mut self, url: &str) -> &WsConnection {
        let conn = WsConnection::new(url);
        self.connections.push(conn);
        let c = self.connections.last().unwrap();
        self.event_bus.emit(WsEvent::ConnectionOpened {
            connection_id: c.id,
            url: url.to_string(),
        });
        c
    }

    pub fn close_connection(&mut self, connection_id: Uuid) {
        if let Some(c) = self.connections.iter_mut().find(|c| c.id == connection_id) {
            c.state = WsState::Closed;
            c.end_time = Some(chrono::Utc::now());
            self.event_bus
                .emit(WsEvent::ConnectionClosed { connection_id });
        }
    }

    pub fn find_connection(&self, id: Uuid) -> Option<&WsConnection> {
        self.connections.iter().find(|c| c.id == id)
    }

    // ── Frames ──

    pub fn parse_incoming(&mut self, connection_id: Uuid, data: &[u8]) -> Option<WsFrame> {
        if let Some(frame) = parser::parse_frame(data, WsDirection::ServerToClient, connection_id) {
            let frame_id = frame.id;
            let frame_size = frame.payload_length;
            let opcode = frame.opcode;
            self.replay.record(connection_id, &frame);
            self.frames.push(frame);

            let conv = self.get_or_create_conversation(connection_id);
            conv.add_frame(frame_id, frame_size, false);
            if opcode == WsOpcode::Close {
                conv.close();
            }

            self.event_bus.emit(WsEvent::FrameReceived {
                connection_id,
                frame_id,
                opcode: opcode.label().to_string(),
                direction: WsDirection::ServerToClient,
            });
            self.frames.last().cloned()
        } else {
            None
        }
    }

    pub fn parse_outgoing(&mut self, connection_id: Uuid, data: &[u8]) -> Option<WsFrame> {
        if let Some(frame) = parser::parse_frame(data, WsDirection::ClientToServer, connection_id) {
            let frame_id = frame.id;
            let frame_size = frame.payload_length;
            let opcode = frame.opcode;
            self.replay.record(connection_id, &frame);
            self.frames.push(frame);

            let conv = self.get_or_create_conversation(connection_id);
            conv.add_frame(frame_id, frame_size, true);
            if opcode == WsOpcode::Close {
                conv.close();
            }

            self.event_bus.emit(WsEvent::FrameSent {
                connection_id,
                frame_id,
            });
            self.frames.last().cloned()
        } else {
            None
        }
    }

    pub fn all_frames(&self) -> &[WsFrame] {
        &self.frames
    }
    pub fn frames_for(&self, connection_id: Uuid) -> Vec<&WsFrame> {
        self.frames
            .iter()
            .filter(|f| f.connection_id == connection_id)
            .collect()
    }

    // ── Edit ──

    pub fn edit_frame(&mut self, frame_id: Uuid, new_payload: Vec<u8>) -> bool {
        if let Some(frame) = self.frames.iter_mut().find(|f| f.id == frame_id) {
            frame.payload = new_payload;
            frame.payload_length = frame.payload.len() as u64;
            self.event_bus.emit(WsEvent::FrameModified {
                connection_id: frame.connection_id,
                frame_id,
            });
            true
        } else {
            false
        }
    }

    pub fn drop_frame(&mut self, frame_id: Uuid) -> bool {
        if let Some(pos) = self.frames.iter().position(|f| f.id == frame_id) {
            let cid = self.frames[pos].connection_id;
            self.frames.remove(pos);
            self.event_bus.emit(WsEvent::FrameDropped {
                connection_id: cid,
                frame_id,
            });
            true
        } else {
            false
        }
    }

    // ── Replay ──

    pub fn replay_frame(&self, frame_id: Uuid) -> Option<&WsFrame> {
        let frame = self.replay.replay_one(frame_id);
        if let Some(f) = frame {
            self.event_bus.emit(WsEvent::FrameReplayed {
                connection_id: f.connection_id,
                frame_id: f.id,
            });
        }
        frame
    }

    pub fn replay_connection(&self, connection_id: Uuid) -> Vec<&WsFrame> {
        self.replay.replay_by_connection(connection_id)
    }
    pub fn replay_sequence(&self, frame_ids: &[Uuid]) -> Vec<&WsFrame> {
        self.replay.replay_sequence(frame_ids)
    }

    // ── Search ──

    pub fn search_payload(&self, query: &str) -> Vec<&WsFrame> {
        self.frames
            .iter()
            .filter(|f| {
                f.payload_as_text()
                    .map(|t| t.contains(query))
                    .unwrap_or(false)
            })
            .collect()
    }

    pub fn search_by_opcode(&self, opcode: WsOpcode) -> Vec<&WsFrame> {
        self.frames.iter().filter(|f| f.opcode == opcode).collect()
    }

    pub fn connections(&self) -> &[WsConnection] {
        &self.connections
    }
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    pub fn record_handshake(
        &mut self,
        connection_id: Uuid,
        method: &str,
        url: &str,
        request_headers: Vec<(String, String)>,
        response_headers: Vec<(String, String)>,
        response_status: u16,
    ) -> &WsHandshake {
        let mut hs = WsHandshake::new(connection_id, method, url);
        hs.request_headers = request_headers;
        hs.response_headers = response_headers;
        hs.response_status = Some(response_status);

        if let Some(conn) = self.connections.iter_mut().find(|c| c.id == connection_id) {
            conn.subprotocol = hs.subprotocol.clone();
            conn.extensions = hs.extensions.clone();
        }

        self.handshakes.push(hs);
        self.handshakes.last().unwrap()
    }

    pub fn get_handshake(&self, connection_id: Uuid) -> Option<&WsHandshake> {
        self.handshakes
            .iter()
            .find(|h| h.connection_id == connection_id)
    }

    pub fn get_or_create_conversation(&mut self, connection_id: Uuid) -> &mut WsConversation {
        let exists = self
            .conversations
            .iter()
            .any(|c| c.connection_id == connection_id);
        if !exists {
            self.conversations.push(WsConversation::new(connection_id));
        }
        self.conversations
            .iter_mut()
            .find(|c| c.connection_id == connection_id)
            .unwrap()
    }

    pub fn get_conversation(&self, connection_id: Uuid) -> Option<&WsConversation> {
        self.conversations
            .iter()
            .find(|c| c.connection_id == connection_id)
    }

    pub fn conversations(&self) -> &[WsConversation] {
        &self.conversations
    }
}
