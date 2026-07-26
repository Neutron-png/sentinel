use crate::scanner::rules::active::path_traversal::providers::OsProvider;

pub struct GenericProvider;
impl OsProvider for GenericProvider {
    fn name(&self) -> &'static str { "Generic" }
    fn file_signatures(&self) -> Vec<&'static str> { vec!["Permission denied", "No such file", "cannot open", "failed to open"] }
    fn traversal_payloads(&self) -> Vec<&'static str> {
        vec!["../../../etc/passwd", "..\\..\\..\\windows\\win.ini", "....//....//etc/passwd"]
    }
}
