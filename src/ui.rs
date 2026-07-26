use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::Paragraph,
    Frame,
};

use crate::router::{Router, Screen};
use crate::screens;
use crate::state::AppState;

pub fn render(f: &mut Frame, state: &mut AppState, router: &Router) {
    let area = f.area();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(area);

    let main = layout[0];
    let hint = layout[1];

    render_screen(f, main, state, router.current_screen);
    render_hint(f, hint, state, router);
}

fn render_screen(f: &mut Frame, area: Rect, state: &mut AppState, screen: Screen) {
    match screen {
        Screen::Home => {
            let session_name = state.active_assessment.as_ref().map(|a| a.name.clone());
            screens::home::render(
                f,
                area,
                &state.assessments,
                &state.active_assessment,
                state.show_resume_prompt,
                session_name.as_deref(),
            )
        }
        Screen::CreateAssessment => screens::create_assessment::render(f, area, &state.create_form),
        Screen::OpenAssessment => {
            screens::open_assessment::render(f, area, &mut state.open_screen);
        }
        Screen::AssessmentView => {
            let name = state
                .active_assessment
                .as_ref()
                .map(|a| a.name.as_str())
                .unwrap_or("Unknown");
            if let Some(ref mut ws) = state.workflow_state {
                screens::assessment_view::render(f, area, ws, name);
            }
        }
        Screen::Report => {
            screens::report::render(
                f,
                area,
                &mut state.report_state,
                state.active_assessment.is_some(),
            );
        }
        Screen::ExportImport => {
            screens::export_import::render(
                f,
                area,
                &mut state.export_import_state,
                state.active_assessment.is_some(),
            );
        }
        Screen::Settings => screens::settings::render(f, area, state.plugin_count),
    }
}

fn render_hint(f: &mut Frame, area: Rect, state: &AppState, router: &Router) {
    let hint_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(1), Constraint::Length(10)])
        .split(area);

    let label = if router.can_go_back() {
        " q:Quit | Esc:Back | 1-6:Navigate | Arrows/Tab/Enter "
    } else {
        " q:Quit | 1:Home 2:Create 3:Open 4:Settings 5:Report 6:Import | Arrows/Tab/Enter "
    };

    let size = format!("{}x{}", state.terminal_width, state.terminal_height);
    let gray = Style::default().fg(Color::Gray);

    f.render_widget(
        Paragraph::new(label).style(gray).alignment(Alignment::Left),
        hint_layout[0],
    );
    f.render_widget(
        Paragraph::new(size).style(gray).alignment(Alignment::Right),
        hint_layout[1],
    );
}
