#![allow(dead_code)]

use serde_json::Value;

use crate::graphql::detector::GraphQlDetector;
use crate::graphql::editor::GraphQlEditor;
use crate::graphql::errors::GraphQlError;
use crate::graphql::models::GraphQlIntrospectionResult;
use crate::graphql::parser::GraphQlParser;
use crate::graphql::repeater::GraphQlRepeater;
use crate::graphql::schema;
use crate::graphql::validator::GraphQlValidator;
use crate::graphql::variables::VariableManager;

pub struct GraphQlEngine {
    editor: GraphQlEditor,
    repeater: GraphQlRepeater,
    variables: VariableManager,
    introspection: Option<GraphQlIntrospectionResult>,
}

impl GraphQlEngine {
    pub fn new(endpoint: &str) -> Self {
        Self {
            editor: GraphQlEditor::new(endpoint),
            repeater: GraphQlRepeater::new(),
            variables: VariableManager::new(),
            introspection: None,
        }
    }

    pub fn editor(&mut self) -> &mut GraphQlEditor {
        &mut self.editor
    }
    pub fn repeater(&self) -> &GraphQlRepeater {
        &self.repeater
    }

    pub fn detect_endpoint(&self, url: &str) -> bool {
        GraphQlDetector::common_endpoints()
            .iter()
            .any(|e| url.contains(e))
    }

    pub fn is_graphql_response(&self, body: &str, content_type: &str) -> bool {
        GraphQlDetector::is_graphql_response(body, content_type)
    }

    pub fn process_introspection(&mut self, json: &Value) {
        let result = schema::parse_introspection_response(json);
        self.introspection = Some(result);
    }

    pub fn has_introspection(&self) -> bool {
        self.introspection
            .as_ref()
            .map(|r| r.success)
            .unwrap_or(false)
    }

    pub fn schema(&self) -> Option<&crate::graphql::models::GraphQlSchema> {
        self.introspection.as_ref().and_then(|r| r.schema.as_ref())
    }

    pub fn parse_operation(&self, query: &str) -> Option<crate::graphql::models::GraphQlOperation> {
        GraphQlParser::detect_operation(query)
    }

    pub fn validate(&self, query: &str) -> Result<(), GraphQlError> {
        GraphQlValidator::validate_query(query)
    }

    pub fn save_request(&mut self) {
        self.editor.save_to_history();
        self.repeater.save(self.editor.request().clone());
    }

    pub fn introspection(&self) -> Option<&GraphQlIntrospectionResult> {
        self.introspection.as_ref()
    }
}
