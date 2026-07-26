use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::models::assessment::Assessment;
use crate::state::{OpenFocus, OpenScreen};

pub fn render(f: &mut Frame, area: Rect, state: &mut OpenScreen) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Open Assessment ")
        .title_alignment(Alignment::Center);

    let inner = block.inner(area);
    f.render_widget(block, area);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(inner);

    let search_area = layout[0];
    let list_area = layout[1];

    render_search_bar(f, search_area, state);
    render_list(f, list_area, state);
}

fn render_search_bar(f: &mut Frame, area: Rect, state: &mut OpenScreen) {
    let search_block = Block::default()
        .borders(Borders::ALL)
        .style(search_style(state.focus == OpenFocus::Search));

    let inner = search_block.inner(area);
    f.render_widget(search_block, area);

    let label_span = Span::styled(" Search: ", Style::default().fg(Color::Gray));
    let query_span = Span::styled(
        state.search_query.as_str(),
        search_style(state.focus == OpenFocus::Search),
    );

    let line = Line::from(vec![label_span, query_span]);
    f.render_widget(Paragraph::new(line), inner);

    if state.focus == OpenFocus::Search {
        let x = inner.x + 9 + state.search_cursor.min(state.search_query.len()) as u16;
        f.set_cursor_position((x, inner.y));
    }
}

fn render_list(f: &mut Frame, area: Rect, state: &mut OpenScreen) {
    if state.filtered.is_empty() {
        render_empty_state(f, area, state.search_query.is_empty());
        return;
    }

    let item_height: u16 = 2;
    let visible: usize = (area.height as usize).saturating_sub(2) / item_height as usize;
    let visible = visible.max(1);

    if state.selected_index < state.scroll_offset {
        state.scroll_offset = state.selected_index;
    }
    if state.selected_index >= state.scroll_offset + visible {
        state.scroll_offset = state.selected_index - visible + 1;
    }

    let visible_items: Vec<&Assessment> = state
        .filtered
        .iter()
        .skip(state.scroll_offset)
        .take(visible)
        .collect();

    let rows: Vec<ratatui::layout::Rect> = (0..visible)
        .map(|i| Rect {
            x: area.x,
            y: area.y + i as u16 * item_height,
            width: area.width,
            height: item_height,
        })
        .collect();

    for (i, assessment) in visible_items.iter().enumerate() {
        if let Some(row_area) = rows.get(i) {
            let global_idx = state.scroll_offset + i;
            let selected = global_idx == state.selected_index;
            render_list_item(f, *row_area, assessment, selected);
        }
    }
}

fn render_list_item(f: &mut Frame, area: Rect, assessment: &Assessment, selected: bool) {
    let style = if selected {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let name_span = Span::styled(&assessment.name, style);
    let status_span = Span::styled(
        format!(" {}", assessment.status),
        Style::default().fg(status_color(assessment.status)),
    );

    let target_span = Span::styled(
        format!(" {}", assessment.target),
        Style::default().fg(Color::DarkGray),
    );
    let date_span = Span::styled(
        format!(" {}", assessment.created_at.format("%Y-%m-%d")),
        Style::default().fg(Color::DarkGray),
    );

    let top_line = if selected {
        Line::from(vec![Span::styled(" > ", style), name_span, status_span])
    } else {
        Line::from(vec![Span::styled("   ", style), name_span, status_span])
    };

    let bot_line = Line::from(vec![
        Span::raw("   "),
        target_span,
        Span::raw(" "),
        date_span,
    ]);

    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(1)])
        .split(area);

    f.render_widget(Paragraph::new(top_line), inner[0]);
    f.render_widget(Paragraph::new(bot_line), inner[1]);
}

fn render_empty_state(f: &mut Frame, area: Rect, no_assessments: bool) {
    let message = if no_assessments {
        "No assessments yet.\n\nPress 2 to create one."
    } else {
        "No assessments match your search."
    };

    let paragraph = Paragraph::new(message)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);

    f.render_widget(paragraph, area);
}

fn search_style(focused: bool) -> Style {
    if focused {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    }
}

fn status_color(status: crate::models::assessment::AssessmentStatus) -> Color {
    match status {
        crate::models::assessment::AssessmentStatus::Draft => Color::Gray,
        crate::models::assessment::AssessmentStatus::Active => Color::Green,
        crate::models::assessment::AssessmentStatus::Completed => Color::Blue,
        crate::models::assessment::AssessmentStatus::Archived => Color::DarkGray,
    }
}
