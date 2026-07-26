#[allow(dead_code)]
use std::sync::Arc;

use reqwest::cookie::Jar;

#[allow(dead_code)]
pub struct CookieJar {
    jar: Arc<Jar>,
}

#[allow(dead_code)]
impl CookieJar {
    pub fn new() -> Self {
        Self {
            jar: Arc::new(Jar::default()),
        }
    }
    pub fn inner(&self) -> Arc<Jar> {
        self.jar.clone()
    }
    pub fn add_cookie(&self, url: &str, name: &str, value: &str) {
        self.jar
            .add_cookie_str(&format!("{name}={value}; Path=/"), &url.parse().unwrap());
    }
    pub fn clear(&self) {}
}

impl Default for CookieJar {
    fn default() -> Self {
        Self::new()
    }
}
impl Clone for CookieJar {
    fn clone(&self) -> Self {
        Self {
            jar: self.jar.clone(),
        }
    }
}
