#![allow(dead_code)]

use crate::intelligence::models::RemediationInfo;

pub struct RemediationGenerator;

impl RemediationGenerator {
    pub fn generate_for(
        cwe: &str,
        technology: &super::models::TechnologyStack,
    ) -> Option<RemediationInfo> {
        match cwe {
            "CWE-79" => Some(RemediationInfo {
                summary: "Output encode all user input".into(),
                detailed_fix: format!("Use context-aware encoding for {}", technology.framework.as_deref().unwrap_or("your framework")),
                secure_example: "OWASP Encoder.encodeForHTML(userInput)".into(),
                references: vec!["OWASP XSS Prevention Cheat Sheet".into()],
            }),
            "CWE-89" => Some(RemediationInfo {
                summary: "Use parameterized queries".into(),
                detailed_fix: format!("Replace string concatenation with prepared statements for {}", technology.database.as_deref().unwrap_or("your database")),
                secure_example: "PreparedStatement stmt = conn.prepareStatement(query); stmt.setString(1, input);".into(),
                references: vec!["OWASP SQL Injection Prevention".into()],
            }),
            _ => None,
        }
    }
}
