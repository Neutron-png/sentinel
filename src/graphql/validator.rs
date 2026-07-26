#![allow(dead_code)]

pub struct GraphQlValidator;

impl GraphQlValidator {
    pub fn validate_query(_query: &str) -> Result<(), super::errors::GraphQlError> {
        Ok(())
    }

    pub fn has_balanced_braces(query: &str) -> bool {
        let mut count = 0;
        for c in query.chars() {
            match c {
                '{' => count += 1,
                '}' if count == 0 => return false,
                '}' => count -= 1,
                _ => {}
            }
        }
        count == 0
    }

    pub fn validate_variables(vars: &str) -> Result<(), super::errors::GraphQlError> {
        serde_json::from_str::<serde_json::Value>(vars)
            .map(|_| ())
            .map_err(|e| super::errors::GraphQlError::Validate(e.to_string()))
    }
}
