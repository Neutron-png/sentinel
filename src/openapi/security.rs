#![allow(dead_code)]

use crate::openapi::models::AuthScheme;

pub fn extract_security(spec: &super::models::OpenApiSpec) -> Vec<AuthScheme> {
    let mut schemes = Vec::new();
    if let Some(components) = &spec.components {
        if let Some(sec_schemes) = components
            .get("securitySchemes")
            .and_then(|s| s.as_object())
        {
            for (name, scheme) in sec_schemes {
                let stype = scheme.get("type").and_then(|v| v.as_str()).unwrap_or("");
                let location = scheme
                    .get("in")
                    .and_then(|v| v.as_str())
                    .unwrap_or("header");
                schemes.push(AuthScheme {
                    scheme_type: stype.to_string(),
                    scheme_name: name.clone(),
                    location: location.to_string(),
                    scheme_format: scheme
                        .get("scheme")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string()),
                });
            }
        }
    }
    schemes
}
