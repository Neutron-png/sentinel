use std::collections::HashMap;

use uuid::Uuid;

use crate::db::repository::Repository;
use crate::methodology::{self, MethodologyDefinition};
use crate::models::workflow::{Activity, ActivityNode, Section, SectionNode, Task, TaskStatus};

pub fn load_or_seed_workflow(
    repo: &Repository,
    assessment_id: Uuid,
    methodology_name: &str,
) -> anyhow::Result<(Vec<SectionNode>, HashMap<Uuid, usize>)> {
    if repo.has_workflow(&assessment_id)? {
        let sections = load_workflow(repo, assessment_id)?;
        let counts = repo
            .load_evidence_counts(&assessment_id)
            .unwrap_or_default();
        return Ok((sections, counts));
    }

    let def = methodology::load_definition(methodology_name)
        .unwrap_or_else(|| fallback_definition(methodology_name));
    generate_from_definition(repo, assessment_id, &def)?;

    let sections = load_workflow(repo, assessment_id)?;
    let counts = repo
        .load_evidence_counts(&assessment_id)
        .unwrap_or_default();
    Ok((sections, counts))
}

fn generate_from_definition(
    repo: &Repository,
    assessment_id: Uuid,
    def: &MethodologyDefinition,
) -> anyhow::Result<()> {
    for (si, sec) in def.sections.iter().enumerate() {
        let section = Section {
            id: Uuid::new_v4(),
            assessment_id,
            title: sec.title.clone(),
            display_order: si as i32,
        };
        repo.insert_section(&section)?;

        for (ai, act) in sec.activities.iter().enumerate() {
            let activity = Activity {
                id: Uuid::new_v4(),
                section_id: section.id,
                title: act.title.clone(),
                description: act.description.clone(),
                display_order: ai as i32,
            };
            repo.insert_activity(&activity)?;

            for (ti, tsk) in act.tasks.iter().enumerate() {
                let task = Task {
                    id: Uuid::new_v4(),
                    activity_id: activity.id,
                    title: tsk.title.clone(),
                    description: tsk.description.clone(),
                    status: TaskStatus::NotStarted,
                    notes: String::new(),
                    reference: tsk.reference.clone(),
                    display_order: ti as i32,
                };
                repo.insert_task(&task)?;
            }
        }
    }

    Ok(())
}

fn fallback_definition(name: &str) -> MethodologyDefinition {
    MethodologyDefinition {
        name: name.to_string(),
        version: "1.0".into(),
        description: "Custom methodology".into(),
        sections: vec![crate::methodology::MethodologySection {
            title: "General Testing".into(),
            activities: vec![crate::methodology::MethodologyActivity {
                title: "Security Assessment".into(),
                description: "Generic security assessment activities".into(),
                tasks: vec![
                    mktsk("Reconnaissance", "Gather information about the target", ""),
                    mktsk(
                        "Vulnerability Analysis",
                        "Identify potential vulnerabilities",
                        "",
                    ),
                    mktsk("Exploitation", "Validate identified vulnerabilities", ""),
                    mktsk("Reporting", "Document findings and recommendations", ""),
                ],
            }],
        }],
    }
}

fn mktsk(title: &str, desc: &str, reference: &str) -> crate::methodology::MethodologyTask {
    crate::methodology::MethodologyTask {
        title: title.into(),
        description: desc.into(),
        reference: reference.into(),
    }
}

fn load_workflow(repo: &Repository, assessment_id: Uuid) -> anyhow::Result<Vec<SectionNode>> {
    let sections = repo.load_sections(&assessment_id)?;
    let mut nodes = Vec::new();

    for section in sections {
        let activities = repo.load_activities(&section.id)?;
        let mut activity_nodes = Vec::new();

        for activity in activities {
            let tasks = repo.load_tasks(&activity.id)?;
            activity_nodes.push(ActivityNode {
                activity,
                tasks,
                expanded: false,
            });
        }

        nodes.push(SectionNode {
            section,
            activities: activity_nodes,
            expanded: false,
        });
    }

    Ok(nodes)
}
