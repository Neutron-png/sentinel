use serde::{Deserialize, Serialize};
use uuid::Uuid;

use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    #[serde(rename = "Not Started")]
    NotStarted,
    #[serde(rename = "In Progress")]
    InProgress,
    #[serde(rename = "Completed")]
    Completed,
    #[serde(rename = "Skipped")]
    Skipped,
}

impl TaskStatus {
    pub fn label(&self) -> &'static str {
        match self {
            Self::NotStarted => "Not Started",
            Self::InProgress => "In Progress",
            Self::Completed => "Completed",
            Self::Skipped => "Skipped",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Self::NotStarted => Self::InProgress,
            Self::InProgress => Self::Completed,
            Self::Completed => Self::Skipped,
            Self::Skipped => Self::NotStarted,
        }
    }
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label())
    }
}

impl FromStr for TaskStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Not Started" => Ok(Self::NotStarted),
            "In Progress" => Ok(Self::InProgress),
            "Completed" => Ok(Self::Completed),
            "Skipped" => Ok(Self::Skipped),
            _ => Err(format!("unknown task status: {s}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    pub id: Uuid,
    pub assessment_id: Uuid,
    pub title: String,
    pub display_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    pub id: Uuid,
    pub section_id: Uuid,
    pub title: String,
    pub description: String,
    pub display_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub activity_id: Uuid,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub notes: String,
    pub reference: String,
    pub display_order: i32,
}

#[derive(Debug, Clone)]
pub struct ActivityNode {
    pub activity: Activity,
    pub tasks: Vec<Task>,
    pub expanded: bool,
}

#[derive(Debug, Clone)]
pub struct SectionNode {
    pub section: Section,
    pub activities: Vec<ActivityNode>,
    pub expanded: bool,
}
