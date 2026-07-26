use std::collections::HashMap;

use chrono::Utc;

use crate::db::repository::Repository;
use crate::models::assessment::Assessment;
use crate::models::evidence::Evidence;
use crate::models::finding::Finding;
use crate::models::workflow::{SectionNode, TaskStatus};
use crate::reports::report::{
    ActivityReport, AssessmentInfo, EvidenceSummary, EvidenceTypeCount, ExecutiveSummary,
    FindingReport, LinkedEvidenceItem, ReportData, ReportMetadata, SectionReport,
    TaskEvidenceGroup, TaskReport,
};

pub fn build_report_data(
    repo: &Repository,
    assessment: &Assessment,
    sections: &[SectionNode],
    _evidence_counts: &HashMap<uuid::Uuid, usize>,
) -> anyhow::Result<ReportData> {
    let mut all_findings: Vec<Finding> = Vec::new();
    let mut all_evidence: Vec<Evidence> = Vec::new();
    let mut evidence_by_task = Vec::new();

    for section in sections {
        for activity in &section.activities {
            for task in &activity.tasks {
                let f = repo.load_findings(&task.id).unwrap_or_default();
                all_findings.extend(f);

                let e = repo.load_evidence(&task.id).unwrap_or_default();
                evidence_by_task.push(TaskEvidenceGroup {
                    task_title: task.title.clone(),
                    count: e.len(),
                });
                all_evidence.extend(e);
            }
        }
    }

    all_findings.sort_by(|a, b| {
        severity_order(&a.severity.to_string()).cmp(&severity_order(&b.severity.to_string()))
    });

    // Build linked evidence for findings
    let finding_reports: Vec<FindingReport> = all_findings
        .iter()
        .map(|f| {
            let linked_ids = repo.load_linked_evidence_ids(&f.id).unwrap_or_default();
            let linked: Vec<LinkedEvidenceItem> = all_evidence
                .iter()
                .filter(|e| linked_ids.contains(&e.id))
                .map(|e| LinkedEvidenceItem {
                    evidence_type: e.evidence_type.to_string(),
                    title: e.title.clone(),
                    content: e.content.clone(),
                })
                .collect();
            FindingReport {
                title: f.title.clone(),
                severity: f.severity.to_string(),
                confidence: f.confidence.to_string(),
                status: f.status.to_string(),
                description: f.description.clone(),
                impact: f.impact.clone(),
                recommendation: f.recommendation.clone(),
                references: f.references.clone(),
                linked_evidence: linked,
            }
        })
        .collect();

    // Workflow report
    let mut workflow = Vec::new();
    let mut total_completed = 0usize;
    let mut total_skipped = 0usize;
    let mut total_in_progress = 0usize;
    let mut total_tasks = 0usize;
    let mut total_activities = 0usize;

    for section in sections {
        let mut activities = Vec::new();
        let mut section_completed = 0usize;
        let mut section_total = 0usize;

        for activity in &section.activities {
            let mut tasks = Vec::new();
            let mut act_completed = 0usize;
            let act_total = activity.tasks.len();

            for task in &activity.tasks {
                let ts = task.status;
                if matches!(ts, TaskStatus::Completed) {
                    act_completed += 1;
                }
                tasks.push(TaskReport {
                    title: task.title.clone(),
                    status: ts.to_string(),
                });
            }
            section_completed += act_completed;
            section_total += act_total;
            activities.push(ActivityReport {
                title: activity.activity.title.clone(),
                tasks,
                completed: act_completed,
                total: act_total,
            });
        }

        total_completed += section_completed;
        total_tasks += section_total;
        total_activities += section.activities.len();

        workflow.push(SectionReport {
            title: section.section.title.clone(),
            activities,
            completed: section_completed,
            total: section_total,
        });
    }

    // Count status
    for section in sections {
        for activity in &section.activities {
            for task in &activity.tasks {
                match task.status {
                    TaskStatus::Skipped => total_skipped += 1,
                    TaskStatus::InProgress => total_in_progress += 1,
                    _ => {}
                }
            }
        }
    }

    // Evidence type distribution
    let mut type_counts: HashMap<String, usize> = HashMap::new();
    for ev in &all_evidence {
        *type_counts.entry(ev.evidence_type.to_string()).or_default() += 1;
    }
    let by_type: Vec<EvidenceTypeCount> = type_counts
        .into_iter()
        .map(|(k, v)| EvidenceTypeCount {
            evidence_type: k,
            count: v,
        })
        .collect();

    Ok(ReportData {
        assessment: AssessmentInfo {
            name: assessment.name.clone(),
            target: assessment.target.clone(),
            environment: assessment.environment.to_string(),
            scope: assessment.scope.to_string(),
            methodology: assessment.methodology.to_string(),
            status: assessment.status.to_string(),
            created_at: assessment.created_at.to_rfc3339(),
            completed_at: Utc::now().to_rfc3339(),
        },
        executive_summary: ExecutiveSummary {
            total_sections: sections.len(),
            total_activities,
            total_tasks,
            completed_tasks: total_completed,
            skipped_tasks: total_skipped,
            in_progress_tasks: total_in_progress,
            total_evidence: all_evidence.len(),
            total_findings: all_findings.len(),
        },
        workflow,
        findings: finding_reports,
        evidence: EvidenceSummary {
            total: all_evidence.len(),
            by_type,
            by_task: evidence_by_task,
        },
        metadata: ReportMetadata {
            generated_at: Utc::now().to_rfc3339(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        },
    })
}

fn severity_order(s: &str) -> u8 {
    match s {
        "Critical" => 0,
        "High" => 1,
        "Medium" => 2,
        "Low" => 3,
        _ => 4,
    }
}
