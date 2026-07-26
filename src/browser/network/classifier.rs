#![allow(dead_code)]

use crate::browser::network::models::ResourceType;

pub struct ResourceClassifier;

impl ResourceClassifier {
    pub fn classify(url: &str, content_type: Option<&str>, initiator: &str) -> ResourceType {
        if let Some(ct) = content_type {
            if ct.contains("text/html") || ct.contains("application/xhtml") {
                return ResourceType::Document;
            }
            if ct.contains("javascript") || ct.contains("ecmascript") {
                return ResourceType::Script;
            }
            if ct.contains("text/css") {
                return ResourceType::Stylesheet;
            }
            if ct.contains("image/") {
                return ResourceType::Image;
            }
            if ct.contains("font/") || ct.contains("application/font") {
                return ResourceType::Font;
            }
            if ct.contains("video/") || ct.contains("audio/") {
                return ResourceType::Media;
            }
            if ct.contains("application/manifest+json") {
                return ResourceType::Manifest;
            }
        }
        if initiator == "fetch" {
            return ResourceType::Fetch;
        }
        if initiator == "xhr" || initiator == "xmlhttprequest" {
            return ResourceType::Xhr;
        }
        if initiator == "beacon" {
            return ResourceType::Beacon;
        }
        let lower = url.to_lowercase();
        if lower.ends_with(".js") {
            return ResourceType::Script;
        }
        if lower.ends_with(".css") {
            return ResourceType::Stylesheet;
        }
        if lower.ends_with(".png")
            || lower.ends_with(".jpg")
            || lower.ends_with(".jpeg")
            || lower.ends_with(".gif")
            || lower.ends_with(".svg")
            || lower.ends_with(".webp")
            || lower.ends_with(".ico")
        {
            return ResourceType::Image;
        }
        if lower.ends_with(".woff")
            || lower.ends_with(".woff2")
            || lower.ends_with(".ttf")
            || lower.ends_with(".eot")
        {
            return ResourceType::Font;
        }
        if lower.ends_with(".mp4")
            || lower.ends_with(".webm")
            || lower.ends_with(".mp3")
            || lower.ends_with(".ogg")
        {
            return ResourceType::Media;
        }
        ResourceType::Other
    }
}
