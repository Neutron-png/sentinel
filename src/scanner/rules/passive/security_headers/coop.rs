use crate::define_header_rule;
use crate::scanner::passive::models::{ScanConfidence, ScanSeverity};

define_header_rule!(
    MissingCoop,
    "PASSIVE-SEC-007",
    "Missing Cross-Origin-Opener-Policy Header",
    "Cross-Origin-Opener-Policy",
    "The Cross-Origin-Opener-Policy (COOP) header is not set.",
    "OWASP WSTG-CLIENT-05\nMDN: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cross-Origin-Opener-Policy\nCWE-942: Permissive Cross-domain Policy",
    ScanSeverity::Low,
    ScanConfidence::Confirmed
);
