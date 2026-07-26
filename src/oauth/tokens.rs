use chrono::Utc;

use super::models::TokenInfo;

pub fn parse_token_response(body: &str, content_type: Option<&str>) -> Option<TokenInfo> {
    let is_json = content_type
        .map(|ct| ct.contains("application/json"))
        .unwrap_or(false);
    let is_form = content_type
        .map(|ct| ct.contains("application/x-www-form-urlencoded"))
        .unwrap_or(false);

    let parsed: serde_json::Value = if is_json {
        serde_json::from_str(body).ok()?
    } else if is_form || !is_json {
        let obj: serde_json::Map<String, serde_json::Value> = body
            .split('&')
            .filter_map(|pair| {
                let mut parts = pair.splitn(2, '=');
                let key = parts.next()?.to_string();
                let value = parts
                    .next()
                    .map(|v| {
                        let decoded = urlencoding_decode(v);
                        serde_json::Value::String(decoded)
                    })
                    .unwrap_or(serde_json::Value::Null);
                Some((key, value))
            })
            .collect();
        serde_json::Value::Object(obj)
    } else {
        serde_json::from_str(body).ok()?
    };

    let access_token = parsed
        .get("access_token")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let token_type = parsed
        .get("token_type")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "Bearer".to_string());

    let expires_in = parsed.get("expires_in").and_then(|v| {
        if let Some(n) = v.as_i64() {
            Some(n)
        } else if let Some(s) = v.as_str() {
            s.parse::<i64>().ok()
        } else {
            None
        }
    });

    let scope = parsed
        .get("scope")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let scopes: Vec<String> = scope
        .map(|s| s.split_whitespace().map(|p| p.to_string()).collect())
        .unwrap_or_default();

    let is_jwt = access_token
        .as_ref()
        .map(|t| crate::jwt::parser::is_jwt_format(t))
        .unwrap_or(false);

    let expires_at = expires_in.map(|secs| Utc::now() + chrono::Duration::seconds(secs));

    Some(TokenInfo {
        token_type,
        algorithm: None,
        scopes,
        expires_at,
        issuer: None,
        audience: None,
        is_expired: None,
        lifetime_seconds: expires_in,
        is_jwt,
    })
}

fn urlencoding_decode(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'+' => {
                result.push(' ');
                i += 1;
            }
            b'%' if i + 2 < bytes.len() => {
                if let (Some(hi), Some(lo)) = (hex_val(bytes[i + 1]), hex_val(bytes[i + 2])) {
                    result.push((hi << 4 | lo) as char);
                    i += 3;
                } else {
                    result.push('%');
                    i += 1;
                }
            }
            _ => {
                result.push(bytes[i] as char);
                i += 1;
            }
        }
    }
    result
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_json_token_response() {
        let body = r#"{"access_token":"abc.def.ghi","token_type":"Bearer","expires_in":3600,"scope":"openid profile"}"#;
        let info = parse_token_response(body, Some("application/json"));
        assert!(info.is_some());
        let t = info.unwrap();
        assert_eq!(t.token_type, "Bearer");
        assert_eq!(t.lifetime_seconds, Some(3600));
        assert!(t.scopes.contains(&"openid".to_string()));
        assert!(t.is_jwt);
    }

    #[test]
    fn test_parse_form_token_response() {
        let body = "access_token=xyz123&token_type=Bearer&expires_in=1800&scope=read+write";
        let info = parse_token_response(body, Some("application/x-www-form-urlencoded"));
        assert!(info.is_some());
        let t = info.unwrap();
        assert_eq!(t.token_type, "Bearer");
        assert_eq!(t.lifetime_seconds, Some(1800));
    }
}
