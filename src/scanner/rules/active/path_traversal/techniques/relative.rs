use crate::scanner::rules::active::path_traversal::techniques::{PathTraversalFinding, PathTraversalTechnique};
use crate::scanner::rules::active::path_traversal::providers::linux::LinuxProvider;
use crate::scanner::rules::active::path_traversal::providers::OsProvider;
use crate::scanner::rules::active::path_traversal::providers::windows::WindowsProvider;
use crate::scanner::rules::active::path_traversal::providers::generic::GenericProvider;

use crate::network::models::{HttpRequest, HttpResponse};
use crate::scanner::analyzer::engine::ResponseAnalyzer;
use crate::scanner::payload::engine::PayloadEngine;
use crate::scanner::payload::models::InsertionPoint;

fn analyze_common(analyzer: &ResponseAnalyzer, request: &HttpRequest, response: &HttpResponse, payload: &str, technique: &str) -> Option<PathTraversalFinding> {
    let providers: Vec<Box<dyn OsProvider>> = vec![Box::new(LinuxProvider), Box::new(WindowsProvider), Box::new(GenericProvider)];
    let body = response.body.as_text().unwrap_or("");
    for provider in &providers {
        for sig in provider.file_signatures() {
            if body.contains(sig) {
                return Some(PathTraversalFinding {
                    technique: technique.to_string(),
                    injection_point: request.url.clone(),
                    payload: payload.to_string(),
                    confidence: crate::scanner::sdk::result::RuleConfidence::High,
                    evidence: format!("Path traversal detected ({}).\nURL: {}\nPayload: {}\nSignature: {}\nOS: {}",
                        technique, request.url, payload, sig, provider.name()),
                });
            }
        }
    }
    let refl = analyzer.detect_reflection(response, payload);
    if refl.found {
        return Some(PathTraversalFinding {
            technique: technique.to_string(),
            injection_point: request.url.clone(),
            payload: payload.to_string(),
            confidence: crate::scanner::sdk::result::RuleConfidence::Medium,
            evidence: format!("Potential path traversal ({}) - payload reflected.\nURL: {}\nPayload: {}",
                technique, request.url, payload),
        });
    }
    None
}

fn generate_traversal_requests(engine: &PayloadEngine, request: &HttpRequest, points: &[InsertionPoint]) -> Vec<(String, HttpRequest)> {
    let payloads = engine.generate("Path Traversal");
    let mut results = Vec::new();
    for p in &payloads {
        for point in points {
            results.push((p.value.clone(), engine.insert_into(request, point, &p.value)));
        }
    }
    results
}

macro_rules! traversal_technique {
    ($name:ident, $display:literal) => {
        pub struct $name;
        impl PathTraversalTechnique for $name {
            fn name(&self) -> &'static str { $display }
            fn description(&self) -> &'static str { concat!("Detects path traversal using ", $display, " techniques") }
            fn generate_requests(&self, engine: &PayloadEngine, request: &HttpRequest, points: &[InsertionPoint]) -> Vec<(String, HttpRequest)> { generate_traversal_requests(engine, request, points) }
            fn analyze(&self, analyzer: &ResponseAnalyzer, request: &HttpRequest, response: &HttpResponse, payload: &str) -> Option<PathTraversalFinding> { analyze_common(analyzer, request, response, payload, self.name()) }
        }
    };
}

traversal_technique!(Relative, "Relative Path Traversal");
traversal_technique!(Absolute, "Absolute Path Traversal");
traversal_technique!(Encoded, "URL-Encoded Traversal");
traversal_technique!(DoubleEncoded, "Double-Encoded Traversal");
traversal_technique!(MixedEncoded, "Mixed Encoding Traversal");
