#![allow(dead_code)]

use crate::reporting::export::ReportExporter;
use crate::reporting::models::{OutputFormat, ReportData};
use crate::reporting::templates::TemplateEngine;

pub struct MarkdownExporter;

impl ReportExporter for MarkdownExporter {
    fn format(&self) -> OutputFormat {
        OutputFormat::Markdown
    }
    fn file_extension(&self) -> &'static str {
        "md"
    }

    fn export(&self, data: &ReportData) -> String {
        let tmpl = TemplateEngine::new();
        let mut s = tmpl.render_executive(data);
        s.push_str("\n\n");
        s.push_str(&tmpl.render_technical(data));
        s
    }
}
