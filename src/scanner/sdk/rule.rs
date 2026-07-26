#![allow(dead_code)]

use crate::network::models::HttpResponse;
use crate::scanner::sdk::context::ScanRuleContext;
use crate::scanner::sdk::metadata::RuleMetadata;
use crate::scanner::sdk::result::RuleResult;

pub trait ScanRule: Send + Sync {
    fn metadata(&self) -> &RuleMetadata;

    fn initialize(&mut self) -> Result<(), super::errors::SdkError> {
        Ok(())
    }

    fn should_run(&self, _context: &ScanRuleContext) -> bool {
        true
    }

    fn build_requests(
        &self,
        _context: &ScanRuleContext,
    ) -> Vec<crate::network::models::HttpRequest> {
        vec![]
    }

    fn execute(
        &self,
        _context: &ScanRuleContext,
        _request: &crate::network::models::HttpRequest,
        _response: &HttpResponse,
    ) -> Vec<RuleResult> {
        vec![]
    }

    fn analyze(&self, results: &[RuleResult]) -> Vec<RuleResult> {
        results.to_vec()
    }

    fn cleanup(&mut self) -> Result<(), super::errors::SdkError> {
        Ok(())
    }
}
