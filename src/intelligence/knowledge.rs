#![allow(dead_code)]

use std::collections::HashMap;
use uuid::Uuid;

use crate::intelligence::models::{KnowledgeEntry, ReferenceMap, RemediationInfo};

pub struct KnowledgeRepository {
    pub entries: HashMap<String, KnowledgeEntry>,
    pub references: ReferenceMap,
}

impl KnowledgeRepository {
    pub fn new() -> Self {
        let mut repo = Self {
            entries: HashMap::new(),
            references: ReferenceMap {
                cwe_to_owasp: vec![],
                cwe_to_wstg: vec![],
                cwe_to_capec: vec![],
            },
        };
        repo.seed_defaults();
        repo
    }

    fn seed_defaults(&mut self) {
        // CWE-79: XSS
        self.add(KnowledgeEntry {
            id: Uuid::new_v4(), cwe_id: "CWE-79".into(), cwe_name: "Cross-Site Scripting".into(),
            owasp_category: "A03:2021 Injection".into(), wstg_reference: "WSTG-INPV-01".into(),
            capec_id: Some("CAPEC-63".into()),
            description: "Cross-Site Scripting (XSS) allows attackers to inject client-side scripts into web pages.".into(),
            technical_details: "XSS occurs when user input is reflected without sanitization in HTML context, attribute context, JavaScript context, or URL context.".into(),
            impact: "Session hijacking, credential theft, defacement, malware distribution.".into(),
            attack_scenario: "An attacker crafts a URL with malicious JavaScript in a parameter. When a victim visits this URL, the script executes in their browser.".into(),
            remediation: RemediationInfo {
                summary: "Implement output encoding for all user-controlled data.".into(),
                detailed_fix: "Use context-aware output encoding: HTML entity encoding for body, attribute encoding, JavaScript escaping, and URL encoding.".into(),
                secure_example: "Use libraries like OWASP Java Encoder or equivalent for the target framework.".into(),
                references: vec!["OWASP XSS Prevention Cheat Sheet".into()],
            },
            verification_steps: vec!["Inject <script>alert(1)</script> in every parameter".into(), "Check for reflection in response".into()],
            references: vec!["OWASP Testing Guide WSTG-INPV-01".into(), "CWE-79".into()],
        });

        // CWE-89: SQLi
        self.add(KnowledgeEntry {
            id: Uuid::new_v4(), cwe_id: "CWE-89".into(), cwe_name: "SQL Injection".into(),
            owasp_category: "A03:2021 Injection".into(), wstg_reference: "WSTG-INPV-05".into(),
            capec_id: Some("CAPEC-66".into()),
            description: "SQL Injection allows attackers to interfere with database queries.".into(),
            technical_details: "Unsanitized user input is concatenated into SQL queries, allowing arbitrary SQL execution.".into(),
            impact: "Data breach, data manipulation, authentication bypass, command execution.".into(),
            attack_scenario: "An attacker enters ' OR 1=1-- in a login form, bypassing authentication by making the SQL always true.".into(),
            remediation: RemediationInfo { summary: "Use parameterized queries.".into(), detailed_fix: "Replace string concatenation with prepared statements and bind variables. Never trust user input in SQL.".into(), secure_example: "PreparedStatement stmt = conn.prepareStatement(\"SELECT * FROM users WHERE name = ?\"); stmt.setString(1, userName);".into(), references: vec![] },
            verification_steps: vec!["Test with single quote".into(), "Use sqlmap".into()],
            references: vec!["OWASP Testing Guide WSTG-INPV-05".into(), "CWE-89".into()],
        });

        self.references
            .cwe_to_owasp
            .push(("CWE-79".into(), "A03:2021".into()));
        self.references
            .cwe_to_owasp
            .push(("CWE-89".into(), "A03:2021".into()));
        self.references
            .cwe_to_wstg
            .push(("CWE-79".into(), "WSTG-INPV-01".into()));
        self.references
            .cwe_to_wstg
            .push(("CWE-89".into(), "WSTG-INPV-05".into()));
    }

    pub fn add(&mut self, entry: KnowledgeEntry) {
        self.entries.insert(entry.cwe_id.clone(), entry);
    }
    pub fn get(&self, cwe: &str) -> Option<&KnowledgeEntry> {
        self.entries.get(cwe)
    }
    pub fn find_by_owasp(&self, category: &str) -> Vec<&KnowledgeEntry> {
        self.entries
            .values()
            .filter(|e| e.owasp_category == category)
            .collect()
    }
    pub fn find_by_wstg(&self, ref_id: &str) -> Vec<&KnowledgeEntry> {
        self.entries
            .values()
            .filter(|e| e.wstg_reference == ref_id)
            .collect()
    }
    pub fn all(&self) -> Vec<&KnowledgeEntry> {
        self.entries.values().collect()
    }
    pub fn cwe_count(&self) -> usize {
        self.entries.len()
    }
}
