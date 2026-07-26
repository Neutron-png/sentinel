#![allow(dead_code)]

use base64::{engine::general_purpose, Engine as _};

use crate::scanner::payload::models::{EncoderType, Payload};

pub fn encode(payload: &str, encoder: EncoderType) -> String {
    match encoder {
        EncoderType::Url => urlencoding(payload),
        EncoderType::DoubleUrl => urlencoding(&urlencoding(payload)),
        EncoderType::Html => html_escape(payload),
        EncoderType::Base64 => general_purpose::STANDARD.encode(payload),
        EncoderType::Hex => hex_encode(payload),
        EncoderType::Unicode => unicode_encode(payload),
        EncoderType::JsonEscape => json_escape(payload),
        EncoderType::XmlEscape => xml_escape(payload),
        EncoderType::None => payload.to_string(),
    }
}

pub fn apply_encoding(payload: &mut Payload, encoder: EncoderType) {
    payload.value = encode(&payload.value, encoder);
    payload.encoding = Some(encoder.label().to_string());
}

fn urlencoding(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' || c == '~' {
                c.to_string()
            } else {
                format!("%{:02X}", c as u8)
            }
        })
        .collect()
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

fn hex_encode(s: &str) -> String {
    s.bytes().map(|b| format!("{:02x}", b)).collect()
}

fn unicode_encode(s: &str) -> String {
    s.chars().map(|c| format!("\\u{:04x}", c as u32)).collect()
}

fn json_escape(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
