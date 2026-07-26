#![allow(dead_code)]

use crate::reporting::builder::ReportBuilder;
use crate::reporting::errors::ReportError;
use crate::reporting::events::{ReportEvent, ReportEventBus};
use crate::reporting::export::html::HtmlExporter;
use crate::reporting::export::json::JsonExporter;
use crate::reporting::export::markdown::MarkdownExporter;
use crate::reporting::export::pdf::PdfExporter;
use crate::reporting::export::ReportExporter;
use crate::reporting::models::{FindingSummary, OutputFormat, ReportConfig, ReportData};

pub struct ReportEngine {
    event_bus: ReportEventBus,
}

impl ReportEngine {
    pub fn new() -> Self {
        Self {
            event_bus: ReportEventBus::new(128),
        }
    }

    pub fn event_bus(&self) -> ReportEventBus {
        self.event_bus.clone()
    }

    pub fn generate(
        &self,
        config: &ReportConfig,
        findings: &[FindingSummary],
    ) -> Result<String, ReportError> {
        self.event_bus.emit(ReportEvent::Started {
            assessment_id: config.assessment_id,
            report_type: format!("{:?}", config.report_type),
        });

        let data = ReportBuilder::build("Assessment", "target.example.com", findings);

        let result = match config.output_format {
            OutputFormat::Html => HtmlExporter.export(&data),
            OutputFormat::Markdown => MarkdownExporter.export(&data),
            OutputFormat::Pdf => PdfExporter.export(&data),
            OutputFormat::Json => JsonExporter.export(&data),
        };

        self.event_bus.emit(ReportEvent::Generated {
            assessment_id: config.assessment_id,
        });
        self.event_bus.emit(ReportEvent::Exported {
            assessment_id: config.assessment_id,
            format: format!("{:?}", config.output_format),
        });

        Ok(result)
    }

    pub fn export_to_file(
        &self,
        data: &ReportData,
        path: &str,
        format: OutputFormat,
    ) -> Result<(), ReportError> {
        let content = match format {
            OutputFormat::Html => HtmlExporter.export(data),
            OutputFormat::Markdown => MarkdownExporter.export(data),
            OutputFormat::Pdf => PdfExporter.export(data),
            OutputFormat::Json => JsonExporter.export(data),
        };
        std::fs::write(path, content).map_err(|e| ReportError::Export(e.to_string()))
    }
}
