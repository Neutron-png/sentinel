use super::models::GrpcMessage;

const GRPC_CONTENT_TYPE_PREFIXES: &[&str] = &["application/grpc", "application/grpc+proto"];

pub fn is_grpc_content_type(content_type: &str) -> bool {
    let ct = content_type.to_lowercase();
    GRPC_CONTENT_TYPE_PREFIXES
        .iter()
        .any(|prefix| ct.starts_with(prefix))
}

pub fn is_grpc_response(headers: &[(String, String)]) -> bool {
    for (name, value) in headers {
        if name.to_lowercase() == "content-type" && is_grpc_content_type(value) {
            return true;
        }
    }
    false
}

pub fn is_grpc_request(headers: &[(String, String)]) -> bool {
    for (name, value) in headers {
        let lower = name.to_lowercase();
        if lower == "content-type" && is_grpc_content_type(value) {
            return true;
        }
        if lower == "te" && value.to_lowercase().contains("trailers") {
            return true;
        }
    }
    false
}

pub fn extract_grpc_metadata(headers: &[(String, String)]) -> Vec<(String, String)> {
    headers
        .iter()
        .filter(|(name, _)| {
            let lower = name.to_lowercase();
            lower.starts_with("grpc-")
                || lower == ":authority"
                || lower == ":path"
                || lower == ":method"
                || lower == "content-type"
                || lower == "te"
                || lower == "user-agent"
                || lower == "authorization"
        })
        .cloned()
        .collect()
}

pub fn extract_grpc_trailers(headers: &[(String, String)]) -> Vec<(String, String)> {
    headers
        .iter()
        .filter(|(name, _)| {
            let lower = name.to_lowercase();
            lower == "grpc-status" || lower == "grpc-message" || lower == "grpc-status-details"
        })
        .cloned()
        .collect()
}

pub fn extract_service_method(
    url_str: &str,
    headers: &[(String, String)],
) -> (Option<String>, Option<String>) {
    for (name, value) in headers {
        if name == ":path" || name.to_lowercase() == ":path" {
            let path = value.trim_start_matches('/');
            if let Some((service, method)) = path.rsplit_once('/') {
                return (Some(service.to_string()), Some(method.to_string()));
            }
        }
    }
    if let Ok(parsed) = url::Url::parse(url_str) {
        let path = parsed.path().trim_start_matches('/');
        if let Some((service, method)) = path.rsplit_once('/') {
            return (Some(service.to_string()), Some(method.to_string()));
        }
    }
    (None, None)
}

pub fn detect_grpc_call(
    method: &str,
    url_str: &str,
    req_headers: &[(String, String)],
    resp_headers: &[(String, String)],
    status_code: u16,
) -> Option<GrpcMessage> {
    if method != "POST" {
        return None;
    }

    let is_grpc = is_grpc_request(req_headers) || is_grpc_response(resp_headers);
    if !is_grpc && !url_str.contains("/grpc.") && status_code != 200 {
        return None;
    }

    let msg = GrpcMessage {
        id: uuid::Uuid::new_v4(),
        service: None,
        method: None,
        call_type: super::models::GrpcCallType::Unary,
        request_frames: Vec::new(),
        response_frames: Vec::new(),
        metadata: extract_grpc_metadata(req_headers),
        response_metadata: extract_grpc_metadata(resp_headers),
        trailers: extract_grpc_trailers(resp_headers),
        grpc_status: None,
        grpc_message: None,
        fields: Vec::new(),
        response_fields: Vec::new(),
        url: Some(url_str.to_string()),
        detected_at: chrono::Utc::now(),
    };

    Some(msg)
}
