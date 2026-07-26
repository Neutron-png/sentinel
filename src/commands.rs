use chrono::Utc;
use uuid::Uuid;

use crate::events::Action;
use crate::models::assessment::{Assessment, AssessmentStatus};
use crate::models::evidence::Evidence;
use crate::models::finding::Finding;
use crate::models::workflow::{SectionNode, TaskStatus};
use crate::state::{
    CreateForm, EvidenceForm, EvidenceFormField, EvidencePanel, EvidencePanelMode, FindingsForm,
    FindingsFormField, FindingsPanel, FindingsPanelMode, FocusField, FocusPath, OpenScreen,
    TaskPanelTab, WorkflowState,
};
use crate::validation;

#[derive(Debug, Clone)]
pub enum FormResult {
    Consumed,
    NotConsumed,
    Created(Assessment),
    Back,
}

pub fn update_create_form(form: &mut CreateForm, action: &Action) -> FormResult {
    if form.success {
        return match action {
            Action::Enter | Action::Back | Action::Char(_) | Action::Tab => {
                form.reset();
                FormResult::Back
            }
            Action::Noop | Action::Tick => FormResult::Consumed,
            _ => FormResult::Consumed,
        };
    }
    match action {
        Action::Tab | Action::Down => {
            form.focused_field = form.focused_field.next();
            FormResult::Consumed
        }
        Action::ShiftTab | Action::Up => {
            form.focused_field = form.focused_field.prev();
            FormResult::Consumed
        }
        Action::Enter => match form.focused_field {
            FocusField::Submit => {
                let errors = validation::validate_assessment_form(&form.name, &form.target);
                if errors.is_empty() {
                    let a = build_assessment(form);
                    form.success = true;
                    return FormResult::Created(a);
                }
                form.errors = errors;
                FormResult::Consumed
            }
            FocusField::Environment => {
                form.environment = form.environment.next();
                FormResult::Consumed
            }
            FocusField::Scope => {
                form.scope = form.scope.next();
                FormResult::Consumed
            }
            FocusField::Methodology => {
                form.methodology = form.methodology.next();
                FormResult::Consumed
            }
            _ => {
                form.focused_field = form.focused_field.next();
                FormResult::Consumed
            }
        },
        Action::Left => {
            form.errors.clear();
            match form.focused_field {
                FocusField::Name => {
                    form.name_cursor = form.name_cursor.saturating_sub(1);
                }
                FocusField::Target => {
                    form.target_cursor = form.target_cursor.saturating_sub(1);
                }
                FocusField::Environment => form.environment = form.environment.prev(),
                FocusField::Scope => form.scope = form.scope.prev(),
                FocusField::Methodology => form.methodology = form.methodology.prev(),
                _ => {}
            }
            FormResult::Consumed
        }
        Action::Right => {
            form.errors.clear();
            match form.focused_field {
                FocusField::Name if form.name_cursor < form.name.len() => {
                    form.name_cursor += 1;
                }
                FocusField::Target if form.target_cursor < form.target.len() => {
                    form.target_cursor += 1;
                }
                FocusField::Environment => form.environment = form.environment.next(),
                FocusField::Scope => form.scope = form.scope.next(),
                FocusField::Methodology => form.methodology = form.methodology.next(),
                _ => {}
            }
            FormResult::Consumed
        }
        Action::Backspace => {
            form.errors.clear();
            match form.focused_field {
                FocusField::Name if form.name_cursor > 0 => {
                    form.name_cursor -= 1;
                    form.name.remove(form.name_cursor);
                }
                FocusField::Target if form.target_cursor > 0 => {
                    form.target_cursor -= 1;
                    form.target.remove(form.target_cursor);
                }
                _ => {}
            }
            FormResult::Consumed
        }
        Action::Delete => {
            form.errors.clear();
            match form.focused_field {
                FocusField::Name if form.name_cursor < form.name.len() => {
                    form.name.remove(form.name_cursor);
                }
                FocusField::Target if form.target_cursor < form.target.len() => {
                    form.target.remove(form.target_cursor);
                }
                _ => {}
            }
            FormResult::Consumed
        }
        Action::Home => {
            match form.focused_field {
                FocusField::Name | FocusField::Target => form.set_text_cursor(0),
                _ => {}
            }
            FormResult::Consumed
        }
        Action::End => {
            match form.focused_field {
                FocusField::Name => form.name_cursor = form.name.len(),
                FocusField::Target => form.target_cursor = form.target.len(),
                _ => {}
            }
            FormResult::Consumed
        }
        Action::Char(c) => {
            form.errors.clear();
            match form.focused_field {
                FocusField::Name => {
                    form.name.insert(form.name_cursor, *c);
                    form.name_cursor += 1;
                }
                FocusField::Target => {
                    form.target.insert(form.target_cursor, *c);
                    form.target_cursor += 1;
                }
                _ => return FormResult::NotConsumed,
            }
            FormResult::Consumed
        }
        Action::Back => FormResult::Back,
        Action::Resize(_, _) | Action::Tick | Action::Noop => FormResult::Consumed,
        _ => FormResult::NotConsumed,
    }
}
fn build_assessment(form: &CreateForm) -> Assessment {
    Assessment {
        id: Uuid::new_v4(),
        name: form.name.clone(),
        target: form.target.clone(),
        environment: form.environment,
        scope: form.scope,
        methodology: form.methodology,
        status: AssessmentStatus::Draft,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

// ── Open screen ──
#[derive(Debug, Clone)]
pub enum OpenResult {
    Consumed,
    NotConsumed,
    Selected(Assessment),
    Back,
}

pub fn update_open_screen(
    screen: &mut OpenScreen,
    assessments: &[Assessment],
    action: &Action,
) -> OpenResult {
    match action {
        Action::Enter => match screen.focus {
            crate::state::OpenFocus::List if !screen.filtered.is_empty() => {
                return OpenResult::Selected(screen.filtered[screen.selected_index].clone())
            }
            crate::state::OpenFocus::Search => {
                screen.focus = crate::state::OpenFocus::List;
            }
            _ => {}
        },
        Action::Tab => {
            screen.focus = crate::state::OpenFocus::List;
        }
        Action::ShiftTab => {
            screen.focus = crate::state::OpenFocus::Search;
        }
        Action::Up => {
            screen.selected_index = screen.selected_index.saturating_sub(1);
            if screen.selected_index < screen.scroll_offset {
                screen.scroll_offset = screen.selected_index;
            }
        }
        Action::Down => {
            let max = screen.filtered.len().saturating_sub(1);
            if screen.selected_index < max {
                screen.selected_index += 1;
            }
            if screen.selected_index >= screen.scroll_offset + 5 {
                screen.scroll_offset = screen.selected_index.saturating_sub(4);
            }
        }
        Action::Left => {
            if screen.focus == crate::state::OpenFocus::Search {
                screen.search_cursor = screen.search_cursor.saturating_sub(1);
            }
        }
        Action::Right => {
            if screen.focus == crate::state::OpenFocus::Search
                && screen.search_cursor < screen.search_query.len()
            {
                screen.search_cursor += 1;
            }
        }
        Action::Home => {
            if screen.focus == crate::state::OpenFocus::Search {
                screen.search_cursor = 0;
            }
        }
        Action::End => {
            if screen.focus == crate::state::OpenFocus::Search {
                screen.search_cursor = screen.search_query.len();
            }
        }
        Action::Char(c) => match screen.focus {
            crate::state::OpenFocus::Search => {
                screen.search_query.insert(screen.search_cursor, *c);
                screen.search_cursor += 1;
                screen.refresh(assessments);
            }
            crate::state::OpenFocus::List => return OpenResult::NotConsumed,
        },
        Action::Backspace => {
            if screen.focus == crate::state::OpenFocus::Search && screen.search_cursor > 0 {
                screen.search_cursor -= 1;
                screen.search_query.remove(screen.search_cursor);
                screen.refresh(assessments);
            }
        }
        Action::Delete => {
            if screen.focus == crate::state::OpenFocus::Search
                && screen.search_cursor < screen.search_query.len()
            {
                screen.search_query.remove(screen.search_cursor);
                screen.refresh(assessments);
            }
        }
        Action::Back => return OpenResult::Back,
        Action::Resize(_, _) | Action::Tick | Action::Noop | Action::Space => {}
        _ => return OpenResult::NotConsumed,
    }
    OpenResult::Consumed
}

// ── Workflow ──
#[derive(Debug, Clone)]
pub enum WorkflowResult {
    Consumed,
    NotConsumed,
    TaskStatusChanged(Uuid, TaskStatus),
    TaskPanelOpened(Uuid, String),
    Back,
    EvidenceSaved(Evidence),
    EvidenceDeleted(Uuid),
    FindingSaved(Finding),
    FindingDeleted(Uuid, Uuid),
}

pub fn update_workflow(state: &mut WorkflowState, action: &Action) -> WorkflowResult {
    if state.task_panel.is_some() {
        let mut panel = state.task_panel.take().unwrap();
        let task_id = panel.task_id;
        let result = match panel.active_tab {
            TaskPanelTab::Evidence => {
                match evidence_ui(&mut panel.evidence, &task_id, state, action) {
                    TabResult::Consumed => WorkflowResult::Consumed,
                    TabResult::Back => {
                        /* panel consumed below */
                        WorkflowResult::Consumed
                    }
                    TabResult::EvidenceSaved(ev) => WorkflowResult::EvidenceSaved(ev),
                    TabResult::EvidenceDeleted(eid) => WorkflowResult::EvidenceDeleted(eid),
                    TabResult::NotConsumed => {
                        if *action == Action::Tab {
                            panel.active_tab = TaskPanelTab::Findings;
                            WorkflowResult::Consumed
                        } else {
                            WorkflowResult::NotConsumed
                        }
                    }
                }
            }
            TaskPanelTab::Findings => {
                match findings_ui(&mut panel.findings, action) {
                    FindingsRes::Consumed => WorkflowResult::Consumed,
                    FindingsRes::Back => {
                        /* panel consumed below */
                        WorkflowResult::Consumed
                    }
                    FindingsRes::Saved(f) => WorkflowResult::FindingSaved(f),
                    FindingsRes::Deleted(fid, tid) => WorkflowResult::FindingDeleted(fid, tid),
                    FindingsRes::NotConsumed => {
                        if *action == Action::Tab {
                            panel.active_tab = TaskPanelTab::Knowledge;
                            WorkflowResult::Consumed
                        } else {
                            WorkflowResult::NotConsumed
                        }
                    }
                }
            }
            TaskPanelTab::Knowledge => {
                if *action == Action::Tab {
                    panel.active_tab = TaskPanelTab::Http;
                    WorkflowResult::Consumed
                } else {
                    WorkflowResult::NotConsumed
                }
            }
            TaskPanelTab::Http => {
                if *action == Action::Tab {
                    panel.active_tab = TaskPanelTab::Evidence;
                    WorkflowResult::Consumed
                } else {
                    WorkflowResult::NotConsumed
                }
            }
        };

        if !matches!(result, WorkflowResult::Back) {
            state.task_panel = Some(panel);
        }
        return result;
    }

    match action {
        Action::Up => {
            move_focus(state, -1);
        }
        Action::Down => {
            move_focus(state, 1);
        }
        Action::Right => match state.focus {
            FocusPath::Section(si) => {
                if let Some(s) = state.sections.get_mut(si) {
                    s.expanded = true;
                }
            }
            FocusPath::Activity(si, ai) => {
                if let Some(s) = state.sections.get_mut(si) {
                    if let Some(a) = s.activities.get_mut(ai) {
                        if a.tasks.is_empty() {
                            s.expanded = true;
                        } else {
                            a.expanded = true;
                            state.focus = FocusPath::Task(si, ai, 0);
                        }
                    }
                }
            }
            FocusPath::Task(si, ai, ti) => {
                if let Some(s) = state.sections.get(si) {
                    if let Some(a) = s.activities.get(ai) {
                        if let Some(t) = a.tasks.get(ti) {
                            return WorkflowResult::TaskPanelOpened(t.id, t.reference.clone());
                        }
                    }
                }
            }
        },
        Action::Left => match state.focus {
            FocusPath::Task(si, ai, _) => state.focus = FocusPath::Activity(si, ai),
            FocusPath::Activity(si, ai) => {
                if let Some(s) = state.sections.get_mut(si) {
                    if let Some(a) = s.activities.get_mut(ai) {
                        a.expanded = false;
                    }
                }
            }
            FocusPath::Section(si) => {
                if let Some(s) = state.sections.get_mut(si) {
                    s.expanded = false;
                }
            }
        },
        Action::Enter | Action::Space => match state.focus {
            FocusPath::Task(si, ai, ti) => {
                if let Some(s) = state.sections.get_mut(si) {
                    if let Some(a) = s.activities.get_mut(ai) {
                        if let Some(t) = a.tasks.get_mut(ti) {
                            t.status = t.status.next();
                            return WorkflowResult::TaskStatusChanged(t.id, t.status);
                        }
                    }
                }
            }
            FocusPath::Activity(si, ai) => {
                if let Some(s) = state.sections.get_mut(si) {
                    if let Some(a) = s.activities.get_mut(ai) {
                        if !a.tasks.is_empty() {
                            a.expanded = !a.expanded;
                            if a.expanded {
                                state.focus = FocusPath::Task(si, ai, 0);
                            }
                        }
                    }
                }
            }
            FocusPath::Section(si) => {
                if let Some(s) = state.sections.get_mut(si) {
                    s.expanded = !s.expanded;
                    if s.expanded && !s.activities.is_empty() {
                        state.focus = FocusPath::Activity(si, 0);
                    }
                }
            }
        },
        Action::Back => return WorkflowResult::Back,
        Action::Resize(_, _) | Action::Tick | Action::Noop => {}
        _ => return WorkflowResult::NotConsumed,
    }
    WorkflowResult::Consumed
}

// ── Evidence tab ──
#[derive(Debug, Clone)]
enum TabResult {
    Consumed,
    NotConsumed,
    EvidenceSaved(Evidence),
    EvidenceDeleted(Uuid),
    Back,
}

fn evidence_ui(
    panel: &mut EvidencePanel,
    task_id: &Uuid,
    state: &mut WorkflowState,
    action: &Action,
) -> TabResult {
    match panel.mode {
        EvidencePanelMode::Add | EvidencePanelMode::Edit(_) => {
            let res = update_evidence_form(panel, action);
            match res {
                EvFormRes::Saved(ev) => {
                    let exists = panel.items.iter().any(|e| e.id == ev.id);
                    if exists {
                        if let Some(p) = panel.items.iter_mut().find(|e| e.id == ev.id) {
                            *p = ev.clone();
                        }
                    } else {
                        panel.items.insert(0, ev.clone());
                    }
                    panel.mode = EvidencePanelMode::List;
                    let count = panel.items.len();
                    state.evidence_counts.insert(*task_id, count);
                    TabResult::EvidenceSaved(ev)
                }
                EvFormRes::Consumed => TabResult::Consumed,
                EvFormRes::Cancelled => {
                    panel.mode = EvidencePanelMode::List;
                    TabResult::Consumed
                }
                EvFormRes::NotConsumed => TabResult::NotConsumed,
            }
        }
        EvidencePanelMode::List | EvidencePanelMode::Detail(_) => {
            update_evidence_list(panel, action)
        }
    }
}

#[derive(Debug, Clone)]
enum EvFormRes {
    Consumed,
    NotConsumed,
    Saved(Evidence),
    Cancelled,
}

fn update_evidence_list(panel: &mut EvidencePanel, action: &Action) -> TabResult {
    if matches!(panel.mode, EvidencePanelMode::Detail(_)) {
        panel.mode = EvidencePanelMode::List;
        return TabResult::Consumed;
    }
    match action {
        Action::Up => {
            panel.selected_index = panel.selected_index.saturating_sub(1);
            TabResult::Consumed
        }
        Action::Down => {
            let max = panel.items.len().saturating_sub(1);
            if panel.selected_index < max {
                panel.selected_index += 1;
            }
            TabResult::Consumed
        }
        Action::Left => TabResult::Back,
        Action::Char('a') | Action::Char('A') => {
            panel.form = Default::default();
            panel.mode = EvidencePanelMode::Add;
            TabResult::Consumed
        }
        Action::Char('d') | Action::Char('D') if !panel.items.is_empty() => {
            let eid = panel.items.get(panel.selected_index).map(|e| e.id).unwrap();
            panel.items.remove(panel.selected_index);
            if panel.selected_index >= panel.items.len() {
                panel.selected_index = panel.items.len().saturating_sub(1);
            }
            panel.mode = EvidencePanelMode::List;
            TabResult::EvidenceDeleted(eid)
        }
        Action::Char('e') | Action::Char('E') if !panel.items.is_empty() => {
            if let Some(ev) = panel.items.get(panel.selected_index) {
                panel.form = EvidenceForm {
                    evidence_type: ev.evidence_type,
                    title: ev.title.clone(),
                    title_cursor: ev.title.len(),
                    content: ev.content.clone(),
                    content_cursor: ev.content.len(),
                    focused_field: EvidenceFormField::Type,
                };
                panel.mode = EvidencePanelMode::Edit(panel.selected_index);
            }
            TabResult::Consumed
        }
        Action::Enter => {
            if panel.selected_index < panel.items.len() {
                panel.mode = EvidencePanelMode::Detail(panel.selected_index);
            }
            TabResult::Consumed
        }
        Action::Back => TabResult::Back,
        Action::Char('d') | Action::Char('D') | Action::Char('e') | Action::Char('E') => {
            TabResult::Consumed
        }
        _ => TabResult::NotConsumed,
    }
}

fn update_evidence_form(panel: &mut EvidencePanel, action: &Action) -> EvFormRes {
    let form = &mut panel.form;
    match action {
        Action::Tab | Action::Down => {
            form.focused_field = form.focused_field.next();
        }
        Action::ShiftTab | Action::Up => {
            form.focused_field = form.focused_field.prev();
        }
        Action::Enter => {
            let id = if let EvidencePanelMode::Edit(idx) = panel.mode {
                panel
                    .items
                    .get(idx)
                    .map(|e| e.id)
                    .unwrap_or_else(Uuid::new_v4)
            } else {
                Uuid::new_v4()
            };
            let ca = if let EvidencePanelMode::Edit(idx) = panel.mode {
                panel
                    .items
                    .get(idx)
                    .map(|e| e.created_at)
                    .unwrap_or_else(Utc::now)
            } else {
                Utc::now()
            };
            let ev = Evidence {
                id,
                assessment_id: Uuid::nil(),
                task_id: panel.task_id,
                evidence_type: form.evidence_type,
                title: form.title.clone(),
                content: form.content.clone(),
                created_at: ca,
                updated_at: Utc::now(),
            };
            return EvFormRes::Saved(ev);
        }
        Action::Back => return EvFormRes::Cancelled,
        Action::Left => match form.focused_field {
            EvidenceFormField::Type => form.evidence_type = form.evidence_type.prev(),
            EvidenceFormField::Title => form.title_cursor = form.title_cursor.saturating_sub(1),
            EvidenceFormField::Content => {
                form.content_cursor = form.content_cursor.saturating_sub(1)
            }
        },
        Action::Right => match form.focused_field {
            EvidenceFormField::Type => form.evidence_type = form.evidence_type.next(),
            EvidenceFormField::Title if form.title_cursor < form.title.len() => {
                form.title_cursor += 1
            }
            EvidenceFormField::Content if form.content_cursor < form.content.len() => {
                form.content_cursor += 1
            }
            _ => {}
        },
        Action::Home => match form.focused_field {
            EvidenceFormField::Title => form.title_cursor = 0,
            EvidenceFormField::Content => form.content_cursor = 0,
            _ => {}
        },
        Action::End => match form.focused_field {
            EvidenceFormField::Title => form.title_cursor = form.title.len(),
            EvidenceFormField::Content => form.content_cursor = form.content.len(),
            _ => {}
        },
        Action::Backspace => match form.focused_field {
            EvidenceFormField::Title if form.title_cursor > 0 => {
                form.title_cursor -= 1;
                form.title.remove(form.title_cursor);
            }
            EvidenceFormField::Content if form.content_cursor > 0 => {
                form.content_cursor -= 1;
                form.content.remove(form.content_cursor);
            }
            _ => {}
        },
        Action::Delete => match form.focused_field {
            EvidenceFormField::Title if form.title_cursor < form.title.len() => {
                form.title.remove(form.title_cursor);
            }
            EvidenceFormField::Content if form.content_cursor < form.content.len() => {
                form.content.remove(form.content_cursor);
            }
            _ => {}
        },
        Action::Char(c) => match form.focused_field {
            EvidenceFormField::Title => {
                form.title.insert(form.title_cursor, *c);
                form.title_cursor += 1;
            }
            EvidenceFormField::Content => {
                form.content.insert(form.content_cursor, *c);
                form.content_cursor += 1;
            }
            _ => return EvFormRes::NotConsumed,
        },
        Action::Resize(_, _) | Action::Tick | Action::Noop | Action::Space => {}
        _ => return EvFormRes::NotConsumed,
    }
    EvFormRes::Consumed
}

// ── Findings tab ──
#[derive(Debug, Clone)]
enum FindingsRes {
    Consumed,
    NotConsumed,
    Saved(Finding),
    Deleted(Uuid, Uuid),
    Back,
}

fn findings_ui(panel: &mut FindingsPanel, action: &Action) -> FindingsRes {
    match panel.mode {
        FindingsPanelMode::Add | FindingsPanelMode::Edit(_) => {
            let res = update_finding_form(panel, action);
            match res {
                FdFormRes::Saved(f) => {
                    let exists = panel.items.iter().any(|x| x.id == f.id);
                    if exists {
                        if let Some(p) = panel.items.iter_mut().find(|x| x.id == f.id) {
                            *p = f.clone();
                        }
                    } else {
                        panel.items.insert(0, f.clone());
                    }
                    panel.mode = FindingsPanelMode::List;
                    FindingsRes::Saved(f)
                }
                FdFormRes::Consumed => FindingsRes::Consumed,
                FdFormRes::Cancelled => {
                    panel.mode = FindingsPanelMode::List;
                    FindingsRes::Consumed
                }
                FdFormRes::NotConsumed => FindingsRes::NotConsumed,
            }
        }
        FindingsPanelMode::List | FindingsPanelMode::Detail(_) => {
            update_finding_list(panel, action)
        }
    }
}

#[derive(Debug, Clone)]
enum FdFormRes {
    Consumed,
    NotConsumed,
    Saved(Finding),
    Cancelled,
}

fn update_finding_list(panel: &mut FindingsPanel, action: &Action) -> FindingsRes {
    if matches!(panel.mode, FindingsPanelMode::Detail(_)) {
        panel.mode = FindingsPanelMode::List;
        return FindingsRes::Consumed;
    }
    match action {
        Action::Up => {
            panel.selected_index = panel.selected_index.saturating_sub(1);
            FindingsRes::Consumed
        }
        Action::Down => {
            let max = panel.items.len().saturating_sub(1);
            if panel.selected_index < max {
                panel.selected_index += 1;
            }
            FindingsRes::Consumed
        }
        Action::Left => FindingsRes::Back,
        Action::Char('a') | Action::Char('A') => {
            panel.form = Default::default();
            panel.mode = FindingsPanelMode::Add;
            FindingsRes::Consumed
        }
        Action::Char('d') | Action::Char('D') if !panel.items.is_empty() => {
            let fid = panel.items.get(panel.selected_index).map(|f| f.id).unwrap();
            let tid = panel.task_id;
            panel.items.remove(panel.selected_index);
            if panel.selected_index >= panel.items.len() {
                panel.selected_index = panel.items.len().saturating_sub(1);
            }
            panel.mode = FindingsPanelMode::List;
            FindingsRes::Deleted(fid, tid)
        }
        Action::Char('e') | Action::Char('E') if !panel.items.is_empty() => {
            if let Some(f) = panel.items.get(panel.selected_index) {
                panel.form = FindingsForm {
                    title: f.title.clone(),
                    title_cursor: f.title.len(),
                    description: f.description.clone(),
                    description_cursor: f.description.len(),
                    severity: f.severity,
                    confidence: f.confidence,
                    status: f.status,
                    impact: f.impact.clone(),
                    impact_cursor: f.impact.len(),
                    recommendation: f.recommendation.clone(),
                    recommendation_cursor: f.recommendation.len(),
                    focused_field: FindingsFormField::Title,
                    linked_evidence_ids: Vec::new(),
                };
                panel.mode = FindingsPanelMode::Edit(panel.selected_index);
            }
            FindingsRes::Consumed
        }
        Action::Enter => {
            if panel.selected_index < panel.items.len() {
                panel.mode = FindingsPanelMode::Detail(panel.selected_index);
            }
            FindingsRes::Consumed
        }
        Action::Back => FindingsRes::Back,
        Action::Char('d') | Action::Char('D') | Action::Char('e') | Action::Char('E') => {
            FindingsRes::Consumed
        }
        _ => FindingsRes::NotConsumed,
    }
}

fn update_finding_form(panel: &mut FindingsPanel, action: &Action) -> FdFormRes {
    let form = &mut panel.form;
    match action {
        Action::Tab | Action::Down => {
            form.focused_field = form.focused_field.next();
        }
        Action::ShiftTab | Action::Up => {
            form.focused_field = form.focused_field.prev();
        }
        Action::Enter => {
            let id = if let FindingsPanelMode::Edit(idx) = panel.mode {
                panel
                    .items
                    .get(idx)
                    .map(|f| f.id)
                    .unwrap_or_else(Uuid::new_v4)
            } else {
                Uuid::new_v4()
            };
            let ca = if let FindingsPanelMode::Edit(idx) = panel.mode {
                panel
                    .items
                    .get(idx)
                    .map(|f| f.created_at)
                    .unwrap_or_else(Utc::now)
            } else {
                Utc::now()
            };
            let f = Finding {
                id,
                assessment_id: Uuid::nil(),
                task_id: panel.task_id,
                title: form.title.clone(),
                description: form.description.clone(),
                severity: form.severity,
                confidence: form.confidence,
                status: form.status,
                impact: form.impact.clone(),
                recommendation: form.recommendation.clone(),
                references: String::new(),
                created_at: ca,
                updated_at: Utc::now(),
            };
            return FdFormRes::Saved(f);
        }
        Action::Back => return FdFormRes::Cancelled,
        Action::Left => match form.focused_field {
            FindingsFormField::Title => form.title_cursor = form.title_cursor.saturating_sub(1),
            FindingsFormField::Description => {
                form.description_cursor = form.description_cursor.saturating_sub(1)
            }
            FindingsFormField::Severity => form.severity = form.severity.prev(),
            FindingsFormField::Confidence => form.confidence = form.confidence.prev(),
            FindingsFormField::Status => form.status = form.status.prev(),
            FindingsFormField::Impact => form.impact_cursor = form.impact_cursor.saturating_sub(1),
            FindingsFormField::Recommendation => {
                form.recommendation_cursor = form.recommendation_cursor.saturating_sub(1)
            }
        },
        Action::Right => match form.focused_field {
            FindingsFormField::Title if form.title_cursor < form.title.len() => {
                form.title_cursor += 1
            }
            FindingsFormField::Description if form.description_cursor < form.description.len() => {
                form.description_cursor += 1
            }
            FindingsFormField::Severity => form.severity = form.severity.next(),
            FindingsFormField::Confidence => form.confidence = form.confidence.next(),
            FindingsFormField::Status => form.status = form.status.next(),
            FindingsFormField::Impact if form.impact_cursor < form.impact.len() => {
                form.impact_cursor += 1
            }
            FindingsFormField::Recommendation
                if form.recommendation_cursor < form.recommendation.len() =>
            {
                form.recommendation_cursor += 1
            }
            _ => {}
        },
        Action::Home => match form.focused_field {
            FindingsFormField::Title => form.title_cursor = 0,
            FindingsFormField::Description => form.description_cursor = 0,
            FindingsFormField::Impact => form.impact_cursor = 0,
            FindingsFormField::Recommendation => form.recommendation_cursor = 0,
            _ => {}
        },
        Action::End => match form.focused_field {
            FindingsFormField::Title => form.title_cursor = form.title.len(),
            FindingsFormField::Description => form.description_cursor = form.description.len(),
            FindingsFormField::Impact => form.impact_cursor = form.impact.len(),
            FindingsFormField::Recommendation => {
                form.recommendation_cursor = form.recommendation.len()
            }
            _ => {}
        },
        Action::Backspace => match form.focused_field {
            FindingsFormField::Title if form.title_cursor > 0 => {
                form.title_cursor -= 1;
                form.title.remove(form.title_cursor);
            }
            FindingsFormField::Description if form.description_cursor > 0 => {
                form.description_cursor -= 1;
                form.description.remove(form.description_cursor);
            }
            FindingsFormField::Impact if form.impact_cursor > 0 => {
                form.impact_cursor -= 1;
                form.impact.remove(form.impact_cursor);
            }
            FindingsFormField::Recommendation if form.recommendation_cursor > 0 => {
                form.recommendation_cursor -= 1;
                form.recommendation.remove(form.recommendation_cursor);
            }
            _ => {}
        },
        Action::Delete => match form.focused_field {
            FindingsFormField::Title if form.title_cursor < form.title.len() => {
                form.title.remove(form.title_cursor);
            }
            FindingsFormField::Description if form.description_cursor < form.description.len() => {
                form.description.remove(form.description_cursor);
            }
            FindingsFormField::Impact if form.impact_cursor < form.impact.len() => {
                form.impact.remove(form.impact_cursor);
            }
            FindingsFormField::Recommendation
                if form.recommendation_cursor < form.recommendation.len() =>
            {
                form.recommendation.remove(form.recommendation_cursor);
            }
            _ => {}
        },
        Action::Char(c) => match form.focused_field {
            FindingsFormField::Title => {
                form.title.insert(form.title_cursor, *c);
                form.title_cursor += 1;
            }
            FindingsFormField::Description => {
                form.description.insert(form.description_cursor, *c);
                form.description_cursor += 1;
            }
            FindingsFormField::Impact => {
                form.impact.insert(form.impact_cursor, *c);
                form.impact_cursor += 1;
            }
            FindingsFormField::Recommendation => {
                form.recommendation.insert(form.recommendation_cursor, *c);
                form.recommendation_cursor += 1;
            }
            _ => return FdFormRes::NotConsumed,
        },
        Action::Resize(_, _) | Action::Tick | Action::Noop | Action::Space => {}
        _ => return FdFormRes::NotConsumed,
    }
    FdFormRes::Consumed
}

// ── Helpers ──
fn visible_items(sections: &[SectionNode]) -> Vec<VisibleItem> {
    let mut items = Vec::new();
    for (si, section) in sections.iter().enumerate() {
        items.push(VisibleItem::Section { si });
        if section.expanded {
            for (ai, activity) in section.activities.iter().enumerate() {
                items.push(VisibleItem::Activity { si, ai });
                if activity.expanded {
                    for ti in 0..activity.tasks.len() {
                        items.push(VisibleItem::Task { si, ai, ti });
                    }
                }
            }
        }
    }
    items
}
#[derive(Debug, Clone, Copy)]
enum VisibleItem {
    Section { si: usize },
    Activity { si: usize, ai: usize },
    Task { si: usize, ai: usize, ti: usize },
}

fn move_focus(state: &mut WorkflowState, delta: isize) {
    let items = visible_items(&state.sections);
    if items.is_empty() {
        return;
    }
    let current = match state.focus {
        FocusPath::Section(si) => items.iter().position(|it| matches!(it, VisibleItem::Section { si: s } if *s == si)),
        FocusPath::Activity(si, ai) => items.iter().position(|it| matches!(it, VisibleItem::Activity { si: s, ai: a } if *s == si && *a == ai)),
        FocusPath::Task(si, ai, ti) => items.iter().position(|it| matches!(it, VisibleItem::Task { si: s, ai: a, ti: t } if *s == si && *a == ai && *t == ti)),
    };
    let new_idx = if let Some(idx) = current {
        if delta < 0 {
            idx.saturating_sub(1)
        } else {
            (idx + 1).min(items.len() - 1)
        }
    } else {
        0
    };
    if let Some(item) = items.get(new_idx) {
        state.focus = match item {
            VisibleItem::Section { si } => FocusPath::Section(*si),
            VisibleItem::Activity { si, ai } => FocusPath::Activity(*si, *ai),
            VisibleItem::Task { si, ai, ti } => FocusPath::Task(*si, *ai, *ti),
        };
    }
}

// ── Report screen ──

#[derive(Debug, Clone)]
pub enum ReportScreenResult {
    Consumed,
    NotConsumed,
    Back,
}

pub fn update_report(
    state: &mut crate::screens::report::ReportState,
    workflow_state: &Option<WorkflowState>,
    active_assessment: &Option<Assessment>,
    repo: &crate::db::repository::Repository,
    action: &Action,
) -> ReportScreenResult {
    // If showing status, any key dismisses
    if state.status.is_some() {
        state.status = None;
        state.success = false;
        return ReportScreenResult::Consumed;
    }

    use crate::screens::report::ReportFocus;
    match action {
        Action::Tab | Action::Down => {
            state.focus = state.focus.next();
            ReportScreenResult::Consumed
        }
        Action::ShiftTab | Action::Up => {
            state.focus = state.focus.prev();
            ReportScreenResult::Consumed
        }
        Action::Enter => match state.focus {
            ReportFocus::Format => {
                state.format = state.format.next();
                ReportScreenResult::Consumed
            }
            ReportFocus::Generate => {
                // Generate the report
                if let Some(ref assessment) = active_assessment {
                    if let Some(ref ws) = workflow_state {
                        match crate::reports::generator::build_report_data(
                            repo,
                            assessment,
                            &ws.sections,
                            &ws.evidence_counts,
                        ) {
                            Ok(data) => {
                                use crate::reports::ReportExporter;
                                let (content, ext) = match state.format {
                                    crate::screens::report::ReportFormat::Markdown => {
                                        (crate::reports::MarkdownExporter.export(&data), "md")
                                    }
                                    crate::screens::report::ReportFormat::Html => {
                                        (crate::reports::HtmlExporter.export(&data), "html")
                                    }
                                    crate::screens::report::ReportFormat::Json => {
                                        (crate::reports::JsonExporter.export(&data), "json")
                                    }
                                };
                                match content {
                                    Ok(text) => {
                                        let dir = std::path::Path::new(&state.output_dir);
                                        if let Err(e) = std::fs::create_dir_all(dir) {
                                            state.status =
                                                Some(format!("Error creating directory: {e}"));
                                            state.success = false;
                                        } else {
                                            let filename = format!(
                                                "sentinel_report_{}.{}",
                                                assessment.created_at.format("%Y%m%d_%H%M%S"),
                                                ext
                                            );
                                            let path = dir.join(&filename);
                                            if let Err(e) = std::fs::write(&path, text) {
                                                state.status =
                                                    Some(format!("Error writing file: {e}"));
                                                state.success = false;
                                            } else {
                                                state.status = Some(format!(
                                                    "Report saved to {}",
                                                    path.display()
                                                ));
                                                state.success = true;
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        state.status = Some(format!("Report error: {e}"));
                                        state.success = false;
                                    }
                                }
                            }
                            Err(e) => {
                                state.status = Some(format!("Data error: {e}"));
                                state.success = false;
                            }
                        }
                    } else {
                        state.status = Some("No workflow state.".into());
                        state.success = false;
                    }
                } else {
                    state.status = Some("No active assessment.".into());
                    state.success = false;
                }
                ReportScreenResult::Consumed
            }
            _ => ReportScreenResult::Consumed,
        },
        Action::Left => {
            state.format = state.format.prev();
            ReportScreenResult::Consumed
        }
        Action::Right => {
            state.format = state.format.next();
            ReportScreenResult::Consumed
        }
        Action::Back => ReportScreenResult::Back,
        Action::Resize(_, _) | Action::Tick | Action::Noop | Action::Space => {
            ReportScreenResult::Consumed
        }
        Action::Char(c) => match state.focus {
            ReportFocus::OutputDir => {
                state.output_dir.insert(state.output_dir_cursor, *c);
                state.output_dir_cursor += 1;
                ReportScreenResult::Consumed
            }
            _ => {
                state.focus = ReportFocus::OutputDir;
                ReportScreenResult::Consumed
            }
        },
        Action::Backspace
            if state.focus == ReportFocus::OutputDir && state.output_dir_cursor > 0 =>
        {
            state.output_dir_cursor -= 1;
            state.output_dir.remove(state.output_dir_cursor);
            ReportScreenResult::Consumed
        }
        _ => ReportScreenResult::NotConsumed,
    }
}

// ── Export/Import screen ──

#[derive(Debug, Clone)]
pub enum ExportImportRes {
    Consumed,
    NotConsumed,
    Back,
}

pub fn update_export_import(
    state: &mut crate::screens::export_import::ExportImportState,
    has_active: bool,
    action: &Action,
) -> ExportImportRes {
    use crate::screens::export_import::{ExportFocus, ExportMode};
    if !has_active {
        return match action {
            Action::Back => ExportImportRes::Back,
            Action::Char(_) => ExportImportRes::NotConsumed,
            _ => ExportImportRes::Consumed,
        };
    }
    if state.status.is_some() {
        state.status = None;
        return ExportImportRes::Consumed;
    }
    match state.mode {
        ExportMode::Menu => match action {
            Action::Up | Action::Down => {
                state.focus = if state.focus == ExportFocus::ExportBtn {
                    ExportFocus::ImportBtn
                } else {
                    ExportFocus::ExportBtn
                };
                ExportImportRes::Consumed
            }
            Action::Enter => match state.focus {
                ExportFocus::ExportBtn => {
                    state.mode = ExportMode::Export;
                    state.focus = ExportFocus::FilePath;
                    ExportImportRes::Consumed
                }
                ExportFocus::ImportBtn => {
                    state.mode = ExportMode::Import;
                    state.focus = ExportFocus::FilePath;
                    ExportImportRes::Consumed
                }
                _ => ExportImportRes::Consumed,
            },
            Action::Back => ExportImportRes::Back,
            _ => ExportImportRes::NotConsumed,
        },
        ExportMode::Export | ExportMode::Import => match action {
            Action::Enter => {
                state.status = Some(("Processing...".into(), true));
                ExportImportRes::Consumed
            }
            Action::Tab => {
                state.focus = ExportFocus::FilePath;
                ExportImportRes::Consumed
            }
            Action::Left if state.focus == ExportFocus::FilePath => {
                state.file_path_cursor = state.file_path_cursor.saturating_sub(1);
                ExportImportRes::Consumed
            }
            Action::Right
                if state.focus == ExportFocus::FilePath
                    && state.file_path_cursor < state.file_path.len() =>
            {
                state.file_path_cursor += 1;
                ExportImportRes::Consumed
            }
            Action::Home if state.focus == ExportFocus::FilePath => {
                state.file_path_cursor = 0;
                ExportImportRes::Consumed
            }
            Action::End if state.focus == ExportFocus::FilePath => {
                state.file_path_cursor = state.file_path.len();
                ExportImportRes::Consumed
            }
            Action::Backspace
                if state.focus == ExportFocus::FilePath && state.file_path_cursor > 0 =>
            {
                state.file_path_cursor -= 1;
                state.file_path.remove(state.file_path_cursor);
                ExportImportRes::Consumed
            }
            Action::Delete
                if state.focus == ExportFocus::FilePath
                    && state.file_path_cursor < state.file_path.len() =>
            {
                state.file_path.remove(state.file_path_cursor);
                ExportImportRes::Consumed
            }
            Action::Char(c) if state.focus == ExportFocus::FilePath => {
                state.file_path.insert(state.file_path_cursor, *c);
                state.file_path_cursor += 1;
                ExportImportRes::Consumed
            }
            Action::Back => {
                state.mode = ExportMode::Menu;
                state.focus = ExportFocus::ExportBtn;
                ExportImportRes::Consumed
            }
            Action::Resize(_, _) | Action::Tick | Action::Noop | Action::Space => {
                ExportImportRes::Consumed
            }
            _ => ExportImportRes::NotConsumed,
        },
    }
}
