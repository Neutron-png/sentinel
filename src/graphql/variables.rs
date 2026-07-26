#![allow(dead_code)]

use serde_json::Value;

pub struct VariableManager {
    variables: Value,
}

impl VariableManager {
    pub fn new() -> Self {
        Self {
            variables: Value::Object(serde_json::Map::new()),
        }
    }

    pub fn set(&mut self, name: &str, value: &str) {
        if let Value::Object(ref mut map) = self.variables {
            map.insert(name.to_string(), Value::String(value.to_string()));
        }
    }
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }
    pub fn all(&self) -> &Value {
        &self.variables
    }
    pub fn to_json_string(&self) -> String {
        serde_json::to_string(&self.variables).unwrap_or_default()
    }

    pub fn inject_payload(&mut self, var_name: &str, payload: &str) {
        if let Value::Object(ref mut map) = self.variables {
            map.insert(var_name.to_string(), Value::String(payload.to_string()));
        }
    }
}
