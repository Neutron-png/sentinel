#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeRule {
    pub id: Uuid,
    pub assessment_id: Uuid,
    pub rule_type: RuleType,
    pub value: String,
    pub action: RuleAction,
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuleType {
    ExactHost,
    WildcardHost,
    Domain,
    Subdomain,
    IPv4,
    IPv6,
    Cidr,
    Port,
    Scheme,
    UrlPrefix,
    Regex,
}

impl RuleType {
    pub fn label(&self) -> &'static str {
        match self {
            Self::ExactHost => "Exact Host",
            Self::WildcardHost => "Wildcard Host",
            Self::Domain => "Domain",
            Self::Subdomain => "Subdomain",
            Self::IPv4 => "IPv4",
            Self::IPv6 => "IPv6",
            Self::Cidr => "CIDR",
            Self::Port => "Port",
            Self::Scheme => "Scheme",
            Self::UrlPrefix => "URL Prefix",
            Self::Regex => "Regex",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuleAction {
    Include,
    Exclude,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeVerdict {
    InScope,
    OutOfScope,
    Ignored,
}
