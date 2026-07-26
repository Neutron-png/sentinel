#![allow(dead_code)]

pub mod html;
pub mod json;
pub mod markdown;
pub mod pdf;

use crate::reporting::models::{OutputFormat, ReportData};

pub trait ReportExporter {
    fn format(&self) -> OutputFormat;
    fn file_extension(&self) -> &'static str;
    fn export(&self, data: &ReportData) -> String;
}
