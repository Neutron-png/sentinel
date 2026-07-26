use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportData {
    pub assessment: AssessmentInfo,
    pub executive_summary: ExecutiveSummary,
    pub workflow: Vec<SectionReport>,
    pub findings: Vec<FindingReport>,
    pub evidence: EvidenceSummary,
    pub metadata: ReportMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssessmentInfo {
    pub name: String,
    pub target: String,
    pub environment: String,
    pub scope: String,
    pub methodology: String,
    pub status: String,
    pub created_at: String,
    pub completed_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutiveSummary {
    pub total_sections: usize,
    pub total_activities: usize,
    pub total_tasks: usize,
    pub completed_tasks: usize,
    pub skipped_tasks: usize,
    pub in_progress_tasks: usize,
    pub total_evidence: usize,
    pub total_findings: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionReport {
    pub title: String,
    pub activities: Vec<ActivityReport>,
    pub completed: usize,
    pub total: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityReport {
    pub title: String,
    pub tasks: Vec<TaskReport>,
    pub completed: usize,
    pub total: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskReport {
    pub title: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindingReport {
    pub title: String,
    pub severity: String,
    pub confidence: String,
    pub status: String,
    pub description: String,
    pub impact: String,
    pub recommendation: String,
    pub references: String,
    pub linked_evidence: Vec<LinkedEvidenceItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkedEvidenceItem {
    pub evidence_type: String,
    pub title: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceSummary {
    pub total: usize,
    pub by_type: Vec<EvidenceTypeCount>,
    pub by_task: Vec<TaskEvidenceGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceTypeCount {
    pub evidence_type: String,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskEvidenceGroup {
    pub task_title: String,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportMetadata {
    pub generated_at: String,
    pub version: String,
}

#[allow(dead_code)]
pub trait ReportExporter {
    fn name(&self) -> &'static str;
    fn extension(&self) -> &'static str;
    fn export(&self, data: &ReportData) -> anyhow::Result<String>;
}
