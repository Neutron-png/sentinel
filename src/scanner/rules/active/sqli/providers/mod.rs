pub mod mssql;
pub mod mysql;
pub mod oracle;
pub mod postgresql;
pub mod sqlite;

pub trait DbProvider: Send + Sync {
    fn name(&self) -> &'static str;
    fn error_payloads(&self) -> Vec<&'static str>;
    fn boolean_payloads(&self) -> Vec<(&'static str, &'static str)>;
    fn time_payloads(&self) -> Vec<&'static str>;
    fn union_payloads(&self) -> Vec<&'static str>;
}
