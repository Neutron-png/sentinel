#![allow(dead_code)]

use crate::graphql::models::GraphQlInjectionPoint;
use crate::network::models::HttpRequest;

pub struct ScannerIntegration;

impl ScannerIntegration {
    pub fn injection_points() -> Vec<GraphQlInjectionPoint> {
        vec![
            GraphQlInjectionPoint {
                field: "query".into(),
                arg: "arguments".into(),
                var_name: None,
                nested_path: vec![],
            },
            GraphQlInjectionPoint {
                field: "mutation".into(),
                arg: "input".into(),
                var_name: Some("input".into()),
                nested_path: vec![],
            },
            GraphQlInjectionPoint {
                field: "query".into(),
                arg: "variables".into(),
                var_name: Some("var".into()),
                nested_path: vec!["variables".into()],
            },
        ]
    }

    pub fn inject_into_argument(
        request: &HttpRequest,
        arg_name: &str,
        payload: &str,
    ) -> HttpRequest {
        let mut req = request.clone();
        let body = format!(
            "{{\"query\":\"{{{{}}}}\",\"variables\":{{\"{}\":\"{}\"}}}}",
            arg_name, payload
        );
        req.body = crate::network::models::HttpBody::Text(body);
        req
    }

    pub fn inject_into_variables(
        request: &HttpRequest,
        var_name: &str,
        payload: &str,
    ) -> HttpRequest {
        let mut req = request.clone();
        let body = format!(
            "{{\"query\":\"query($v:String){{field(arg:$v)}}\",\"variables\":{{\"{}\":\"{}\"}}}}",
            var_name, payload
        );
        req.body = crate::network::models::HttpBody::Text(body);
        req
    }
}
