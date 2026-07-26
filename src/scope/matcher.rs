#![allow(dead_code)]

use crate::scope::models::{RuleType, ScopeRule, ScopeVerdict};

pub fn matches_rule(rule: &ScopeRule, host: &str, port: u16, scheme: &str, url: &str) -> bool {
    if !rule.enabled {
        return false;
    }
    match rule.rule_type {
        RuleType::ExactHost => host.eq_ignore_ascii_case(&rule.value),
        RuleType::WildcardHost => wildcard_match(&rule.value, host),
        RuleType::Domain => host.ends_with(&rule.value) || host.eq_ignore_ascii_case(&rule.value),
        RuleType::Subdomain => host.ends_with(&format!(".{}", rule.value)),
        RuleType::Port => port.to_string() == rule.value,
        RuleType::Scheme => scheme.eq_ignore_ascii_case(&rule.value),
        RuleType::UrlPrefix => url.to_lowercase().starts_with(&rule.value.to_lowercase()),
        RuleType::IPv4 | RuleType::IPv6 => host == rule.value,
        RuleType::Cidr | RuleType::Regex => false,
    }
}

fn wildcard_match(pattern: &str, host: &str) -> bool {
    if let Some(suffix) = pattern.strip_prefix("*.") {
        return host.ends_with(suffix) || host.eq_ignore_ascii_case(suffix);
    }
    pattern.eq_ignore_ascii_case(host)
}

pub fn evaluate(
    rules: &[ScopeRule],
    host: &str,
    port: u16,
    scheme: &str,
    url: &str,
) -> ScopeVerdict {
    let mut has_include = false;
    let mut matched = false;
    for rule in rules {
        if matches_rule(rule, host, port, scheme, url) {
            match rule.action {
                crate::scope::models::RuleAction::Exclude => return ScopeVerdict::OutOfScope,
                crate::scope::models::RuleAction::Include => {
                    matched = true;
                    has_include = true;
                }
            }
        }
        if rule.action == crate::scope::models::RuleAction::Include {
            has_include = true;
        }
    }
    if matched {
        ScopeVerdict::InScope
    } else if has_include {
        ScopeVerdict::OutOfScope
    } else {
        ScopeVerdict::Ignored
    }
}
