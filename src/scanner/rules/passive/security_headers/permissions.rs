use crate::define_header_rule;
use crate::scanner::passive::models::{ScanConfidence, ScanSeverity};

define_header_rule!(
    MissingPermissionsPolicy,
    "PASSIVE-SEC-006",
    "Missing Permissions-Policy Header",
    "Permissions-Policy",
    "The Permissions-Policy header (formerly Feature-Policy) is not set.",
    "OWASP WSTG-CONF-04\nMDN: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Permissions-Policy\nW3C: https://www.w3.org/TR/permissions-policy-1/",
    ScanSeverity::Low,
    ScanConfidence::Confirmed
);
