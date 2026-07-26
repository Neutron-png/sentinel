#![allow(dead_code)]

use crate::scanner::payload::models::Payload;

pub struct PayloadGenerator;

impl PayloadGenerator {
    pub fn from_list(strings: &[&str]) -> Vec<Payload> {
        strings.iter().map(|s| Payload::new(s)).collect()
    }

    pub fn from_file(_path: &str) -> Vec<Payload> {
        // Returns empty - file loading deferred to runtime
        vec![]
    }

    pub fn single(value: &str) -> Payload {
        Payload::new(value)
    }

    pub fn range(prefix: &str, start: usize, end: usize) -> Vec<Payload> {
        (start..=end)
            .map(|i| Payload::new(&format!("{}{}", prefix, i)))
            .collect()
    }

    pub fn combine(a: &[Payload], b: &[Payload]) -> Vec<Payload> {
        let mut results = Vec::new();
        for pa in a {
            for pb in b {
                results.push(Payload::new(&format!("{}{}", pa.value, pb.value)));
            }
        }
        results
    }
}
