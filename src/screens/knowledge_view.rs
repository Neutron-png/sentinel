use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::models::knowledge::KnowledgeEntry;

pub fn render(f: &mut Frame, area: Rect, entry: &KnowledgeEntry) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" Knowledge: {} ", entry.reference))
        .title_alignment(Alignment::Center);
    let inner = block.inner(area);
    f.render_widget(block, area);

    let rows = build_rows(entry);
    let visible = inner.height as usize;
    for (i, (label, text)) in rows.iter().take(visible).enumerate() {
        let y = inner.y + i as u16;
        let row_area = Rect {
            x: inner.x,
            y,
            width: inner.width,
            height: 1,
        };
        let l = Line::from(vec![
            Span::styled(
                format!(" {} ", label),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(text, Style::default()),
        ]);
        f.render_widget(Paragraph::new(l), row_area);
    }
}

fn build_rows(entry: &KnowledgeEntry) -> Vec<(&'static str, String)> {
    let mut rows = Vec::new();
    rows.push(("Title:", entry.title.clone()));
    if !entry.objective.is_empty() {
        rows.push(("Objective:", entry.objective.clone()));
    }
    if !entry.description.is_empty() {
        rows.push(("Description:", entry.description.clone()));
    }
    if !entry.prerequisites.is_empty() {
        rows.push(("Prerequisites:", entry.prerequisites.clone()));
    }
    if !entry.testing_steps.is_empty() {
        rows.push(("Testing Steps:", entry.testing_steps.clone()));
    }
    if !entry.expected_result.is_empty() {
        rows.push(("Expected Result:", entry.expected_result.clone()));
    }
    if !entry.required_evidence.is_empty() {
        rows.push(("Required Evidence:", entry.required_evidence.clone()));
    }
    if !entry.references.is_empty() {
        rows.push(("References:", entry.references.clone()));
    }
    rows
}
