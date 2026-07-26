#![allow(dead_code)]

use crate::network::models::{HttpRequest, HttpResponse};
use crate::scanner::passive::models::ScanResult;

pub trait ScanRule: Send + Sync {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn category(&self) -> &'static str;
    fn severity(&self) -> super::models::ScanSeverity;
    fn confidence(&self) -> super::models::ScanConfidence;
    fn references(&self) -> &'static str {
        ""
    }
    fn applies_to(&self, request: &HttpRequest, response: &HttpResponse) -> bool;
    fn analyze(
        &self,
        transaction_id: uuid::Uuid,
        request: &HttpRequest,
        response: &HttpResponse,
    ) -> Vec<ScanResult>;
}
