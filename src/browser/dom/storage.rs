#![allow(dead_code)]

use crate::browser::dom::models::{CookieInfo, StorageEntry};

pub struct StorageApi;

impl StorageApi {
    pub fn get_cookies() -> Vec<CookieInfo> {
        vec![]
    }
    pub fn set_cookie(_name: &str, _value: &str) {}
    pub fn delete_cookie(_name: &str) {}
    pub fn get_local_storage() -> Vec<StorageEntry> {
        vec![]
    }
    pub fn set_local_storage(_key: &str, _value: &str) {}
    pub fn clear_local_storage() {}
    pub fn get_session_storage() -> Vec<StorageEntry> {
        vec![]
    }
    pub fn set_session_storage(_key: &str, _value: &str) {}
    pub fn clear_session_storage() {}
}
