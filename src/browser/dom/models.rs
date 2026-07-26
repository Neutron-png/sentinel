#![allow(dead_code)]

#[derive(Debug, Clone)]
pub struct DomElement {
    pub tag_name: String,
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub attributes: Vec<(String, String)>,
    pub inner_html: String,
    pub outer_html: String,
    pub text_content: String,
    pub value: Option<String>,
    pub checked: Option<bool>,
    pub selected: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct FormInfo {
    pub action: String,
    pub method: String,
    pub enctype: Option<String>,
    pub name: Option<String>,
    pub inputs: Vec<FormField>,
    pub selects: Vec<FormField>,
    pub textareas: Vec<FormField>,
}

#[derive(Debug, Clone)]
pub struct FormField {
    pub name: String,
    pub field_type: String,
    pub value: String,
    pub required: bool,
    pub placeholder: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LinkInfo {
    pub url: String,
    pub text: String,
    pub element: String,
    pub rel: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ResourceInfo {
    pub url: String,
    pub resource_type: ResourceType,
    pub integrity: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceType {
    Css,
    JavaScript,
    Font,
    Image,
    Media,
    Manifest,
    Favicon,
    Other,
}

#[derive(Debug, Clone)]
pub struct CookieInfo {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    pub secure: bool,
    pub http_only: bool,
    pub same_site: Option<String>,
}

#[derive(Debug, Clone)]
pub struct StorageEntry {
    pub key: String,
    pub value: String,
}
