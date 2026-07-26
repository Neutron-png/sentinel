#![allow(dead_code)]

use crate::browser::dom::models::ResourceInfo;

pub struct ResourceApi;

impl ResourceApi {
    pub fn extract_css() -> Vec<ResourceInfo> {
        vec![]
    }
    pub fn extract_js() -> Vec<ResourceInfo> {
        vec![]
    }
    pub fn extract_fonts() -> Vec<ResourceInfo> {
        vec![]
    }
    pub fn extract_images() -> Vec<ResourceInfo> {
        vec![]
    }
    pub fn extract_media() -> Vec<ResourceInfo> {
        vec![]
    }
    pub fn get_all_resources() -> Vec<ResourceInfo> {
        vec![]
    }
}
