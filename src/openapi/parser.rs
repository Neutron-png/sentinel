#![allow(dead_code)]

use serde_json::Value;

use crate::openapi::errors::OpenApiError;
use crate::openapi::models::{
    ApiEndpoint, ApiParameter, ApiResponse, OpenApiSpec, RequestBodySchema,
};

pub fn parse_spec(content: &str, _is_yaml: bool) -> Result<OpenApiSpec, OpenApiError> {
    serde_json::from_str(content).map_err(|e| OpenApiError::Parse(e.to_string()))
}

pub fn extract_endpoints(spec: &OpenApiSpec, base_url: &str) -> Vec<ApiEndpoint> {
    let mut endpoints = Vec::new();
    let paths = match &spec.paths {
        Some(p) => p,
        None => return endpoints,
    };
    let _server_url = spec
        .servers
        .as_ref()
        .and_then(|s| s.first())
        .and_then(|s| s.url.clone())
        .unwrap_or_else(|| base_url.to_string());

    for (path, methods) in paths {
        let methods_obj = methods.as_object();
        if let Some(obj) = methods_obj {
            for (method, op) in obj {
                if !["get", "post", "put", "delete", "patch", "options", "head"]
                    .contains(&method.as_str())
                {
                    continue;
                }
                let params = extract_parameters(op);
                let req_body = extract_request_body(op);
                let responses = extract_responses(op);
                endpoints.push(ApiEndpoint {
                    method: method.to_uppercase(),
                    path: path.clone(),
                    operation_id: op
                        .get("operationId")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    summary: op
                        .get("summary")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                    parameters: params,
                    request_body: req_body,
                    responses,
                    security: vec![],
                    tags: vec![],
                });
            }
        }
    }
    endpoints
}

fn extract_parameters(op: &Value) -> Vec<ApiParameter> {
    op.get("parameters")
        .and_then(|p| p.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|p| {
                    let required = p.get("required").and_then(|v| v.as_bool()).unwrap_or(false);
                    let location = p.get("in").and_then(|v| v.as_str()).unwrap_or("query");
                    let param_type = p
                        .get("schema")
                        .and_then(|s| s.get("type"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("string");
                    let schema_ref = p
                        .get("schema")
                        .and_then(|s| s.get("$ref"))
                        .and_then(|v| v.as_str());
                    Some(ApiParameter {
                        name: p.get("name")?.as_str()?.to_string(),
                        location: location.to_string(),
                        required,
                        param_type: param_type.to_string(),
                        description: p
                            .get("description")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        example: p
                            .get("example")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        schema_ref: schema_ref.map(|s| s.to_string()),
                    })
                })
                .collect()
        })
        .unwrap_or_default()
}

fn extract_request_body(op: &Value) -> Option<RequestBodySchema> {
    op.get("requestBody").and_then(|rb| {
        let required = rb
            .get("required")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        rb.get("content")
            .and_then(|c| c.get("application/json"))
            .map(|json| RequestBodySchema {
                content_type: "application/json".into(),
                schema_ref: json
                    .get("schema")
                    .and_then(|s| s.get("$ref"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                required,
            })
    })
}

fn extract_responses(op: &Value) -> Vec<ApiResponse> {
    op.get("responses")
        .and_then(|r| r.as_object())
        .map(|obj| {
            obj.iter()
                .map(|(code, resp)| ApiResponse {
                    status_code: code.clone(),
                    description: resp
                        .get("description")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    schema_ref: resp
                        .get("content")
                        .and_then(|c| c.get("application/json"))
                        .and_then(|j| j.get("schema"))
                        .and_then(|s| s.get("$ref"))
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                })
                .collect()
        })
        .unwrap_or_default()
}
