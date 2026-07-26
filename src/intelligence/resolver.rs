#![allow(dead_code)]

use crate::intelligence::models::ReferenceMap;

pub struct ReferenceResolver;

impl ReferenceResolver {
    pub fn cwe_to_owasp<'a>(refs: &'a ReferenceMap, cwe: &str) -> Option<&'a String> {
        refs.cwe_to_owasp
            .iter()
            .find(|(c, _)| c == cwe)
            .map(|(_, o)| o)
    }
    pub fn cwe_to_wstg<'a>(refs: &'a ReferenceMap, cwe: &str) -> Option<&'a String> {
        refs.cwe_to_wstg
            .iter()
            .find(|(c, _)| c == cwe)
            .map(|(_, w)| w)
    }
    pub fn cwe_to_capec<'a>(refs: &'a ReferenceMap, cwe: &str) -> Option<&'a Vec<String>> {
        refs.cwe_to_capec
            .iter()
            .find(|(c, _)| c == cwe)
            .map(|(_, cap)| cap)
    }
    pub fn resolve_all(refs: &ReferenceMap, cwe: &str) -> ResolvedRefs {
        ResolvedRefs {
            cwe: cwe.to_string(),
            owasp: Self::cwe_to_owasp(refs, cwe).cloned(),
            wstg: Self::cwe_to_wstg(refs, cwe).cloned(),
            capec: Self::cwe_to_capec(refs, cwe)
                .cloned()
                .unwrap_or_default()
                .clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResolvedRefs {
    pub cwe: String,
    pub owasp: Option<String>,
    pub wstg: Option<String>,
    pub capec: Vec<String>,
}
