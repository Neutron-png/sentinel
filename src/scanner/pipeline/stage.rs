#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineStage {
    Prepare,
    Validate,
    SelectTargets,
    GenerateRequests,
    ExecuteRequests,
    CollectResponses,
    AnalyzeResponses,
    ProduceResults,
    Cleanup,
    Complete,
    Failed,
}

impl PipelineStage {
    pub fn next(self) -> Self {
        match self {
            Self::Prepare => Self::Validate,
            Self::Validate => Self::SelectTargets,
            Self::SelectTargets => Self::GenerateRequests,
            Self::GenerateRequests => Self::ExecuteRequests,
            Self::ExecuteRequests => Self::CollectResponses,
            Self::CollectResponses => Self::AnalyzeResponses,
            Self::AnalyzeResponses => Self::ProduceResults,
            Self::ProduceResults => Self::Cleanup,
            Self::Cleanup => Self::Complete,
            _ => Self::Complete,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Prepare => "Prepare",
            Self::Validate => "Validate",
            Self::SelectTargets => "Select Targets",
            Self::GenerateRequests => "Generate Requests",
            Self::ExecuteRequests => "Execute Requests",
            Self::CollectResponses => "Collect Responses",
            Self::AnalyzeResponses => "Analyze Responses",
            Self::ProduceResults => "Produce Results",
            Self::Cleanup => "Cleanup",
            Self::Complete => "Complete",
            Self::Failed => "Failed",
        }
    }
}
