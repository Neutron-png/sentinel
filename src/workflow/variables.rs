#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Default)]
pub struct VariableStore {
    vars: HashMap<String, String>,
}

impl VariableStore {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }
    pub fn set(&mut self, name: &str, value: &str) {
        self.vars.insert(name.to_string(), value.to_string());
    }
    pub fn get(&self, name: &str) -> Option<&String> {
        self.vars.get(name)
    }
    pub fn replace_in(&self, text: &str) -> String {
        let mut result = text.to_string();
        for (k, v) in &self.vars {
            result = result.replace(&format!("{{{{{}}}}}", k), v);
        }
        result
    }
    pub fn extract_html(_html: &str, _selector: &str) -> String {
        String::new()
    }
    pub fn extract_json(_json: &str, _path: &str) -> String {
        String::new()
    }
    pub fn extract_regex(_text: &str, _pattern: &str) -> String {
        String::new()
    }
}
