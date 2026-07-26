use super::models::{GrpcAnalysisFinding, GrpcMessage};

pub fn analyze_grpc_message(msg: &GrpcMessage) -> Vec<GrpcAnalysisFinding> {
    let mut findings = Vec::new();

    findings.extend(check_sensitive_fields(msg));
    findings.extend(check_missing_auth(msg));
    findings.extend(check_large_messages(msg));
    findings.extend(check_grpc_status(msg));

    findings
}

fn check_sensitive_fields(msg: &GrpcMessage) -> Vec<GrpcAnalysisFinding> {
    let mut findings = Vec::new();
    let all_fields: Vec<&super::models::ProtobufField> = msg
        .fields
        .iter()
        .chain(msg.response_fields.iter())
        .collect();

    for field in &all_fields {
        if field.is_sensitive() {
            let name = field.name.as_deref().unwrap_or("unknown");
            findings.push(GrpcAnalysisFinding {
                title: format!("gRPC message contains sensitive field: {}", name),
                description: format!(
                    "Field '{}' (field_number={}) appears to contain sensitive data. gRPC messages may contain PII, credentials, or internal data that should not be transmitted without proper encryption and access controls.",
                    name, field.field_number
                ),
                severity: "Medium".into(),
                confidence: "Medium".into(),
                recommendation: "Review gRPC message schemas to ensure sensitive fields are not unnecessary. Use field-level encryption or authentication at the service/method level. Consider using gRPC interceptors for data masking.".into(),
                cwe: Some("CWE-359".into()),
                owasp_category: Some("A04:2021 - Insecure Design".into()),
                service: msg.service.clone(),
                method: msg.method.clone(),
                evidence: format!("Field: {} (number {})", name, field.field_number),
            });
        }
    }

    findings
}

fn check_missing_auth(msg: &GrpcMessage) -> Option<GrpcAnalysisFinding> {
    let has_auth = msg
        .metadata
        .iter()
        .any(|(k, _)| k.to_lowercase() == "authorization" || k.to_lowercase().contains("auth"));

    if !has_auth {
        Some(GrpcAnalysisFinding {
            title: "gRPC call without authentication metadata".into(),
            description: format!(
                "The gRPC call to {}/{} does not include Authorization headers. Unauthenticated gRPC endpoints may expose sensitive functionality to unauthorized callers.",
                msg.service.as_deref().unwrap_or("unknown"),
                msg.method.as_deref().unwrap_or("unknown")
            ),
            severity: "High".into(),
            confidence: "High".into(),
            recommendation: "Implement authentication for all gRPC services using TLS mutual authentication, JWT tokens, or OAuth2. gRPC supports interceptors that can enforce authentication at the server level.".into(),
            cwe: Some("CWE-306".into()),
            owasp_category: Some("A01:2021 - Broken Access Control".into()),
            service: msg.service.clone(),
            method: msg.method.clone(),
            evidence: "No Authorization or authentication-related headers found in gRPC metadata.".into(),
        })
    } else {
        None
    }
}

fn check_large_messages(msg: &GrpcMessage) -> Option<GrpcAnalysisFinding> {
    let total_size: usize = msg
        .request_frames
        .iter()
        .map(|f| f.data.len())
        .sum::<usize>()
        + msg
            .response_frames
            .iter()
            .map(|f| f.data.len())
            .sum::<usize>();

    let max_size: usize = 4 * 1024 * 1024; // 4MB
    if total_size > max_size {
        Some(GrpcAnalysisFinding {
            title: "Unusually large gRPC message detected".into(),
            description: format!(
                "The gRPC call has a total message size of {} bytes that exceeds the default {} byte limit. Large messages may indicate data exfiltration or excessive data exposure.",
                total_size, max_size
            ),
            severity: "Low".into(),
            confidence: "Medium".into(),
            recommendation: "Set appropriate gRPC message size limits (default 4MB). Use streaming for large data transfers. Review whether all transmitted data is necessary.".into(),
            cwe: Some("CWE-770".into()),
            owasp_category: Some("A04:2021 - Insecure Design".into()),
            service: msg.service.clone(),
            method: msg.method.clone(),
            evidence: format!("Total message size: {} bytes", total_size),
        })
    } else {
        None
    }
}

fn check_grpc_status(msg: &GrpcMessage) -> Option<GrpcAnalysisFinding> {
    match msg.grpc_status {
        Some(0) => None,
        Some(code) if code >= 13 => Some(GrpcAnalysisFinding {
            title: format!("gRPC call returned error status: {}", code),
            description: format!(
                "The gRPC call returned status code {}{}. Internal errors may leak information about the service implementation.",
                code,
                msg.grpc_message.as_deref().map(|m| format!(": {}", m)).unwrap_or_default()
            ),
            severity: "Informational".into(),
            confidence: "High".into(),
            recommendation: "Ensure gRPC error responses do not leak sensitive implementation details. Use generic error messages for internal errors.".into(),
            cwe: Some("CWE-209".into()),
            owasp_category: Some("A04:2021 - Insecure Design".into()),
            service: msg.service.clone(),
            method: msg.method.clone(),
            evidence: format!("gRPC status: {} ({})", code, msg.grpc_message.as_deref().unwrap_or("")),
        }),
        Some(_) => None,
        None => None,
    }
}
