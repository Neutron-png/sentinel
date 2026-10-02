use crate::define_header_rule;
use crate::scanner::passive::models::{ScanConfidence, ScanSeverity};

define_header_rule!(
    MissingReferrerPolicy,
    "PASSIVE-SEC-005",
    "Missing Referrer-Policy Header",
    "Referrer-Policy",
    "The Referrer-Policy header is not set.",
    "OWASP WSTG-CONF-04\nMDN: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Referrer-Policy\nCWE-200: Exposure of Sensitive Information",
    ScanSeverity::Low,
    ScanConfidence::Medium
);
