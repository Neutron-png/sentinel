#![allow(dead_code)]

use serde_json::Value;

use crate::openapi::models::SchemaObject;

pub fn parse_schema(name: &str, schema: &Value) -> Option<SchemaObject> {
    let obj = schema.as_object()?;
    let schema_type = obj
        .get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("object")
        .to_string();
    let mut props = Vec::new();
    if let Some(properties) = obj.get("properties").and_then(|v| v.as_object()) {
        for (k, v) in properties {
            let prop_type = v
                .get("type")
                .and_then(|t| t.as_str())
                .unwrap_or("string")
                .to_string();
            props.push((k.clone(), prop_type));
        }
    }
    let required: Vec<String> = obj
        .get("required")
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();
    let enums = obj.get("enum").and_then(|v| v.as_array()).map(|a| {
        a.iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect()
    });
    Some(SchemaObject {
        name: name.to_string(),
        schema_type,
        properties: props,
        required_fields: required,
        enum_values: enums,
    })
}

pub fn parse_all_of(name: &str, schema: &Value) -> Vec<SchemaObject> {
    schema
        .get("allOf")
        .and_then(|a| a.as_array())
        .map(|arr| arr.iter().filter_map(|s| parse_schema(name, s)).collect())
        .unwrap_or_default()
}
