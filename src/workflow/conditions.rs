#![allow(dead_code)]

pub struct ConditionEvaluator;

impl ConditionEvaluator {
    pub fn evaluate(_condition: &str, _variables: &super::variables::VariableStore) -> bool {
        true
    }
    pub fn check_response_code(_code: u16, _expected: u16) -> bool {
        _code == _expected
    }
    pub fn check_header(_headers: &str, _name: &str, _expected: &str) -> bool {
        _headers.contains(&format!("{}: {}", _name, _expected))
    }
    pub fn check_cookie_exists(_cookies: &str, _name: &str) -> bool {
        _cookies.contains(_name)
    }
    pub fn check_variable_exists(vars: &super::variables::VariableStore, name: &str) -> bool {
        vars.get(name).is_some()
    }
}
