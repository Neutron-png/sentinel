#![allow(dead_code)]

use crate::browser::dom::models::DomElement;

pub struct ElementApi;

impl ElementApi {
    pub fn query_selector(_selector: &str) -> Option<DomElement> {
        None
    }
    pub fn query_selector_all(_selector: &str) -> Vec<DomElement> {
        vec![]
    }
    pub fn get_element_by_id(_id: &str) -> Option<DomElement> {
        None
    }
    pub fn get_elements_by_class(_class: &str) -> Vec<DomElement> {
        vec![]
    }
    pub fn get_elements_by_tag(_tag: &str) -> Vec<DomElement> {
        vec![]
    }
}
