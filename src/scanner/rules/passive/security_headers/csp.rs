use crate::define_header_rule;
use crate::scanner::passive::models::{ScanConfidence, ScanSeverity};

define_header_rule!(
    MissingCsp,
    "PASSIVE-SEC-001",
    "Missing Content-Security-Policy Header",
    "Content-Security-Policy",
    "Content Security Policy (CSP) is not configured.",
    "OWASP WSTG-CONF-04\nMDN: https://developer.mozilla.org/en-US/docs/Web/HTTP/CSP\nCWE-693: Protection Mechanism Failure",
    ScanSeverity::Medium,
    ScanConfidence::Confirmed
);
