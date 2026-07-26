#![allow(dead_code)]

use serde_json::Value;

use crate::graphql::models::{
    GraphQlField, GraphQlIntrospectionResult, GraphQlSchema, GraphQlType,
};

pub const INTROSPECTION_QUERY: &str = r#"
query IntrospectionQuery {
    __schema { queryType { name } mutationType { name } subscriptionType { name }
        types { name kind fields { name args { name type { name kind } defaultValue } }
        enumValues { name } interfaces { name } possibleTypes { name } }
    }
}
"#;

pub fn parse_introspection_response(json: &Value) -> GraphQlIntrospectionResult {
    let schema = json.get("data").and_then(|d| d.get("__schema"));
    match schema {
        Some(s) => {
            let types = s.get("types").map(parse_types).unwrap_or_default();
            let query_name = s
                .get("queryType")
                .and_then(|q| q.get("name"))
                .and_then(|n| n.as_str())
                .unwrap_or("Query");
            let mutation_name = s
                .get("mutationType")
                .and_then(|q| q.get("name"))
                .and_then(|n| n.as_str())
                .unwrap_or("Mutation");
            GraphQlIntrospectionResult {
                success: true,
                schema: Some(GraphQlSchema {
                    types: types.clone(),
                    queries: find_fields(&types, query_name),
                    mutations: find_fields(&types, mutation_name),
                    subscriptions: vec![],
                    enums: vec![],
                    interfaces: vec![],
                }),
                error: None,
            }
        }
        None => GraphQlIntrospectionResult {
            success: false,
            schema: None,
            error: json.get("errors").map(|e| e.to_string()),
        },
    }
}

fn parse_types(arr: &Value) -> Vec<GraphQlType> {
    arr.as_array()
        .map(|a| {
            a.iter()
                .filter_map(|t| {
                    let fields = t
                        .get("fields")
                        .map(|f| {
                            f.as_array()
                                .map(|fa| {
                                    fa.iter()
                                        .filter_map(|ff| {
                                            Some(GraphQlField {
                                                name: ff.get("name")?.as_str()?.to_string(),
                                                field_type: ff
                                                    .get("type")
                                                    .and_then(|ty| ty.get("name"))
                                                    .and_then(|n| n.as_str())
                                                    .unwrap_or("Unknown")
                                                    .to_string(),
                                                args: vec![],
                                            })
                                        })
                                        .collect()
                                })
                                .unwrap_or_default()
                        })
                        .unwrap_or_default();
                    Some(GraphQlType {
                        name: t.get("name")?.as_str()?.to_string(),
                        kind: t
                            .get("kind")
                            .and_then(|k| k.as_str())
                            .unwrap_or("")
                            .to_string(),
                        fields,
                    })
                })
                .collect()
        })
        .unwrap_or_default()
}

fn find_fields(types: &[GraphQlType], name: &str) -> Vec<GraphQlField> {
    types
        .iter()
        .find(|t| t.name == name)
        .map(|t| t.fields.clone())
        .unwrap_or_default()
}
