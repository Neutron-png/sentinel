use std::io::{self, stdout};
use std::time::Duration;

use anyhow::Result;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::commands::{self, FormResult, OpenResult, WorkflowResult};
use crate::db::repository::Repository;
use crate::events::Action;
use crate::router::{Router, Screen};
use crate::services;
use crate::state::{
    AppState, EvidencePanel, FindingsPanel, TaskPanel, TaskPanelTab, WorkflowState,
};
use crate::ui;

pub struct App;

impl App {
    pub fn run() -> Result<()> {
        let mut state = AppState::default();
        let mut router = Router::new();
        let db_path = std::path::Path::new("sentinel.db");
        let repo = Repository::open(db_path)?;
        state.assessments = repo.list_assessments().unwrap_or_default();

        // Check for pending session
        if let Some(session) = services::session::load_last(&repo) {
            state.pending_session = Some(session);
            state.show_resume_prompt = true;
        }

        // Plugin system
        let plugins_dir = std::path::Path::new("plugins");
        let mut plugin_manager = crate::plugins::PluginManager::new();
        match plugin_manager.discover(plugins_dir) {
            Ok(count) if count > 0 => {
                let results = plugin_manager.validate_manifests();
                let (valid, invalid): (Vec<_>, Vec<_>) =
                    results.iter().partition(|(_, e)| e.is_none());
                for (_, err) in &invalid {
                    if let Some(e) = err {
                        eprintln!("Plugin validation failed: {e}");
                    }
                }
                state.plugin_count = valid.len();
            }
            Ok(_) => {
                state.plugin_count = 0;
            }
            Err(e) => eprintln!("Plugin discovery failed: {e}"),
        }

        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;
        terminal.clear()?;

        let result = Self::main_loop(&mut terminal, &mut state, &mut router, &repo);

        // Plugin shutdown
        for (name, err) in plugin_manager.shutdown_all() {
            if let Err(e) = err {
                eprintln!("Plugin '{}' shutdown failed: {e}", name);
            }
        }

        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;
        result
    }

    fn main_loop(
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
        state: &mut AppState,
        router: &mut Router,
        repo: &Repository,
    ) -> Result<()> {
        while !state.should_quit {
            terminal.draw(|f| ui::render(f, state, router))?;
            let action = Self::poll_action()?;
            Self::dispatch(&action, state, router, repo);
        }
        Ok(())
    }

