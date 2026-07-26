#![allow(dead_code)]

use regex::Regex;

#[derive(Debug, Clone)]
pub struct KeywordMatch {
    pub keyword: String,
    pub count: usize,
    pub positions: Vec<usize>,
}

pub fn count_keywords(text: &str, keywords: &[&str]) -> Vec<KeywordMatch> {
    keywords
        .iter()
        .map(|kw| {
            let mut pos = Vec::new();
            let mut search = text;
            let mut offset = 0;
            while let Some(p) = search.to_lowercase().find(&kw.to_lowercase()) {
                pos.push(offset + p);
                offset += p + kw.len();
                search = &search[p + kw.len()..];
            }
            KeywordMatch {
                keyword: kw.to_string(),
                count: pos.len(),
                positions: pos,
            }
        })
        .collect()
}

pub fn match_regex(text: &str, pattern: &str) -> bool {
    Regex::new(pattern)
        .map(|r| r.is_match(text))
        .unwrap_or(false)
}

pub fn find_all_regex(text: &str, pattern: &str) -> Vec<String> {
    Regex::new(pattern)
        .map(|r| r.find_iter(text).map(|m| m.as_str().to_string()).collect())
        .unwrap_or_default()
}

pub fn normalize_for_diff(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn line_diff(a: &str, b: &str) -> Vec<(DiffLine, String)> {
    let mut results = Vec::new();
    let a_lines: Vec<&str> = a.lines().collect();
    let b_lines: Vec<&str> = b.lines().collect();
    let max = a_lines.len().max(b_lines.len());
    for i in 0..max {
        let al = a_lines.get(i).copied().unwrap_or("");
        let bl = b_lines.get(i).copied().unwrap_or("");
        if al != bl {
            if !al.is_empty() {
                results.push((DiffLine::Removed, al.to_string()));
            }
            if !bl.is_empty() {
                results.push((DiffLine::Added, bl.to_string()));
            }
        }
    }
    results
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffLine {
    Added,
    Removed,
}
