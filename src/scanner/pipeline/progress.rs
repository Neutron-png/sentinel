#![allow(dead_code)]

use crate::scanner::pipeline::stage::PipelineStage;

#[derive(Debug, Clone)]
pub struct ProgressTracker {
    pub current_rule: String,
    pub current_target: String,
    pub current_stage: PipelineStage,
    pub requests_sent: usize,
    pub responses_received: usize,
    pub rules_completed: usize,
    pub rules_failed: usize,
    pub total_rules: usize,
    pub total_targets: usize,
    pub targets_completed: usize,
    pub error_count: usize,
}

impl ProgressTracker {
    pub fn new(total_rules: usize) -> Self {
        Self {
            current_rule: String::new(),
            current_target: String::new(),
            current_stage: PipelineStage::Prepare,
            requests_sent: 0,
            responses_received: 0,
            rules_completed: 0,
            rules_failed: 0,
            total_rules,
            total_targets: 0,
            targets_completed: 0,
            error_count: 0,
        }
    }

    pub fn progress_pct(&self) -> f64 {
        if self.total_rules == 0 {
            return 0.0;
        }
        let stage_progress = match self.current_stage {
            PipelineStage::Prepare => 0.05,
            PipelineStage::Validate => 0.10,
            PipelineStage::SelectTargets => 0.15,
            PipelineStage::GenerateRequests => 0.20,
            PipelineStage::ExecuteRequests => 0.40,
            PipelineStage::CollectResponses => 0.55,
            PipelineStage::AnalyzeResponses => 0.70,
            PipelineStage::ProduceResults => 0.85,
            PipelineStage::Cleanup => 0.95,
            PipelineStage::Complete => 1.0,
            PipelineStage::Failed => 1.0,
        };
        let rule_progress =
            (self.rules_completed + self.rules_failed) as f64 / self.total_rules as f64;
        (rule_progress + stage_progress / self.total_rules as f64).min(1.0)
    }

    pub fn complete_rule(&mut self) {
        self.rules_completed += 1;
    }
    pub fn fail_rule(&mut self) {
        self.rules_failed += 1;
    }
    pub fn request_sent(&mut self) {
        self.requests_sent += 1;
    }
    pub fn response_received(&mut self) {
        self.responses_received += 1;
    }
}
