#![allow(dead_code)]

pub struct SpecDiscovery;

impl SpecDiscovery {
    pub fn common_paths() -> Vec<&'static str> {
        vec![
            "/openapi.json",
            "/swagger.json",
            "/swagger.yaml",
            "/swagger.yml",
            "/api-docs",
            "/v3/api-docs",
            "/v1/openapi.json",
            "/v2/api-docs",
        ]
    }

    pub fn is_spec_response(body: &str) -> bool {
        let s = body.trim();
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(s) {
            return v.get("openapi").is_some()
                || v.get("swagger").is_some() && v.get("paths").is_some();
        }
        s.contains("openapi") && s.contains("paths") && s.contains("info")
    }
}
