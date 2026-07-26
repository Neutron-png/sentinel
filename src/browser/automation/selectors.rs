#![allow(dead_code)]

use crate::browser::automation::models::Selector;

pub struct SelectorEngine;

impl SelectorEngine {
    pub fn resolve_css(_css: &str) -> Vec<Selector> {
        vec![]
    }
    pub fn resolve_xpath(_xpath: &str) -> Vec<Selector> {
        vec![]
    }
    pub fn by_id(id: &str) -> Selector {
        Selector {
            kind: super::models::SelectorKind::Id,
            value: id.to_string(),
        }
    }
    pub fn by_class(class: &str) -> Selector {
        Selector {
            kind: super::models::SelectorKind::Class,
            value: class.to_string(),
        }
    }
    pub fn by_text(text: &str) -> Selector {
        Selector {
            kind: super::models::SelectorKind::Text,
            value: text.to_string(),
        }
    }
    pub fn by_placeholder(text: &str) -> Selector {
        Selector {
            kind: super::models::SelectorKind::Placeholder,
            value: text.to_string(),
        }
    }
}
