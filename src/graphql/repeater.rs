#![allow(dead_code)]

use crate::graphql::models::GraphQlRequest;

pub struct GraphQlRepeater {
    saved: Vec<GraphQlRequest>,
}

impl GraphQlRepeater {
    pub fn new() -> Self {
        Self { saved: Vec::new() }
    }

    pub fn save(&mut self, request: GraphQlRequest) {
        self.saved.push(request);
    }
    pub fn all(&self) -> &[GraphQlRequest] {
        &self.saved
    }
    pub fn get(&self, id: uuid::Uuid) -> Option<&GraphQlRequest> {
        self.saved.iter().find(|r| r.id == id)
    }
    pub fn duplicate(&mut self, id: uuid::Uuid) -> Option<&GraphQlRequest> {
        if let Some(req) = self.saved.iter().find(|r| r.id == id) {
            let mut dup = req.clone();
            dup.id = uuid::Uuid::new_v4();
            self.saved.push(dup);
            self.saved.last()
        } else {
            None
        }
    }
    pub fn delete(&mut self, id: uuid::Uuid) {
        self.saved.retain(|r| r.id != id);
    }
}
