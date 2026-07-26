use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::engine::GrpcEngine;
use super::models::GrpcMessage;

pub struct GrpcIntegration {
    engine: GrpcEngine,
}

impl GrpcIntegration {
    pub fn new() -> Self {
        GrpcIntegration {
            engine: GrpcEngine::new(),
        }
    }

    pub fn engine(&self) -> &GrpcEngine {
        &self.engine
    }

    pub fn engine_mut(&mut self) -> &mut GrpcEngine {
        &mut self.engine
    }

    #[allow(clippy::too_many_arguments)]
    pub fn scan(
        &mut self,
        method: &str,
        url_str: &str,
        req_headers: &[(String, String)],
        resp_headers: &[(String, String)],
        status_code: u16,
        req_body: Option<&[u8]>,
        resp_body: Option<&[u8]>,
    ) -> Option<GrpcScanResult> {
        let id = self.engine.detect(
            method,
            url_str,
            req_headers,
            resp_headers,
            status_code,
            req_body,
            resp_body,
        )?;

        let msg = self.engine.get_message(id)?;
        let findings = self.engine.analyze_message(id);

        Some(GrpcScanResult {
            message_id: id,
            service: msg.service.clone(),
            method: msg.method.clone(),
            fields_count: msg.fields.len(),
            response_fields_count: msg.response_fields.len(),
            request_size: msg.request_frames.iter().map(|f| f.data.len()).sum(),
            response_size: msg.response_frames.iter().map(|f| f.data.len()).sum(),
            grpc_status: msg.grpc_status,
            findings: findings
                .into_iter()
                .map(|f| ScannerGrpcFinding {
                    title: f.title,
                    description: f.description,
                    severity: f.severity,
                    confidence: f.confidence,
                    recommendation: f.recommendation,
                    cwe: f.cwe,
                    owasp_category: f.owasp_category,
                    service: f.service,
                    method: f.method,
                    evidence: f.evidence,
                })
                .collect(),
        })
    }

    pub fn get_all_findings(&self) -> Vec<ScannerGrpcFinding> {
        self.engine
            .analysis_findings
            .iter()
            .flat_map(|(_, findings)| {
                findings.iter().map(|f| ScannerGrpcFinding {
                    title: f.title.clone(),
                    description: f.description.clone(),
                    severity: f.severity.clone(),
                    confidence: f.confidence.clone(),
                    recommendation: f.recommendation.clone(),
                    cwe: f.cwe.clone(),
                    owasp_category: f.owasp_category.clone(),
                    service: f.service.clone(),
                    method: f.method.clone(),
                    evidence: f.evidence.clone(),
                })
            })
            .collect()
    }
}

impl Default for GrpcIntegration {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrpcScanResult {
    pub message_id: Uuid,
    pub service: Option<String>,
    pub method: Option<String>,
    pub fields_count: usize,
    pub response_fields_count: usize,
    pub request_size: usize,
    pub response_size: usize,
    pub grpc_status: Option<i32>,
    pub findings: Vec<ScannerGrpcFinding>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannerGrpcFinding {
    pub title: String,
    pub description: String,
    pub severity: String,
    pub confidence: String,
    pub recommendation: String,
    pub cwe: Option<String>,
    pub owasp_category: Option<String>,
    pub service: Option<String>,
    pub method: Option<String>,
    pub evidence: String,
}
