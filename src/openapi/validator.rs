#![allow(dead_code)]

use crate::openapi::errors::OpenApiError;
use crate::openapi::models::OpenApiSpec;

pub fn validate(spec: &OpenApiSpec) -> Result<(), OpenApiError> {
    if spec.openapi.is_none() && spec.swagger.is_none() {
        return Err(OpenApiError::Validate(
            "Missing openapi or swagger version field".into(),
        ));
    }
    if spec.info.is_none() {
        return Err(OpenApiError::Validate("Missing info section".into()));
    }
    if spec.paths.is_none() {
        return Err(OpenApiError::Validate("Missing paths section".into()));
    }
    Ok(())
}
