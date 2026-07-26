use crate::define_header_rule;
use crate::scanner::passive::models::{ScanConfidence, ScanSeverity};

define_header_rule!(
    MissingXcto,
    "PASSIVE-SEC-004",
    "Missing X-Content-Type-Options Header",
    "X-Content-Type-Options",
    "The X-Content-Type-Options header is not set.",
    "OWASP WSTG-CONF-04\nMDN: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/X-Content-Type-Options\nCWE-434: Unrestricted Upload of File with Dangerous Type",
    ScanSeverity::Low,
    ScanConfidence::Confirmed
);
