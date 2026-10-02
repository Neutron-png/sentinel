#![allow(dead_code)]

/// Protocol-specific finding metadata
#[derive(Debug, Clone)]
pub struct ProtocolFindingDetail {
    pub protocol_version: String,
    pub negotiated_alpn: Option<String>,
    pub connection_id: Option<String>,
    pub stream_count: Option<usize>,
    pub frame_count: Option<usize>,
    pub quic_connection_id: Option<String>,
    pub zero_rtt_used: Option<bool>,
}

impl ProtocolFindingDetail {
    pub fn format_for_report(&self) -> String {
        let mut lines = vec![format!("Protocol Version: {}", self.protocol_version)];
        if let Some(ref alpn) = self.negotiated_alpn {
            lines.push(format!("ALPN Negotiated: {alpn}"));
        }
        if let Some(ref cid) = self.connection_id {
            lines.push(format!("Connection ID: {cid}"));
        }
        if let Some(streams) = self.stream_count {
            lines.push(format!("Streams: {streams}"));
        }
        if let Some(frames) = self.frame_count {
            lines.push(format!("Frames: {frames}"));
        }
        lines.join("\n")
    }
}

/// Protocol information included in report data
#[derive(Debug, Clone)]
#[derive(Default)]
pub struct ProtocolReportInfo {
    pub versions_detected: Vec<String>,
    pub alpn_negotiations: Vec<String>,
    pub total_connections: usize,
    pub total_streams: u64,
    pub total_frames: u64,
    pub quic_connections: usize,
}

