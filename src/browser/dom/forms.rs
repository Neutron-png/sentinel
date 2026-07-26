#![allow(dead_code)]

use crate::browser::dom::models::{FormField, FormInfo};

pub struct FormApi;

impl FormApi {
    pub fn discover_forms() -> Vec<FormInfo> {
        vec![]
    }
    pub fn get_form_by_id(_id: &str) -> Option<FormInfo> {
        None
    }
    pub fn get_inputs() -> Vec<FormField> {
        vec![]
    }
    pub fn get_hidden_fields() -> Vec<FormField> {
        vec![]
    }
    pub fn get_file_inputs() -> Vec<FormField> {
        vec![]
    }
    pub fn set_input_value(_selector: &str, _value: &str) {}
    pub fn submit(_selector: &str) {}
}
