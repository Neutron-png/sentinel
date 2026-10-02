#![allow(dead_code)]

use base64::Engine;

use crate::decoder::errors::DecoderError;
use crate::decoder::models::{HashCandidate, JwtParts, Transform};

fn is_url_safe_unreserved(b: u8) -> bool {
    matches!(b, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~')
}

pub fn apply(input: &str, transform: Transform) -> Result<String, DecoderError> {
    match transform {
        Transform::UrlEncode => Ok(url_encode(input, false)),
        Transform::UrlDecode => url_decode(input, false),
        Transform::UrlDecodeAll => url_decode(input, true),
        Transform::Base64Encode => {
            Ok(base64::engine::general_purpose::STANDARD.encode(input))
        }
        Transform::Base64UrlEncode => {
            Ok(base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(input))
        }
        Transform::Base64Decode => base64_decode(input, false),
        Transform::Base64UrlDecode => base64_decode(input, true),
        Transform::HexEncode => Ok(hex_encode(input)),
        Transform::HexDecode => hex_decode(input),
        Transform::HtmlEncode => Ok(html_encode(input)),
        Transform::HtmlDecode => Ok(html_decode(input)),
        Transform::UnicodeEncode => Ok(unicode_encode(input)),
        Transform::UnicodeDecode => unicode_decode(input),
        Transform::JsonFormat => json_format(input, true),
        Transform::JsonMinify => json_format(input, false),
        Transform::JwtDecode => jwt_decode(input).map(|j| jwt_to_string(&j)),
        Transform::ToUpper => Ok(input.to_uppercase()),
        Transform::ToLower => Ok(input.to_lowercase()),
        Transform::Reverse => Ok(input.chars().rev().collect()),
    }
}

pub fn apply_chain(input: &str, chain: &[Transform]) -> Result<String, DecoderError> {
    let mut current = input.to_string();
    for transform in chain {
        let next = apply(&current, *transform)
            .map_err(|e| DecoderError::UnknownTransform(format!("{}: {e}", transform.label())))?;
        current = next;
    }
    Ok(current)
}

fn url_encode(value: &str, plus_for_space: bool) -> String {
    let mut out = String::with_capacity(value.len());
    for b in value.as_bytes() {
        if is_url_safe_unreserved(*b) {
            out.push(*b as char);
        } else if *b == b' ' && plus_for_space {
            out.push('+');
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

fn url_decode(value: &str, recursive: bool) -> Result<String, DecoderError> {
    let decoded = url_decode_once(value)?;
    if recursive {
        let mut current = decoded;
        loop {
            let next = url_decode_once(&current)?;
            if next == current {
                return Ok(current);
            }
            current = next;
        }
    }
    Ok(decoded)
}

fn url_decode_once(value: &str) -> Result<String, DecoderError> {
    let bytes = value.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'%' => {
                if i + 2 >= bytes.len() {
                    return Err(DecoderError::InvalidUrl(format!(
                        "truncated percent escape at byte {i}"
                    )));
                }
                let hex = &value[i + 1..i + 3];
                let byte = u8::from_str_radix(hex, 16).map_err(|_| {
                    DecoderError::InvalidUrl(format!("invalid percent escape %{hex} at byte {i}"))
                })?;
                out.push(byte);
                i += 3;
            }
            b'+' => {
                out.push(b' ');
                i += 1;
            }
            b => {
                out.push(b);
                i += 1;
            }
        }
    }
    String::from_utf8(out).map_err(|e| DecoderError::Utf8(e.to_string()))
}

fn base64_decode(input: &str, url_safe: bool) -> Result<String, DecoderError> {
    let trimmed: String = input.chars().filter(|c| !c.is_whitespace()).collect();
    let bytes = if url_safe {
        base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(trimmed.as_bytes())
            .or_else(|_| base64::engine::general_purpose::URL_SAFE.decode(trimmed.as_bytes()))
            .map_err(|e| DecoderError::InvalidBase64(e.to_string()))?
    } else {
        base64::engine::general_purpose::STANDARD
            .decode(trimmed.as_bytes())
            .map_err(|e| DecoderError::InvalidBase64(e.to_string()))?
    };
    String::from_utf8(bytes).map_err(|e| DecoderError::Utf8(e.to_string()))
}

fn hex_encode(value: &str) -> String {
    value.as_bytes().iter().map(|b| format!("{b:02x}")).collect()
}

fn hex_decode(value: &str) -> Result<String, DecoderError> {
    let cleaned: String = value
        .chars()
        .filter(|c| !c.is_whitespace() && *c != ':' && *c != '-')
        .collect();
    if cleaned.len() % 2 != 0 {
        return Err(DecoderError::InvalidHex(
            "hex input must have an even number of digits".into(),
        ));
    }
    let bytes = cleaned
        .as_bytes()
        .chunks(2)
        .map(|pair| {
            let s = std::str::from_utf8(pair).unwrap_or("");
            u8::from_str_radix(s, 16)
                .map_err(|_| DecoderError::InvalidHex(format!("invalid hex byte '{s}'")))
        })
        .collect::<Result<Vec<u8>, _>>()?;
    String::from_utf8(bytes).map_err(|e| DecoderError::Utf8(e.to_string()))
}

fn html_encode(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for c in value.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#x27;"),
            c if c.is_ascii() => out.push(c),
            c => out.push_str(&format!("&#{};", c as u32)),
        }
    }
    out
}

