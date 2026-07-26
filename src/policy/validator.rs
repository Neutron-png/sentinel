#![allow(dead_code)]

use crate::policy::errors::PolicyError;
use crate::policy::models::ScanPolicy;

pub fn validate(policy: &ScanPolicy) -> Result<(), PolicyError> {
    if policy.name.is_empty() {
        return Err(PolicyError::Validate("Name is required".into()));
    }
    if policy.limits.max_requests == 0 {
        return Err(PolicyError::Validate("max_requests must be > 0".into()));
    }
    if policy.limits.concurrent_requests == 0 {
        return Err(PolicyError::Validate(
            "concurrent_requests must be > 0".into(),
        ));
    }
    Ok(())
}
