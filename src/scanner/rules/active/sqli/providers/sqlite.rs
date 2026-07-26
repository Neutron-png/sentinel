use crate::scanner::rules::active::sqli::providers::DbProvider;

pub struct SqliteProvider;

impl DbProvider for SqliteProvider {
    fn name(&self) -> &'static str { "SQLite" }
    fn error_payloads(&self) -> Vec<&'static str> { vec!["'", "\"", "'; SELECT 1--"] }
    fn boolean_payloads(&self) -> Vec<(&'static str, &'static str)> { vec![("true", " AND 1=1"), ("false", " AND 1=2")] }
    fn time_payloads(&self) -> Vec<&'static str> { vec!["' AND RANDOMBLOB(100000000) IS NOT NULL--"] }
    fn union_payloads(&self) -> Vec<&'static str> { vec![" UNION SELECT NULL", " UNION SELECT NULL,NULL"] }
}
