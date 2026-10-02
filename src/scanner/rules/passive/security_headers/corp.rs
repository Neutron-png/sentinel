use crate::define_header_rule;
use crate::scanner::passive::models::{ScanConfidence, ScanSeverity};

define_header_rule!(
    MissingCorp,
    "PASSIVE-SEC-009",
    "Missing Cross-Origin-Resource-Policy Header",
    "Cross-Origin-Resource-Policy",
    "The Cross-Origin-Resource-Policy (CORP) header is not set.",
    "OWASP WSTG-CLIENT-05\nMDN: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cross-Origin-Resource-Policy\nCWE-942: Permissive Cross-domain Policy",
    ScanSeverity::Informational,
    ScanConfidence::Medium
);