fn html_decode(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut chars = value.char_indices().peekable();
    while let Some((_, c)) = chars.next() {
        if c != '&' {
            out.push(c);
            continue;
        }
        let mut entity = String::new();
        let mut consumed = 0usize;
        let mut closed = false;
        while let Some((_, ec)) = chars.peek().copied() {
            if ec == ';' {
                chars.next();
                consumed += 1;
                closed = true;
                break;
            }
            if entity.len() > 12 || (!ec.is_ascii_alphanumeric() && ec != '#') {
                break;
            }
            entity.push(ec);
            chars.next();
            consumed += 1;
        }
        if !closed {
            out.push('&');
            out.push_str(&entity);
            continue;
        }
        let _ = consumed;
        let replacement = decode_entity(&entity);
        match replacement {
            Some(text) => out.push_str(&text),
            None => {
                out.push('&');
                out.push_str(&entity);
                out.push(';');
            }
        }
    }
    out
}

fn decode_entity(entity: &str) -> Option<String> {
    let named = match entity.to_ascii_lowercase().as_str() {
        "amp" => Some('&'),
        "lt" => Some('<'),
        "gt" => Some('>'),
        "quot" => Some('"'),
        "apos" => Some('\''),
        "nbsp" => Some('\u{a0}'),
        _ => None,
    };
    if let Some(c) = named {
        return Some(c.to_string());
    }
    let codepoint = if let Some(hex) = entity
        .strip_prefix("#x")
        .or_else(|| entity.strip_prefix("#X"))
    {
        u32::from_str_radix(hex, 16).ok()
    } else if let Some(dec) = entity.strip_prefix('#') {
        dec.parse::<u32>().ok()
    } else {
        None
    };
    codepoint.and_then(char::from_u32).map(|c| c.to_string())
}

fn unicode_encode(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for c in value.chars() {
        match c {
            c if c.is_ascii() => out.push(c),
            c if (c as u32) <= 0xFFFF => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => {
                let cp = c as u32 - 0x10000;
                let high = 0xD800 + (cp >> 10);
                let low = 0xDC00 + (cp & 0x3FF);
                out.push_str(&format!("\\u{high:04x}\\u{low:04x}"));
            }
        }
    }
    out
}

fn unicode_decode(value: &str) -> Result<String, DecoderError> {
    let mut out = String::with_capacity(value.len());
    let chars: Vec<char> = value.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c == '\\' && i + 1 < chars.len() && chars[i + 1] == 'u' {
            let code = read_unicode_escape(&chars, &mut i)?;
            out.push(code);
        } else if c == '\\' && i + 1 < chars.len() && chars[i + 1] == 'x' {
            if i + 3 >= chars.len() {
                return Err(DecoderError::InvalidUnicode("truncated \\x escape".into()));
            }
            let hex: String = chars[i + 2..i + 4].iter().collect();
            let byte = u8::from_str_radix(&hex, 16)
                .map_err(|_| DecoderError::InvalidUnicode(format!("invalid \\x{hex}")))?;
            out.push(byte as char);
            i += 4;
        } else {
            out.push(c);
            i += 1;
        }
    }
    Ok(out)
}

fn read_unicode_escape(chars: &[char], i: &mut usize) -> Result<char, DecoderError> {
    let read4 = |start: usize| -> Result<u32, DecoderError> {
        if start + 4 > chars.len() {
            return Err(DecoderError::InvalidUnicode("truncated \\u escape".into()));
        }
        let hex: String = chars[start..start + 4].iter().collect();
        u32::from_str_radix(&hex, 16)
            .map_err(|_| DecoderError::InvalidUnicode(format!("invalid \\u{hex}")))
    };
    let first = read4(*i + 2)?;
    *i += 6;
    if (0xD800..=0xDBFF).contains(&first) && *i + 1 < chars.len() && chars[*i] == '\\' && chars[*i + 1] == 'u'
    {
        let second = read4(*i + 2)?;
        *i += 6;
        if (0xDC00..=0xDFFF).contains(&second) {
            let cp = 0x10000 + ((first - 0xD800) << 10) + (second - 0xDC00);
            return char::from_u32(cp)
                .ok_or_else(|| DecoderError::InvalidUnicode("invalid surrogate pair".into()));
        }
    }
    char::from_u32(first)
        .ok_or_else(|| DecoderError::InvalidUnicode(format!("invalid codepoint U+{first:04X}")))
}

