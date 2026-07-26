#![allow(dead_code)]

use crate::reporting::export::ReportExporter;
use crate::reporting::models::{OutputFormat, ReportData};

pub struct PdfExporter;

impl ReportExporter for PdfExporter {
    fn format(&self) -> OutputFormat {
        OutputFormat::Pdf
    }
    fn file_extension(&self) -> &'static str {
        "pdf"
    }

    fn export(&self, _data: &ReportData) -> String {
        // PDF generation requires external libraries
        String::from("PDF export not yet implemented")
    }
}
