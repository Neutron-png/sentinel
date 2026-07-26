#![allow(dead_code)]

use crate::graphql::models::GraphQlRequest;

pub struct GraphQlEditor {
    request: GraphQlRequest,
    history: Vec<GraphQlRequest>,
}

impl GraphQlEditor {
    pub fn new(endpoint: &str) -> Self {
        Self {
            request: GraphQlRequest::new(endpoint, ""),
            history: Vec::new(),
        }
    }

    pub fn set_query(&mut self, query: &str) {
        self.request.query = query.to_string();
    }
    pub fn set_variables(&mut self, vars: serde_json::Value) {
        self.request.variables = vars;
    }
    pub fn set_operation(&mut self, name: &str) {
        self.request.operation_name = Some(name.to_string());
    }
    pub fn request(&self) -> &GraphQlRequest {
        &self.request
    }
    pub fn save_to_history(&mut self) {
        self.history.push(self.request.clone());
    }
    pub fn history(&self) -> &[GraphQlRequest] {
        &self.history
    }
    pub fn format_json(json: &str) -> String {
        serde_json::from_str::<serde_json::Value>(json)
            .and_then(|v| {
                serde_json::to_string_pretty(&v)
                    .map_err(|_| serde_json::Error::io(std::io::Error::other("")))
            })
            .unwrap_or_else(|_| json.to_string())
    }
}