    fn poll_action() -> io::Result<Action> {
        if event::poll(Duration::from_millis(16))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => Ok(Action::from(key)),
                Event::Resize(w, h) => Ok(Action::Resize(w, h)),
                _ => Ok(Action::Noop),
            }
        } else {
            Ok(Action::Tick)
        }
    }

    fn dispatch(action: &Action, state: &mut AppState, router: &mut Router, repo: &Repository) {
        // Handle resume prompt
        if state.show_resume_prompt {
            if router.current_screen == Screen::ExportImport {
                match commands::update_export_import(
                    &mut state.export_import_state,
                    state.active_assessment.is_some(),
                    action,
                ) {
                    commands::ExportImportRes::Consumed => {
                        // Handle execution if status was set to "Processing..."
                        if let Some((ref status, _)) = state.export_import_state.status {
                            if status == "Processing..." {
                                let has_active = state.active_assessment.is_some();
                                if has_active {
                                    let result = handle_export_import(
                                        &state.export_import_state,
                                        state,
                                        repo,
                                    );
                                    state.export_import_state.status = result.clone();
                                    if let Some((_, true)) = result {
                                        state.assessments =
                                            repo.list_assessments().unwrap_or_default();
                                    }
                                }
                            }
                        }
                        return;
                    }
                    commands::ExportImportRes::Back => {
                        if !router.go_back() {
                            router.current_screen = Screen::Home;
                        }
                        return;
                    }
                    commands::ExportImportRes::NotConsumed => {}
                }
            }

            match action {
                Action::Char('y') | Action::Char('Y') => {
                    if let Some(ref session) = state.pending_session.take() {
                        if let Ok(Some(assessment)) = repo.get_assessment(&session.assessment_id) {
                            let (sections, counts) = services::workflow::load_or_seed_workflow(
                                repo,
                                assessment.id,
                                &assessment.methodology.to_string(),
                            )
                            .unwrap_or_default();
                            state.workflow_state = Some(WorkflowState::new(sections, counts));
                            state.active_assessment = Some(assessment);
                            match session.current_screen.as_str() {
                                "AssessmentView" => router.current_screen = Screen::AssessmentView,
                                "OpenAssessment" => router.current_screen = Screen::OpenAssessment,
                                _ => router.current_screen = Screen::Home,
                            }
                        } else {
                            services::session::clear_session(repo);
                        }
                    }
                    state.show_resume_prompt = false;
                    return;
                }
                Action::Char('n') | Action::Char('N') => {
                    services::session::clear_session(repo);
                    state.pending_session = None;
                    state.show_resume_prompt = false;
                    return;
                }
                Action::Char(_) => return,
                _ => return,
            }
        }

        if matches!(action, Action::Char('q')) {
            // Auto-save session before quit
            if let Some(ref a) = state.active_assessment {
                services::session::save_current(repo, a.id, screen_name(&router.current_screen));
            }
            state.should_quit = true;
            return;
        }
        if let Action::Resize(w, h) = action {
            state.terminal_width = *w;
            state.terminal_height = *h;
            return;
        }

        if router.current_screen == Screen::CreateAssessment {
            match commands::update_create_form(&mut state.create_form, action) {
                FormResult::Created(assessment) => {
                    if let Err(e) = repo.create_assessment(&assessment) {
                        eprintln!("Failed to save assessment: {e}");
                    }
                    state.assessments.push(assessment);
                    return;
                }
                FormResult::Back => {
                    state.create_form.reset();
                    if !router.go_back() {
                        router.current_screen = Screen::Home;
                    }
                    return;
                }
                FormResult::Consumed => return,
                FormResult::NotConsumed => {}
            }
        }
        if router.current_screen == Screen::OpenAssessment {
            match commands::update_open_screen(&mut state.open_screen, &state.assessments, action) {
                OpenResult::Selected(assessment) => {
                    let (sections, counts) = services::workflow::load_or_seed_workflow(
                        repo,
                        assessment.id,
                        &assessment.methodology.to_string(),
                    )
                    .unwrap_or_default();
                    state.workflow_state = Some(WorkflowState::new(sections, counts));
                    state.active_assessment = Some(assessment.clone());
                    router.navigate_to(Screen::AssessmentView);
                    services::session::save_current(repo, assessment.id, "AssessmentView");
                    return;
                }
                OpenResult::Back => {
                    if !router.go_back() {
                        router.current_screen = Screen::Home;
                    }
                    return;
                }
                OpenResult::Consumed => return,
                OpenResult::NotConsumed => {}
            }
        }
        if router.current_screen == Screen::AssessmentView {
            if let Some(ref mut ws) = state.workflow_state {
                match commands::update_workflow(ws, action) {
                    WorkflowResult::TaskStatusChanged(tid, st) => {
                        if let Err(e) = repo.update_task_status(&tid, &st) {
                            eprintln!("Failed to save task status: {e}");
                        }
                        return;
                    }
                    WorkflowResult::TaskPanelOpened(task_id, reference) => {
                        let evidence = repo.load_evidence(&task_id).unwrap_or_default();
                        let findings = repo.load_findings(&task_id).unwrap_or_default();
                        let mut evidence_link_counts = std::collections::HashMap::new();
                        for f in &findings {
                            if let Ok(ids) = repo.load_linked_evidence_ids(&f.id) {
                                evidence_link_counts.insert(f.id, ids.len());
                            }
                        }
                        let knowledge_entry = crate::knowledge::load_all()
                            .into_iter()
                            .find(|e| e.reference == reference);
                        let transactions = repo.list_transactions(&task_id).unwrap_or_default();
                        let http = crate::screens::http_view::HttpPanelState {
                            task_id,
                            transactions,
                            ..Default::default()
                        };
                        let panel = TaskPanel {
                            task_id,
                            active_tab: TaskPanelTab::Evidence,
                            evidence: EvidencePanel::new(task_id, evidence),
                            findings: FindingsPanel {
                                evidence_link_counts,
                                ..FindingsPanel::new(task_id, findings)
                            },
                            knowledge_entry,
                            http,
                        };
                        ws.task_panel = Some(panel);
                        return;
                    }
                    WorkflowResult::Back => {
                        if !router.go_back() {
                            router.current_screen = Screen::Home;
                        }
                        return;
                    }
                    WorkflowResult::Consumed => return,
                    WorkflowResult::EvidenceSaved(ev) => {
                        let exists = state
                            .workflow_state
                            .as_ref()
                            .and_then(|ws| ws.task_panel.as_ref())
                            .map(|p| p.evidence.items.iter().any(|e| e.id == ev.id))
                            .unwrap_or(false);
                        if exists {
                            if let Err(e) = repo.update_evidence(&ev) {
                                eprintln!("Failed to update evidence: {e}");
                            }
                        } else {
                            if let Err(e) = repo.insert_evidence(&ev) {
                                eprintln!("Failed to save evidence: {e}");
                            }
                        }
                        return;
                    }
                    WorkflowResult::EvidenceDeleted(eid) => {
                        if let Err(e) = repo.delete_evidence(&eid) {
                            eprintln!("Failed to delete evidence: {e}");
                        }
                        return;
                    }
                    WorkflowResult::FindingSaved(f) => {
                        let exists = state
                            .workflow_state
                            .as_ref()
                            .and_then(|ws| ws.task_panel.as_ref())
                            .map(|p| p.findings.items.iter().any(|x| x.id == f.id))
                            .unwrap_or(false);
                        let mut f2 = f.clone();
                        if let Some(ref a) = state.active_assessment {
                            f2.assessment_id = a.id;
                        }
                        if exists {
                            if let Err(e) = repo.update_finding(&f2) {
                                eprintln!("Failed to update finding: {e}");
                            }
                        } else {
                            if let Err(e) = repo.insert_finding(&f2) {
                                eprintln!("Failed to save finding: {e}");
                            }
                        }
                        return;
                    }
                    WorkflowResult::FindingDeleted(fid, _tid) => {
                        if let Err(e) = repo.delete_finding(&fid) {
                            eprintln!("Failed to delete finding: {e}");
                        }
                        return;
                    }
                    WorkflowResult::NotConsumed => {}
                }
            }
        }

        if router.current_screen == Screen::Report {
            match commands::update_report(
                &mut state.report_state,
                &state.workflow_state,
                &state.active_assessment,
                repo,
                action,
            ) {
                commands::ReportScreenResult::Consumed => return,
                commands::ReportScreenResult::Back => {
                    if !router.go_back() {
                        router.current_screen = Screen::Home;
                    }
                    return;
                }
                commands::ReportScreenResult::NotConsumed => {}
            }
        }

        match action {
            Action::Back => {
                if !router.go_back() {
                    state.should_quit = true;
                }
            }
            Action::NavigateHome => router.navigate_to(Screen::Home),
            Action::NavigateCreate => router.navigate_to(Screen::CreateAssessment),
            Action::NavigateOpen => {
                state.open_screen.refresh(&state.assessments);
                router.navigate_to(Screen::OpenAssessment);
            }
            Action::NavigateSettings => router.navigate_to(Screen::Settings),
            Action::NavigateReport => router.navigate_to(Screen::Report),
            Action::NavigateExportImport => router.navigate_to(Screen::ExportImport),
            _ => {}
        }
    }
}

