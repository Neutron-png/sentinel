use uuid::Uuid;

use super::analysis;
use super::detector;
use super::messages;
use super::models::{GrpcAnalysisFinding, GrpcMessage, GrpcService};
use super::parser;
use super::proto::ProtoLoader;

pub struct GrpcEngine {
    pub detected_messages: Vec<GrpcMessage>,
    pub discovered_services: Vec<GrpcService>,
    pub proto_loader: ProtoLoader,
    pub analysis_findings: Vec<(Uuid, Vec<GrpcAnalysisFinding>)>,
}

impl GrpcEngine {
    pub fn new() -> Self {
        GrpcEngine {
            detected_messages: Vec::new(),
            discovered_services: Vec::new(),
            proto_loader: ProtoLoader::new(),
            analysis_findings: Vec::new(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn detect(
        &mut self,
        method: &str,
        url_str: &str,
        req_headers: &[(String, String)],
        resp_headers: &[(String, String)],
        status_code: u16,
        req_body: Option<&[u8]>,
        resp_body: Option<&[u8]>,
    ) -> Option<Uuid> {
        let mut msg =
            detector::detect_grpc_call(method, url_str, req_headers, resp_headers, status_code)?;

        let (service, method_name) = detector::extract_service_method(url_str, req_headers);
        msg.service = service;
        msg.method = method_name;

        if let Some(body) = req_body {
            if let Ok(frames) = parser::parse_grpc_frames(body) {
                let combined = messages::combined_frame_data(&frames);
                msg.fields = messages::inspect_message_fields(&combined);
                msg.request_frames = frames;
            }
        }

        if let Some(body) = resp_body {
            if let Ok(frames) = parser::parse_grpc_frames(body) {
                let combined = messages::combined_frame_data(&frames);
                msg.response_fields = messages::inspect_message_fields(&combined);
                msg.response_frames = frames;
            }
        }

        for (name, value) in resp_headers {
            let lower = name.to_lowercase();
            if lower == "grpc-status" {
                msg.grpc_status = value.parse().ok();
            } else if lower == "grpc-message" {
                msg.grpc_message = Some(value.clone());
            }
        }

        let id = msg.id;
        self.detected_messages.push(msg);
        Some(id)
    }

    pub fn load_proto(&mut self, filename: &str, content: &str) -> Result<usize, String> {
        let schema = self.proto_loader.load_proto_text(filename, content)?;
        let service_count = schema.services.len();
        self.discovered_services.extend(schema.services.clone());
        Ok(service_count)
    }

    pub fn analyze_message(&self, id: Uuid) -> Vec<GrpcAnalysisFinding> {
        self.detected_messages
            .iter()
            .find(|m| m.id == id)
            .map(analysis::analyze_grpc_message)
            .unwrap_or_default()
    }

    pub fn analyze_all(&mut self) {
        self.analysis_findings.clear();
        for msg in &self.detected_messages {
            let findings = analysis::analyze_grpc_message(msg);
            if !findings.is_empty() {
                self.analysis_findings.push((msg.id, findings));
            }
        }
    }

    pub fn get_message(&self, id: Uuid) -> Option<&GrpcMessage> {
        self.detected_messages.iter().find(|m| m.id == id)
    }

    pub fn parse_frames(
        &self,
        data: &[u8],
    ) -> Result<Vec<super::models::GrpcFrame>, super::errors::GrpcError> {
        parser::parse_grpc_frames(data)
    }

    pub fn parse_fields(&self, data: &[u8]) -> Vec<super::models::ProtobufField> {
        parser::parse_protobuf_wire(data)
    }

    pub fn messages(&self) -> &[GrpcMessage] {
        &self.detected_messages
    }

    pub fn services(&self) -> &[GrpcService] {
        &self.discovered_services
    }

    pub fn clear(&mut self) {
        self.detected_messages.clear();
        self.discovered_services.clear();
        self.proto_loader.clear();
        self.analysis_findings.clear();
    }
}

impl Default for GrpcEngine {
    fn default() -> Self {
        Self::new()
    }
}
