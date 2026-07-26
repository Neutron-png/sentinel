#![allow(dead_code)]

use crate::reporting::charts;
use crate::reporting::export::ReportExporter;
use crate::reporting::models::{OutputFormat, ReportData};

pub struct HtmlExporter;

impl ReportExporter for HtmlExporter {
    fn format(&self) -> OutputFormat {
        OutputFormat::Html
    }
    fn file_extension(&self) -> &'static str {
        "html"
    }

    fn export(&self, data: &ReportData) -> String {
        let brand = crate::reporting::models::Branding::default();
        let chart = charts::severity_chart(&data.severity_counts);
        let mut s = format!("<html><head><title>{} - Report</title><style>body{{font-family:Arial;max-width:900px;margin:0 auto}}h1{{color:{}}}</style></head><body>", data.assessment_name, brand.primary_color);
        s.push_str(&format!(
            "<h1>{} Security Assessment</h1>",
            data.assessment_name
        ));
        s.push_str(&format!(
            "<p>Target: {} | Findings: {}</p>",
            data.assessment_target, data.total_findings
        ));
        s.push_str("<h2>Severity Distribution</h2><pre>");
        s.push_str(&chart);
        s.push_str("</pre><h2>Findings</h2>");
        for (i, f) in data.findings.iter().enumerate() {
            s.push_str(&format!(
                "<h3>{}: {} ({})</h3><p>{}</p>",
                i + 1,
                f.title,
                f.severity,
                f.description
            ));
        }
        s.push_str(&format!("<hr><p>{}</p></body></html>", brand.footer_text));
        s
    }
}
