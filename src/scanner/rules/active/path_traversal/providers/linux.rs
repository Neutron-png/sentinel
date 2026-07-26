use crate::scanner::rules::active::path_traversal::providers::OsProvider;

pub struct LinuxProvider;
impl OsProvider for LinuxProvider {
    fn name(&self) -> &'static str { "Linux" }
    fn file_signatures(&self) -> Vec<&'static str> { vec!["root:", "daemon:", "/bin/bash", "/usr/sbin", "nobody:", "www-data:"] }
    fn traversal_payloads(&self) -> Vec<&'static str> {
        vec!["../../../etc/passwd", "....//....//....//etc/passwd", "/etc/passwd", "..%2f..%2f..%2fetc%2fpasswd"]
    }
}
