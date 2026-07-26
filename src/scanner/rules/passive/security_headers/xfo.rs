use crate::define_header_rule;
use crate::scanner::passive::models::{ScanConfidence, ScanSeverity};

define_header_rule!(
    MissingXfo,
    "PASSIVE-SEC-003",
    "Missing X-Frame-Options Header",
    "X-Frame-Options",
    "The X-Frame-Options header is not set.",
    "OWASP WSTG-CONF-04\nMDN: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-Frame-Options\nCWE-1021: Improper Restriction of Rendered UI Layers",
    ScanSeverity::Medium,
    ScanConfidence::Confirmed
);