fn json_format(input: &str, pretty: bool) -> Result<String, DecoderError> {
    let value: serde_json::Value =
        serde_json::from_str(input).map_err(|e| DecoderError::InvalidJson(e.to_string()))?;
    if pretty {
        serde_json::to_string_pretty(&value).map_err(|e| DecoderError::InvalidJson(e.to_string()))
    } else {
        serde_json::to_string(&value).map_err(|e| DecoderError::InvalidJson(e.to_string()))
    }
}

fn b64url_decode(segment: &str) -> Result<Vec<u8>, DecoderError> {
    base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(segment.as_bytes())
        .or_else(|_| base64::engine::general_purpose::URL_SAFE.decode(segment.as_bytes()))
        .map_err(|e| DecoderError::InvalidJwt(e.to_string()))
}

pub fn jwt_decode(token: &str) -> Result<JwtParts, DecoderError> {
    let parts: Vec<&str> = token.trim().split('.').collect();
    if parts.len() != 3 {
        return Err(DecoderError::InvalidJwt(format!(
            "expected 3 dot-separated segments, found {}",
            parts.len()
        )));
    }
    let header_bytes = b64url_decode(parts[0])?;
    let payload_bytes = b64url_decode(parts[1])?;
    let header: serde_json::Value = serde_json::from_slice(&header_bytes)
        .map_err(|e| DecoderError::InvalidJwt(format!("header: {e}")))?;
    let payload: serde_json::Value = serde_json::from_slice(&payload_bytes)
        .map_err(|e| DecoderError::InvalidJwt(format!("payload: {e}")))?;
    if !header.is_object() {
        return Err(DecoderError::InvalidJwt("header is not a JSON object".into()));
    }
    Ok(JwtParts {
        header,
        payload,
        signature_present: !parts[2].is_empty(),
    })
}

fn jwt_to_string(parts: &JwtParts) -> String {
    let mut out = String::new();
    out.push_str("Header:\n");
    out.push_str(
        &serde_json::to_string_pretty(&parts.header).unwrap_or_else(|_| "{}".into()),
    );
    out.push_str("\n\nPayload:\n");
    out.push_str(
        &serde_json::to_string_pretty(&parts.payload).unwrap_or_else(|_| "{}".into()),
    );
    out.push_str(&format!(
        "\n\nSignature present: {}",
        parts.signature_present
    ));
    out
}

