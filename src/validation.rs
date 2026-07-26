#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    pub field: &'static str,
    pub message: String,
}

pub fn validate_assessment_form(name: &str, target: &str) -> Vec<ValidationError> {
    let mut errors = Vec::new();

    if name.trim().is_empty() {
        errors.push(ValidationError {
            field: "name",
            message: "Assessment name is required.".into(),
        });
    }

    if target.trim().is_empty() {
        errors.push(ValidationError {
            field: "target",
            message: "Target is required.".into(),
        });
    }

    errors
}
