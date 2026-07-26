use crate::scanner::rules::active::path_traversal::providers::OsProvider;

pub struct WindowsProvider;
impl OsProvider for WindowsProvider {
    fn name(&self) -> &'static str { "Windows" }
    fn file_signatures(&self) -> Vec<&'static str> { vec!["[fonts]", "[extensions]", "for 16-bit app support", "mci extensions"] }
    fn traversal_payloads(&self) -> Vec<&'static str> {
        vec!["..\\..\\..\\windows\\win.ini", "..\\..\\..\\windows\\system32\\drivers\\etc\\hosts", "....\\\\....\\\\....\\\\windows\\\\win.ini"]
    }
}
