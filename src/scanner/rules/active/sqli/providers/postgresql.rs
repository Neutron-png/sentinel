use crate::scanner::rules::active::sqli::providers::DbProvider;

pub struct PostgresProvider;

impl DbProvider for PostgresProvider {
    fn name(&self) -> &'static str { "PostgreSQL" }
    fn error_payloads(&self) -> Vec<&'static str> { vec!["'", "\"", "'; SELECT 1--", "' OR '1'='1"] }
    fn boolean_payloads(&self) -> Vec<(&'static str, &'static str)> { vec![("true", " AND 1=1"), ("false", " AND 1=2")] }
    fn time_payloads(&self) -> Vec<&'static str> { vec!["'; SELECT PG_SLEEP(5)--"] }
    fn union_payloads(&self) -> Vec<&'static str> { vec![" UNION SELECT NULL", " UNION SELECT NULL,NULL"] }
}
