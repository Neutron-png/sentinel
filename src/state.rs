use std::collections::HashMap;

use uuid::Uuid;

use crate::models::assessment::{Assessment, Environment, Methodology, Scope};
use crate::models::evidence::{Evidence, EvidenceType};
use crate::models::finding::{Confidence, Finding, FindingStatus, Severity};
use crate::models::session::Session;
use crate::models::workflow::SectionNode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusField {
    Name,
    Target,
    Environment,
    Scope,
    Methodology,
    Submit,
}

impl FocusField {
    pub fn next(self) -> Self {
        match self {
            Self::Name => Self::Target,
            Self::Target => Self::Environment,
            Self::Environment => Self::Scope,
            Self::Scope => Self::Methodology,
            Self::Methodology => Self::Submit,
            Self::Submit => Self::Name,
        }
    }
    pub fn prev(self) -> Self {
        match self {
            Self::Name => Self::Submit,
            Self::Target => Self::Name,
            Self::Environment => Self::Target,
            Self::Scope => Self::Environment,
            Self::Methodology => Self::Scope,
            Self::Submit => Self::Methodology,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreateForm {
    pub name: String,
    pub name_cursor: usize,
    pub target: String,
    pub target_cursor: usize,
    pub environment: Environment,
    pub scope: Scope,
    pub methodology: Methodology,
    pub focused_field: FocusField,
    pub errors: Vec<crate::validation::ValidationError>,
    pub success: bool,
}
impl Default for CreateForm {
    fn default() -> Self {
        Self {
            name: String::new(),
            name_cursor: 0,
            target: String::new(),
            target_cursor: 0,
            environment: Environment::Production,
            scope: Scope::WebApplication,
            methodology: Methodology::Owasp,
            focused_field: FocusField::Name,
            errors: Vec::new(),
            success: false,
        }
    }
}
impl CreateForm {
    pub fn set_text_cursor(&mut self, pos: usize) {
        match self.focused_field {
            FocusField::Name => self.name_cursor = pos.min(self.name.len()),
            FocusField::Target => self.target_cursor = pos.min(self.target.len()),
            _ => {}
        }
    }
    pub fn reset(&mut self) {
        self.name.clear();
        self.name_cursor = 0;
        self.target.clear();
        self.target_cursor = 0;
        self.environment = Environment::Production;
        self.scope = Scope::WebApplication;
        self.methodology = Methodology::Owasp;
        self.focused_field = FocusField::Name;
        self.errors.clear();
        self.success = false;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenFocus {
    Search,
    List,
}

#[derive(Debug, Clone)]
pub struct OpenScreen {
    pub search_query: String,
    pub search_cursor: usize,
    pub selected_index: usize,
    pub scroll_offset: usize,
    pub filtered: Vec<Assessment>,
    pub focus: OpenFocus,
}
impl Default for OpenScreen {
    fn default() -> Self {
        Self {
            search_query: String::new(),
            search_cursor: 0,
            selected_index: 0,
            scroll_offset: 0,
            filtered: Vec::new(),
            focus: OpenFocus::Search,
        }
    }
}
impl OpenScreen {
    pub fn refresh(&mut self, assessments: &[Assessment]) {
        self.filtered = if self.search_query.trim().is_empty() {
            assessments.to_vec()
        } else {
            let q = self.search_query.to_lowercase();
            assessments
                .iter()
                .filter(|a| {
                    a.name.to_lowercase().contains(&q) || a.target.to_lowercase().contains(&q)
                })
                .cloned()
                .collect()
        };
        self.selected_index = self
            .selected_index
            .min(self.filtered.len().saturating_sub(1));
        self.scroll_offset = self
            .scroll_offset
            .min(self.filtered.len().saturating_sub(1));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusPath {
    Section(usize),
    Activity(usize, usize),
    Task(usize, usize, usize),
}

// ── Evidence panel ──

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidencePanelMode {
    List,
    Add,
    Edit(usize),
    Detail(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceFormField {
    Type,
    Title,
    Content,
}
impl EvidenceFormField {
    pub fn next(self) -> Self {
        match self {
            Self::Type => Self::Title,
            Self::Title => Self::Content,
            Self::Content => Self::Type,
        }
    }
    pub fn prev(self) -> Self {
        match self {
            Self::Type => Self::Content,
            Self::Title => Self::Type,
            Self::Content => Self::Title,
        }
    }
}
#[derive(Debug, Clone)]
pub struct EvidenceForm {
    pub evidence_type: EvidenceType,
    pub title: String,
    pub title_cursor: usize,
    pub content: String,
    pub content_cursor: usize,
    pub focused_field: EvidenceFormField,
}
impl Default for EvidenceForm {
    fn default() -> Self {
        Self {
            evidence_type: EvidenceType::Note,
            title: String::new(),
            title_cursor: 0,
            content: String::new(),
            content_cursor: 0,
            focused_field: EvidenceFormField::Type,
        }
    }
}
#[derive(Debug, Clone)]
pub struct EvidencePanel {
    pub task_id: Uuid,
    pub items: Vec<Evidence>,
    pub selected_index: usize,
    pub scroll_offset: usize,
    pub mode: EvidencePanelMode,
    pub form: EvidenceForm,
}
impl EvidencePanel {
    pub fn new(task_id: Uuid, items: Vec<Evidence>) -> Self {
        Self {
            task_id,
            items,
            selected_index: 0,
            scroll_offset: 0,
            mode: EvidencePanelMode::List,
            form: EvidenceForm::default(),
        }
    }
}

// ── Findings panel ──

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindingsPanelMode {
    List,
    Add,
    Edit(usize),
    Detail(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindingsFormField {
    Title,
    Description,
    Severity,
    Confidence,
    Status,
    Impact,
    Recommendation,
}
impl FindingsFormField {
    pub fn next(self) -> Self {
        match self {
            Self::Title => Self::Description,
            Self::Description => Self::Severity,
            Self::Severity => Self::Confidence,
            Self::Confidence => Self::Status,
            Self::Status => Self::Impact,
            Self::Impact => Self::Recommendation,
            Self::Recommendation => Self::Title,
        }
    }
    pub fn prev(self) -> Self {
        match self {
            Self::Title => Self::Recommendation,
            Self::Description => Self::Title,
            Self::Severity => Self::Description,
            Self::Confidence => Self::Severity,
            Self::Status => Self::Confidence,
            Self::Impact => Self::Status,
            Self::Recommendation => Self::Impact,
        }
    }
}
#[derive(Debug, Clone)]
pub struct FindingsForm {
    pub title: String,
    pub title_cursor: usize,
    pub description: String,
    pub description_cursor: usize,
    pub severity: Severity,
    pub confidence: Confidence,
    pub status: FindingStatus,
    pub impact: String,
    pub impact_cursor: usize,
    pub recommendation: String,
    pub recommendation_cursor: usize,
    pub focused_field: FindingsFormField,
    #[allow(dead_code)]
    pub linked_evidence_ids: Vec<Uuid>,
}
impl Default for FindingsForm {
    fn default() -> Self {
        Self {
            title: String::new(),
            title_cursor: 0,
            description: String::new(),
            description_cursor: 0,
            severity: Severity::Medium,
            confidence: Confidence::Medium,
            status: FindingStatus::Draft,
            impact: String::new(),
            impact_cursor: 0,
            recommendation: String::new(),
            recommendation_cursor: 0,
            focused_field: FindingsFormField::Title,
            linked_evidence_ids: Vec::new(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct FindingsPanel {
    pub task_id: Uuid,
    pub items: Vec<Finding>,
    pub selected_index: usize,
    pub scroll_offset: usize,
    pub mode: FindingsPanelMode,
    pub form: FindingsForm,
    pub evidence_link_counts: std::collections::HashMap<Uuid, usize>,
}
impl FindingsPanel {
    pub fn new(task_id: Uuid, items: Vec<Finding>) -> Self {
        Self {
            task_id,
            items,
            selected_index: 0,
            scroll_offset: 0,
            mode: FindingsPanelMode::List,
            form: FindingsForm::default(),
            evidence_link_counts: std::collections::HashMap::new(),
        }
    }
}

// ── Task panel (unified) ──

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskPanelTab {
    Evidence,
    Findings,
    Knowledge,
    Http,
}

#[derive(Debug, Clone)]
pub struct TaskPanel {
    pub task_id: Uuid,
    pub active_tab: TaskPanelTab,
    pub evidence: EvidencePanel,
    pub findings: FindingsPanel,
    pub knowledge_entry: Option<crate::models::knowledge::KnowledgeEntry>,
    pub http: crate::screens::http_view::HttpPanelState,
}

#[derive(Debug, Clone)]
pub struct WorkflowState {
    pub sections: Vec<SectionNode>,
    pub scroll_offset: usize,
    pub focus: FocusPath,
    pub evidence_counts: HashMap<Uuid, usize>,
    pub task_panel: Option<TaskPanel>,
}
impl WorkflowState {
    pub fn new(sections: Vec<SectionNode>, evidence_counts: HashMap<Uuid, usize>) -> Self {
        Self {
            sections,
            scroll_offset: 0,
            focus: FocusPath::Section(0),
            evidence_counts,
            task_panel: None,
        }
    }
}

pub use crate::screens::export_import::ExportImportState;
pub use crate::screens::report::ReportState;

#[derive(Debug, Clone)]
pub struct AppState {
    pub should_quit: bool,
    pub terminal_width: u16,
    pub terminal_height: u16,
    pub assessments: Vec<Assessment>,
    pub create_form: CreateForm,
    pub open_screen: OpenScreen,
    pub workflow_state: Option<WorkflowState>,
    pub active_assessment: Option<Assessment>,
    pub report_state: ReportState,
    pub export_import_state: ExportImportState,
    pub pending_session: Option<Session>,
    pub show_resume_prompt: bool,
    pub plugin_count: usize,
}
impl Default for AppState {
    fn default() -> Self {
        Self {
            should_quit: false,
            terminal_width: 80,
            terminal_height: 24,
            assessments: Vec::new(),
            create_form: CreateForm::default(),
            open_screen: OpenScreen::default(),
            workflow_state: None,
            active_assessment: None,
            report_state: ReportState::default(),
            export_import_state: ExportImportState::default(),
            pending_session: None,
            show_resume_prompt: false,
            plugin_count: 0,
        }
    }
}
