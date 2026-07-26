pub mod generic;
pub mod linux;
pub mod windows;

pub trait OsProvider: Send + Sync {
    fn name(&self) -> &'static str;
    fn file_signatures(&self) -> Vec<&'static str>;
    fn traversal_payloads(&self) -> Vec<&'static str>;
}
