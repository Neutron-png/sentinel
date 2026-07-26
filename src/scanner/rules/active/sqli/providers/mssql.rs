use crate::scanner::rules::active::sqli::providers::DbProvider;

pub struct MssqlProvider;

impl DbProvider for MssqlProvider {
    fn name(&self) -> &'static str { "Microsoft SQL Server" }
    fn error_payloads(&self) -> Vec<&'static str> { vec!["'", "; SELECT 1--", "' HAVING 1=1--"] }
    fn boolean_payloads(&self) -> Vec<(&'static str, &'static str)> { vec![("true", " AND 1=1"), ("false", " AND 1=2")] }
    fn time_payloads(&self) -> Vec<&'static str> { vec!["'; WAITFOR DELAY '00:00:05'--"] }
    fn union_payloads(&self) -> Vec<&'static str> { vec![" UNION SELECT NULL", " UNION SELECT NULL,NULL"] }
}
