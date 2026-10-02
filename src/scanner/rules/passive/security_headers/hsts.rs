use crate::define_header_rule;
use crate::scanner::passive::models::{ScanConfidence, ScanSeverity};

define_header_rule!(
    MissingHsts,
    "PASSIVE-SEC-002",
    "Missing Strict-Transport-Security Header",
    "Strict-Transport-Security",
    "HTTP Strict Transport Security (HSTS) is not configured.",
    "OWASP WSTG-CONF-01\nMDN: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Strict-Transport-Security\nRFC 6797: HTTP Strict Transport Security\nCWE-319: Cleartext Transmission of Sensitive Information",
    ScanSeverity::Medium,
    ScanConfidence::Medium
);
