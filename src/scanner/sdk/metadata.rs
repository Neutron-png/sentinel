#![allow(dead_code)]

#[derive(Debug, Clone)]
pub struct RuleMetadata {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub version: String,
    pub author: String,
    pub severity: super::result::RuleSeverity,
    pub confidence: super::result::RuleConfidence,
    pub tags: Vec<String>,
    pub supported_methods: Vec<String>,
    pub supported_content_types: Vec<String>,
}

impl RuleMetadata {
    pub fn new(id: &str, name: &str, category: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: String::new(),
            category: category.to_string(),
            version: "1.0".into(),
            author: String::new(),
            severity: super::result::RuleSeverity::Medium,
            confidence: super::result::RuleConfidence::Medium,
            tags: vec![],
            supported_methods: vec!["GET".into(), "POST".into()],
            supported_content_types: vec![],
        }
    }

    pub fn with_severity(mut self, s: super::result::RuleSeverity) -> Self {
        self.severity = s;
        self
    }
    pub fn with_confidence(mut self, c: super::result::RuleConfidence) -> Self {
        self.confidence = c;
        self
    }
    pub fn with_tags(mut self, t: Vec<&str>) -> Self {
        self.tags = t.into_iter().map(|s| s.to_string()).collect();
        self
    }
}
