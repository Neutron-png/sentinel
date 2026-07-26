#![allow(dead_code)]

pub struct GraphQlDetector;

impl GraphQlDetector {
    pub fn common_endpoints() -> Vec<&'static str> {
        vec![
            "/graphql",
            "/api/graphql",
            "/v1/graphql",
            "/v2/graphql",
            "/query",
            "/gql",
        ]
    }

    pub fn is_graphql_response(body: &str, content_type: &str) -> bool {
        if content_type.contains("application/json") {
            if body.contains("\"__typename\"")
                || body.contains("\"data\"") && body.contains("\"errors\"")
            {
                return true;
            }
            if body.contains("\"queryType\"") || body.contains("\"mutationType\"") {
                return true;
            }
        }
        false
    }

    pub fn is_graphql_request(body: &str) -> bool {
        let s = body.trim();
        s.starts_with("{")
            && (s.contains("query")
                || s.contains("mutation")
                || s.contains("subscription")
                || s.contains("__schema")
                || s.contains("__type"))
    }
}
