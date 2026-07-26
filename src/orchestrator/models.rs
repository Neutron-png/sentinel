#![allow(dead_code)]

use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssessmentPhase {
    Initialize,
    LoadConfig,
    RestoreSession,
    Authenticate,
    CrawlTarget,
    BuildSiteMap,
    PassiveScan,
    ActiveScan,
    VerifyFindings,
    GenerateReport,
    Cleanup,
    Complete,
    Failed,
}

impl AssessmentPhase {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Initialize => "Initialize",
            Self::LoadConfig => "Load Config",
            Self::RestoreSession => "Restore Session",
            Self::Authenticate => "Authenticate",
            Self::CrawlTarget => "Crawl Target",
            Self::BuildSiteMap => "Build Site Map",
            Self::PassiveScan => "Passive Scan",
            Self::ActiveScan => "Active Scan",
            Self::VerifyFindings => "Verify Findings",
            Self::GenerateReport => "Generate Report",
            Self::Cleanup => "Cleanup",
            Self::Complete => "Complete",
            Self::Failed => "Failed",
        }
    }

    pub fn order() -> Vec<AssessmentPhase> {
        vec![
            Self::Initialize,
            Self::LoadConfig,
            Self::RestoreSession,
            Self::Authenticate,
            Self::CrawlTarget,
            Self::BuildSiteMap,
            Self::PassiveScan,
            Self::ActiveScan,
            Self::VerifyFindings,
            Self::GenerateReport,
            Self::Cleanup,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionMode {
    PassiveOnly,
    ActiveOnly,
    FullAssessment,
    SelectedModules,
    ResumeAssessment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrchestratorState {
    Idle,
    Running,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone)]
pub struct OrchestratorTask {
    pub id: Uuid,
    pub phase: AssessmentPhase,
    pub name: String,
    pub state: OrchestratorState,
    pub progress: f64,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
}
