#![allow(dead_code)]

pub struct PathNormalizer {
    patterns: Vec<(String, String)>,
}

impl PathNormalizer {
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
        }
    }

    pub fn add_rule(&mut self, pattern: &str, replacement: &str) {
        self.patterns
            .push((pattern.to_string(), replacement.to_string()));
    }

    pub fn normalize(&self, path: &str) -> String {
        let mut result = path.to_string();
        for (pattern, replacement) in &self.patterns {
            result = result.replace(pattern, replacement);
        }
        // Auto-normalize: replace numeric segments with {id}
        result = auto_normalize(&result);
        result
    }

    pub fn with_defaults() -> Self {
        let mut n = Self::new();
        n.add_rule("/{id}", "/{id}");
        n
    }
}

fn auto_normalize(path: &str) -> String {
    path.split('/')
        .map(|segment| {
            if segment.chars().all(|c| c.is_ascii_digit()) {
                "{id}".to_string()
            } else if segment.len() == 36 && segment.chars().filter(|c| *c == '-').count() == 4 {
                "{uuid}".to_string()
            } else {
                segment.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("/")
}
