#![allow(dead_code)]

use serde_json::Value;

use crate::browser::dom::errors::DomError;

pub struct JavaScriptApi;

impl JavaScriptApi {
    pub fn execute(_script: &str) -> Result<(), DomError> {
        Ok(())
    }
    pub fn evaluate(_script: &str) -> Result<Value, DomError> {
        Ok(Value::Null)
    }
    pub async fn execute_async(_script: &str) -> Result<(), DomError> {
        Ok(())
    }
    pub async fn evaluate_async(_script: &str) -> Result<Value, DomError> {
        Ok(Value::Null)
    }
}