pub fn identify_hashes(input: &str) -> Vec<HashCandidate> {
    let value = input.trim();
    let mut candidates = Vec::new();
    let is_hex = !value.is_empty() && value.chars().all(|c| c.is_ascii_hexdigit());
    if is_hex {
        match value.len() {
            32 => candidates.push(HashCandidate {
                algorithm: "MD5".into(),
                confidence: 60,
                reason: "32 hex characters (MD5 digest length)".into(),
            }),
            40 => candidates.push(HashCandidate {
                algorithm: "SHA-1".into(),
                confidence: 60,
                reason: "40 hex characters (SHA-1 digest length)".into(),
            }),
            56 => candidates.push(HashCandidate {
                algorithm: "SHA-224 / SHA3-224".into(),
                confidence: 45,
                reason: "56 hex characters".into(),
            }),
            64 => candidates.push(HashCandidate {
                algorithm: "SHA-256 / SHA3-256".into(),
                confidence: 60,
                reason: "64 hex characters".into(),
            }),
            96 => candidates.push(HashCandidate {
                algorithm: "SHA-384 / SHA3-384".into(),
                confidence: 55,
                reason: "96 hex characters".into(),
            }),
            128 => candidates.push(HashCandidate {
                algorithm: "SHA-512 / SHA3-512 / Whirlpool".into(),
                confidence: 55,
                reason: "128 hex characters".into(),
            }),
            _ => candidates.push(HashCandidate {
                algorithm: "Hex-encoded data".into(),
                confidence: 20,
                reason: format!("{} hex characters matches no common digest length", value.len()),
            }),
        }
    }
    if value.starts_with("$2a$")
        || value.starts_with("$2b$")
        || value.starts_with("$2y$")
        || value.starts_with("$2x$")
    {
        candidates.push(HashCandidate {
            algorithm: "bcrypt".into(),
            confidence: 95,
            reason: "modular crypt format $2*$ prefix".into(),
        });
    }
    if value.starts_with("$argon2") {
        candidates.push(HashCandidate {
            algorithm: "Argon2".into(),
            confidence: 95,
            reason: "$argon2 prefix".into(),
        });
    }
    if value.starts_with("$pbkdf2") || value.starts_with("$pbkdf2-sha") {
        candidates.push(HashCandidate {
            algorithm: "PBKDF2".into(),
            confidence: 85,
            reason: "$pbkdf2 prefix".into(),
        });
    }
    if value.starts_with("$scrypt$") {
        candidates.push(HashCandidate {
            algorithm: "scrypt".into(),
            confidence: 85,
            reason: "$scrypt prefix".into(),
        });
    }
    if value.starts_with("{SSHA}") || value.starts_with("{SHA}") || value.starts_with("{MD5}") {
        candidates.push(HashCandidate {
            algorithm: "LDAP-style digest".into(),
            confidence: 80,
            reason: "LDAP {scheme} prefix".into(),
        });
    }
    candidates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_roundtrip_and_recursive() {
        let encoded = apply("a b&c", Transform::UrlEncode).unwrap();
        assert_eq!(encoded, "a%20b%26c");
        assert_eq!(apply(&encoded, Transform::UrlDecode).unwrap(), "a b&c");
        assert_eq!(
            apply("a%2520b", Transform::UrlDecodeAll).unwrap(),
            "a b"
        );
    }

    #[test]
    fn url_decode_rejects_bad_escapes() {
        assert!(apply("%", Transform::UrlDecode).is_err());
        assert!(apply("%zz", Transform::UrlDecode).is_err());
    }

    #[test]
    fn base64_and_base64url() {
        let e = apply("admin", Transform::Base64Encode).unwrap();
        assert_eq!(e, "YWRtaW4=");
        assert_eq!(apply(&e, Transform::Base64Decode).unwrap(), "admin");
        let url = apply("a+b", Transform::Base64UrlEncode).unwrap();
        assert!(!url.contains('+') && !url.contains('/'));
        assert_eq!(apply(&url, Transform::Base64UrlDecode).unwrap(), "a+b");
    }

    #[test]
    fn hex_roundtrip() {
        let e = apply("AB", Transform::HexEncode).unwrap();
        assert_eq!(e, "4142");
        assert_eq!(apply("41:42", Transform::HexDecode).unwrap(), "AB");
        assert!(apply("414", Transform::HexDecode).is_err());
    }

    #[test]
    fn html_entities() {
        assert_eq!(apply("<a>&'\"", Transform::HtmlEncode).unwrap(), "&lt;a&gt;&amp;&#x27;&quot;");
        assert_eq!(apply("&lt;&#65;&#x42;&amp;", Transform::HtmlDecode).unwrap(), "<AB&");
        assert_eq!(apply("&unknown;", Transform::HtmlDecode).unwrap(), "&unknown;");
    }

    #[test]
    fn unicode_escapes() {
        assert_eq!(apply("A\u{e9}", Transform::UnicodeEncode).unwrap(), "A\\u00e9");
        assert_eq!(apply("A\\u00e9", Transform::UnicodeDecode).unwrap(), "A\u{e9}");
        assert_eq!(apply("\\x41", Transform::UnicodeDecode).unwrap(), "A");
        assert!(apply("\\uZZZZ", Transform::UnicodeDecode).is_err());
    }

    #[test]
    fn json_format_and_minify() {
        let pretty = apply("{\"a\":1}", Transform::JsonFormat).unwrap();
        assert!(pretty.contains('\n'));
        assert_eq!(apply(&pretty, Transform::JsonMinify).unwrap(), "{\"a\":1}");
        assert!(apply("{bad", Transform::JsonFormat).is_err());
    }

    #[test]
    fn jwt_decode_valid() {
        let token = "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxMjMifQ.sig";
        let parts = jwt_decode(token).unwrap();
        assert_eq!(parts.header["alg"], "HS256");
        assert_eq!(parts.payload["sub"], "123");
        assert!(parts.signature_present);
        assert!(jwt_decode("nope").is_err());
    }

    #[test]
    fn chaining() {
        let out = apply_chain(
            "a b",
            &[Transform::UrlEncode, Transform::ToUpper],
        )
        .unwrap();
        assert_eq!(out, "A%20B");
    }

    #[test]
    fn hash_identification_is_evidence_based() {
        let md5 = identify_hashes("d41d8cd98f00b204e9800998ecf8427e");
        assert!(md5.iter().any(|c| c.algorithm.contains("MD5")));
        let bcrypt = identify_hashes("$2b$12$abcdefghijklmnopqrstuv");
        assert!(bcrypt.iter().any(|c| c.algorithm == "bcrypt"));
        assert!(identify_hashes("not a hash").is_empty());
    }
}
