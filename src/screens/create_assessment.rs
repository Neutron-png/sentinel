use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::state::{CreateForm, FocusField};

const LABEL_WIDTH: usize = 16;

pub fn render(f: &mut Frame, area: Rect, form: &CreateForm) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Create Assessment ")
        .title_alignment(Alignment::Center);

    let inner = block.inner(area);
    f.render_widget(block, area);

    if form.success {
        render_success(f, inner);
        return;
    }

    let layout = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            ratatui::layout::Constraint::Length(1),
            ratatui::layout::Constraint::Length(1),
            ratatui::layout::Constraint::Length(1),
            ratatui::layout::Constraint::Length(1),
            ratatui::layout::Constraint::Length(1),
            ratatui::layout::Constraint::Length(1),
            ratatui::layout::Constraint::Length(1),
            ratatui::layout::Constraint::Length(1),
            ratatui::layout::Constraint::Min(0),
        ])
        .split(inner);

    let fields = &form.focused_field;
    let errors = &form.errors;

    render_text_field(
        f,
        layout[0],
        "Name",
        &form.name,
        form.name_cursor,
        fields == &FocusField::Name,
        error_for_field(errors, "name"),
    );
    render_text_field(
        f,
        layout[1],
        "Target",
        &form.target,
        form.target_cursor,
        fields == &FocusField::Target,
        error_for_field(errors, "target"),
    );
    render_select_field(
        f,
        layout[2],
        "Environment",
        &form.environment.to_string(),
        fields == &FocusField::Environment,
    );
    render_select_field(
        f,
        layout[3],
        "Scope",
        &form.scope.to_string(),
        fields == &FocusField::Scope,
    );
    render_select_field(
        f,
        layout[4],
        "Methodology",
        &form.methodology.to_string(),
        fields == &FocusField::Methodology,
    );
    render_spacer(f, layout[5]);
    render_submit_button(f, layout[6], fields == &FocusField::Submit);
    render_spacer(f, layout[7]);

    if !errors.is_empty() {
        render_errors(f, layout[8], errors);
    }
}

fn error_for_field(errors: &[crate::validation::ValidationError], field: &str) -> bool {
    errors.iter().any(|e| e.field == field)
}

fn render_text_field(
    f: &mut Frame,
    area: Rect,
    label: &str,
    value: &str,
    cursor: usize,
    focused: bool,
    has_error: bool,
) {
    let style = if has_error {
        Style::default().fg(Color::Red)
    } else if focused {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let label_text = format!("  {:.<width$} ", label, width = LABEL_WIDTH);
    let label_span = Span::styled(&label_text, style);

    let value_display = if value.is_empty() {
        "_".to_string()
    } else {
        value.to_string()
    };

    let value_span = Span::styled(&value_display, style);
    let line = Line::from(vec![label_span, value_span]);
    let paragraph = Paragraph::new(line);
    f.render_widget(paragraph, area);

    if focused {
        if value.is_empty() {
            f.set_cursor_position((area.x + label_text.len() as u16, area.y));
        } else {
            let c = cursor.min(value.len());
            f.set_cursor_position((area.x + label_text.len() as u16 + c as u16, area.y));
        }
    }
}

fn render_select_field(f: &mut Frame, area: Rect, label: &str, value: &str, focused: bool) {
    let style = if focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    };

    let label_text = format!("  {:.<width$} ", label, width = LABEL_WIDTH);
    let value_text = format!("< {} >", value);

    let line = Line::from(vec![
        Span::styled(&label_text, style),
        Span::styled(value_text, style),
    ]);
    f.render_widget(Paragraph::new(line), area);
}

fn render_submit_button(f: &mut Frame, area: Rect, focused: bool) {
    let style = if focused {
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::UNDERLINED)
    } else {
        Style::default()
    };

    let paragraph = Paragraph::new("  [ Create Assessment ]  ")
        .style(style)
        .alignment(Alignment::Center);
    f.render_widget(paragraph, area);
}

fn render_errors(f: &mut Frame, area: Rect, errors: &[crate::validation::ValidationError]) {
    let red = Style::default().fg(Color::Red);
    let lines: Vec<Line> = errors
        .iter()
        .map(|e| Line::from(Span::styled(format!("  Error: {}", e.message), red)))
        .collect();

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, area);
}

fn render_success(f: &mut Frame, area: Rect) {
    let green = Style::default().fg(Color::Green);
    let text = "Assessment created successfully.\n\nPress Enter or Esc to return to Home.";

    let paragraph = Paragraph::new(text)
        .style(green)
        .alignment(Alignment::Center);
    f.render_widget(paragraph, area);
}

fn render_spacer(f: &mut Frame, area: Rect) {
    let paragraph = Paragraph::new("");
    f.render_widget(paragraph, area);
}
