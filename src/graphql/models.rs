#![allow(dead_code)]

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct GraphQlRequest {
    pub id: Uuid,
    pub endpoint: String,
    pub query: String,
    pub variables: serde_json::Value,
    pub operation_name: Option<String>,
    pub method: String,
}

impl GraphQlRequest {
    pub fn new(endpoint: &str, query: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            endpoint: endpoint.to_string(),
            query: query.to_string(),
            variables: serde_json::Value::Object(serde_json::Map::new()),
            operation_name: None,
            method: "POST".into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GraphQlSchema {
    pub types: Vec<GraphQlType>,
    pub queries: Vec<GraphQlField>,
    pub mutations: Vec<GraphQlField>,
    pub subscriptions: Vec<GraphQlField>,
    pub enums: Vec<String>,
    pub interfaces: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct GraphQlType {
    pub name: String,
    pub kind: String,
    pub fields: Vec<GraphQlField>,
}

#[derive(Debug, Clone)]
pub struct GraphQlField {
    pub name: String,
    pub field_type: String,
    pub args: Vec<GraphQlArg>,
}

#[derive(Debug, Clone)]
pub struct GraphQlArg {
    pub name: String,
    pub arg_type: String,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GraphQlIntrospectionResult {
    pub success: bool,
    pub schema: Option<GraphQlSchema>,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub enum GraphQlOperation {
    Query,
    Mutation,
    Subscription,
}

#[derive(Debug, Clone)]
pub struct GraphQlInjectionPoint {
    pub field: String,
    pub arg: String,
    pub var_name: Option<String>,
    pub nested_path: Vec<String>,
}
