#![allow(dead_code)]

use uuid::Uuid;

use crate::websocket::models::WsFrame;

#[derive(Default)]
pub struct ReplayManager {
    history: Vec<(Uuid, WsFrame)>,
}

impl ReplayManager {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }

    pub fn record(&mut self, connection_id: Uuid, frame: &WsFrame) {
        self.history.push((connection_id, frame.clone()));
    }

    pub fn history(&self) -> &[(Uuid, WsFrame)] {
        &self.history
    }

    pub fn replay_one(&self, frame_id: Uuid) -> Option<&WsFrame> {
        self.history
            .iter()
            .find(|(_, f)| f.id == frame_id)
            .map(|(_, f)| f)
    }

    pub fn replay_by_connection(&self, connection_id: Uuid) -> Vec<&WsFrame> {
        self.history
            .iter()
            .filter(|(cid, _)| *cid == connection_id)
            .map(|(_, f)| f)
            .collect()
    }

    pub fn replay_sequence(&self, frame_ids: &[Uuid]) -> Vec<&WsFrame> {
        frame_ids
            .iter()
            .filter_map(|id| self.replay_one(*id))
            .collect()
    }

    pub fn duplicates(&self) -> Vec<WsFrame> {
        self.history
            .iter()
            .filter_map(|(_, f)| {
                let dup = self
                    .history
                    .iter()
                    .filter(|(_, f2)| f2.opcode == f.opcode && f2.payload == f.payload)
                    .count();
                if dup > 0 {
                    Some(f.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}
