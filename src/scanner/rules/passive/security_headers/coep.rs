use crate::define_header_rule;
use crate::scanner::passive::models::{ScanConfidence, ScanSeverity};

define_header_rule!(
    MissingCoep,
    "PASSIVE-SEC-008",
    "Missing Cross-Origin-Embedder-Policy Header",
    "Cross-Origin-Embedder-Policy",
    "The Cross-Origin-Embedder-Policy (COEP) header is not set.",
    "OWASP WSTG-CLIENT-05\nMDN: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cross-Origin-Embedder-Policy\nCWE-942: Permissive Cross-domain Policy",
    ScanSeverity::Informational,
    ScanConfidence::Medium
);
