#![allow(dead_code)]

use crate::orchestrator::models::{AssessmentPhase, ExecutionMode};

pub struct PhasePipeline;

impl PhasePipeline {
    pub fn phases_for(mode: ExecutionMode) -> Vec<AssessmentPhase> {
        match mode {
            ExecutionMode::PassiveOnly => vec![
                AssessmentPhase::Initialize,
                AssessmentPhase::LoadConfig,
                AssessmentPhase::CrawlTarget,
                AssessmentPhase::BuildSiteMap,
                AssessmentPhase::PassiveScan,
                AssessmentPhase::GenerateReport,
                AssessmentPhase::Cleanup,
            ],
            ExecutionMode::ActiveOnly => vec![
                AssessmentPhase::Initialize,
                AssessmentPhase::Authenticate,
                AssessmentPhase::PassiveScan,
                AssessmentPhase::ActiveScan,
                AssessmentPhase::VerifyFindings,
            ],
            ExecutionMode::FullAssessment => AssessmentPhase::order(),
            ExecutionMode::ResumeAssessment => vec![
                AssessmentPhase::RestoreSession,
                AssessmentPhase::Authenticate,
                AssessmentPhase::CrawlTarget,
                AssessmentPhase::BuildSiteMap,
                AssessmentPhase::PassiveScan,
                AssessmentPhase::ActiveScan,
                AssessmentPhase::GenerateReport,
                AssessmentPhase::Cleanup,
            ],
            ExecutionMode::SelectedModules => AssessmentPhase::order(),
        }
    }

    pub fn next_phase(current: AssessmentPhase, mode: ExecutionMode) -> Option<AssessmentPhase> {
        let phases = Self::phases_for(mode);
        let pos = phases.iter().position(|p| *p == current)?;
        phases.get(pos + 1).copied()
    }
}