fn screen_name(screen: &Screen) -> &'static str {
    match screen {
        Screen::Home => "Home",
        Screen::CreateAssessment => "CreateAssessment",
        Screen::OpenAssessment => "OpenAssessment",
        Screen::AssessmentView => "AssessmentView",
        Screen::Report => "Report",
        Screen::ExportImport => "ExportImport",
        Screen::Settings => "Settings",
    }
}

fn handle_export_import(
    ei_state: &crate::screens::export_import::ExportImportState,
    app_state: &AppState,
    repo: &Repository,
) -> Option<(String, bool)> {
    use crate::screens::export_import::ExportMode;
    let assessment = app_state.active_assessment.as_ref()?;
    let path = &ei_state.file_path;

    match ei_state.mode {
        ExportMode::Export => {
            let content = crate::services::export::export_project(
                repo,
                assessment.id,
                &assessment.name,
                &assessment.methodology.to_string(),
            );
            match content {
                Ok(json) => match std::fs::write(path, json) {
                    Ok(_) => Some((format!("Exported to {}", path), true)),
                    Err(e) => Some((format!("Export failed: {e}"), false)),
                },
                Err(e) => Some((format!("Export failed: {e}"), false)),
            }
        }
        ExportMode::Import => match std::fs::read_to_string(path) {
            Ok(data) => match serde_json::from_str::<crate::models::manifest::ProjectBundle>(&data)
            {
                Ok(bundle) => match crate::services::import::validate_bundle(&bundle) {
                    Ok(()) => match crate::services::import::import_project(repo, &bundle) {
                        Ok(()) => Some((
                            format!("Imported: {}", bundle.manifest.assessment_name),
                            true,
                        )),
                        Err(e) => Some((format!("Import failed: {e}"), false)),
                    },
                    Err(errs) => Some((format!("Validation: {}", errs.join("; ")), false)),
                },
                Err(e) => Some((format!("Invalid file: {e}"), false)),
            },
            Err(e) => Some((format!("Cannot read file: {e}"), false)),
        },
        _ => Some(("Unknown mode".into(), false)),
    }
}
