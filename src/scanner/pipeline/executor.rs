#![allow(dead_code)]

use std::sync::Arc;
use tokio::sync::watch;
use tokio::time::sleep;

use crate::network::client::HttpClient;
use crate::network::models::{HttpRequest, HttpResponse};
use crate::scanner::pipeline::context::PipelineContext;
use crate::scanner::pipeline::errors::PipelineError;
use crate::scanner::pipeline::progress::ProgressTracker;
use crate::scanner::pipeline::retry::RetryPolicy;
use crate::scanner::pipeline::stage::PipelineStage;
use crate::scanner::pipeline::timeout::TimeoutConfig;
use crate::scanner::sdk::result::RuleResult;
use crate::scanner::sdk::rule::ScanRule;

pub struct PipelineExecutor {
    retry: RetryPolicy,
    timeout: TimeoutConfig,
}

impl PipelineExecutor {
    pub fn new() -> Self {
        Self {
            retry: RetryPolicy::default(),
            timeout: TimeoutConfig::default(),
        }
    }

    pub fn with_retry(mut self, policy: RetryPolicy) -> Self {
        self.retry = policy;
        self
    }
    pub fn with_timeout(mut self, config: TimeoutConfig) -> Self {
        self.timeout = config;
        self
    }

    pub async fn execute(
        &self,
        rule: &dyn ScanRule,
        client: Arc<HttpClient>,
        target: &str,
        cancel_tx: watch::Sender<bool>,
    ) -> (Vec<RuleResult>, ProgressTracker) {
        let metadata = rule.metadata();
        let cancel_rx = cancel_tx.subscribe();
        let mut progress = ProgressTracker::new(1);
        progress.current_rule = metadata.id.clone();
        progress.current_target = target.to_string();

        // Stage: Prepare
        progress.current_stage = PipelineStage::Prepare;
        if *cancel_rx.borrow() {
            return (vec![], progress);
        }

        // Stage: Validate
        progress.current_stage = PipelineStage::Validate;

        // Stage: Generate Requests
        progress.current_stage = PipelineStage::GenerateRequests;
        let _ctx = PipelineContext::new(client.clone(), target, &metadata.id, cancel_rx.clone());
        let requests: Vec<HttpRequest> = rule.build_requests(
            &crate::scanner::sdk::context::ScanRuleContext::new(client.clone(), target),
        );

        // Stage: Execute Requests
        progress.current_stage = PipelineStage::ExecuteRequests;
        progress.total_targets = requests.len();

        let mut responses: Vec<(HttpRequest, Option<HttpResponse>)> = Vec::new();
        for req in requests {
            if *cancel_rx.borrow() {
                break;
            }
            progress.request_sent();

            let result = self
                .execute_request_with_retry(&client, &req, &cancel_rx)
                .await;
            match result {
                Ok(resp) => {
                    progress.response_received();
                    responses.push((req, Some(resp)));
                }
                Err(_) => {
                    progress.error_count += 1;
                    responses.push((req, None));
                }
            }
            progress.targets_completed += 1;
        }

        // Stage: Collect Responses
        progress.current_stage = PipelineStage::CollectResponses;

        // Stage: Analyze Responses
        progress.current_stage = PipelineStage::AnalyzeResponses;
        let mut results = Vec::new();
        let sdk_ctx = crate::scanner::sdk::context::ScanRuleContext::new(client, target);
        for (req, resp_opt) in &responses {
            if let Some(resp) = resp_opt {
                let rule_results = rule.execute(&sdk_ctx, req, resp);
                results.extend(rule_results);
            }
        }

        // Stage: Analyze
        results = rule.analyze(&results);

        // Stage: Produce Results
        progress.current_stage = PipelineStage::ProduceResults;
        // Stage: Cleanup
        progress.current_stage = PipelineStage::Cleanup;
        // Done
        progress.current_stage = PipelineStage::Complete;
        progress.complete_rule();

        (results, progress)
    }

    async fn execute_request_with_retry(
        &self,
        client: &HttpClient,
        request: &HttpRequest,
        cancel: &watch::Receiver<bool>,
    ) -> Result<HttpResponse, PipelineError> {
        let mut attempt: u32 = 0;
        loop {
            if *cancel.borrow() {
                return Err(PipelineError::Cancelled);
            }
            match tokio::time::timeout(
                self.timeout.request_timeout,
                client.execute(request.clone()),
            )
            .await
            {
                Ok(Ok(resp)) => return Ok(resp),
                Ok(Err(e)) => {
                    let err = PipelineError::Network(e.to_string());
                    if !self.retry.should_retry(attempt, &err) {
                        return Err(err);
                    }
                }
                Err(_) => {
                    let err = PipelineError::Timeout("Request timed out".into());
                    if !self.retry.should_retry(attempt, &err) {
                        return Err(err);
                    }
                }
            }
            attempt += 1;
            sleep(self.retry.delay_for(attempt)).await;
        }
    }
}
