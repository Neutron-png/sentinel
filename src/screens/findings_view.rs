use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::services::verification;
use crate::state::{FindingsFormField, FindingsPanel, FindingsPanelMode};

pub fn render(f: &mut Frame, area: Rect, panel: &mut FindingsPanel) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Findings ")
        .title_alignment(Alignment::Center);

    let inner = block.inner(area);
    f.render_widget(block, area);

    match panel.mode {
        FindingsPanelMode::List | FindingsPanelMode::Detail(_) => render_list_view(f, inner, panel),
        FindingsPanelMode::Add | FindingsPanelMode::Edit(_) => render_form(f, inner, panel),
    }
}

fn render_list_view(f: &mut Frame, area: Rect, panel: &mut FindingsPanel) {
    if panel.items.is_empty() {
        let msg = Paragraph::new("No findings.\n\n[A] Add  [D] Delete")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);
        f.render_widget(msg, area);
        return;
    }

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(1)])
        .split(area);
    let list_area = layout[0];
    let hint_area = layout[1];

    let visible = list_area.height as usize;
    if panel.selected_index < panel.scroll_offset {
        panel.scroll_offset = panel.selected_index;
    }
    if panel.selected_index >= panel.scroll_offset + visible {
        panel.scroll_offset = panel.selected_index - visible + 1;
    }

    for (i, finding) in panel
        .items
        .iter()
        .skip(panel.scroll_offset)
        .take(visible)
        .enumerate()
    {
        let y = list_area.y + i as u16;
        let row = Rect {
            x: list_area.x + 1,
            y,
            width: list_area.width.saturating_sub(2),
            height: 1,
        };
        let selected = panel.scroll_offset + i == panel.selected_index;
        let base = if selected {
            Style::default().fg(Color::Black).bg(Color::Cyan)
        } else {
            Style::default()
        };
        let sev_color = severity_color(finding.severity);
        let marker = if selected { ">" } else { " " };
        let ec = panel
            .evidence_link_counts
            .get(&finding.id)
            .copied()
            .unwrap_or(0);
        let vr = verification::verify(finding, ec);
        let vi = vr.status.icon();
        let vc = verification_color(vr.status);
        let line = Line::from(vec![
            Span::styled(format!("{} ", marker), base.add_modifier(Modifier::BOLD)),
            Span::styled(format!("{} ", vi), Style::default().fg(vc)),
            Span::styled(
                format!("[{}] ", finding.severity),
                Style::default().fg(sev_color),
            ),
            Span::styled(&finding.title, base),
        ]);
        f.render_widget(Paragraph::new(line), row);
    }

    if let FindingsPanelMode::Detail(idx) = panel.mode {
        if let Some(fd) = panel.items.get(idx) {
            let ec = panel.evidence_link_counts.get(&fd.id).copied().unwrap_or(0);
            let vr = verification::verify(fd, ec);
            let dy = list_area.y + list_area.height.saturating_sub(8).max(list_area.y);
            let da = Rect {
                x: area.x + 1,
                y: dy,
                width: area.width.saturating_sub(2),
                height: 8.min(list_area.height),
            };
            let db = Block::default().borders(Borders::TOP);
            let di = db.inner(da);
            f.render_widget(db, da);

            let vc = verification_color(vr.status);
            let mut lines = vec![
                Line::from(Span::styled(
                    format!(" Verification: {} ", vr.status.label()),
                    Style::default().fg(vc).add_modifier(Modifier::BOLD),
                )),
                Line::from(Span::styled(
                    format!(" {}: {}", fd.status, fd.description),
                    Style::default().fg(Color::Gray),
                )),
                Line::from(Span::styled(
                    format!(" Impact: {}", fd.impact),
                    Style::default().fg(Color::Gray),
                )),
                Line::from(Span::styled(
                    format!(" Recommendation: {}", fd.recommendation),
                    Style::default().fg(Color::Gray),
                )),
            ];
            if !vr.missing.is_empty() {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    " Missing:",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                )));
                for m in &vr.missing {
                    lines.push(Line::from(Span::styled(
                        format!("   - {}", m),
                        Style::default().fg(Color::Red),
                    )));
                }
            }
            f.render_widget(Paragraph::new(lines), di);
        }
    }

    let hint = Paragraph::new(" A:Add  Enter:View  E:Edit  D:Delete  Left:Back ")
        .style(Style::default().fg(Color::Gray));
    f.render_widget(hint, hint_area);
}

fn render_form(f: &mut Frame, area: Rect, panel: &mut FindingsPanel) {
    let form = &mut panel.form;
    let focused = form.focused_field;

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .split(area);

    let w = 16usize;

    render_text_row(
        f,
        layout[0],
        "Title",
        &form.title,
        form.title_cursor,
        focused == FindingsFormField::Title,
        w,
    );
    render_text_row(
        f,
        layout[1],
        "Description",
        &form.description,
        form.description_cursor,
        focused == FindingsFormField::Description,
        w,
    );
    render_select_row(
        f,
        layout[2],
        "Severity",
        &form.severity.to_string(),
        focused == FindingsFormField::Severity,
        w,
    );
    render_select_row(
        f,
        layout[3],
        "Confidence",
        &form.confidence.to_string(),
        focused == FindingsFormField::Confidence,
        w,
    );
    render_select_row(
        f,
        layout[4],
        "Status",
        &form.status.to_string(),
        focused == FindingsFormField::Status,
        w,
    );
    render_text_row(
        f,
        layout[5],
        "Impact",
        &form.impact,
        form.impact_cursor,
        focused == FindingsFormField::Impact,
        w,
    );
    render_text_row(
        f,
        layout[6],
        "Recommendation",
        &form.recommendation,
        form.recommendation_cursor,
        focused == FindingsFormField::Recommendation,
        w,
    );

    // Verification preview
    let ec = form.linked_evidence_ids.len();
    let tmp = crate::models::finding::Finding {
        id: uuid::Uuid::nil(),
        assessment_id: uuid::Uuid::nil(),
        task_id: panel.task_id,
        title: form.title.clone(),
        description: form.description.clone(),
        severity: form.severity,
        confidence: form.confidence,
        status: form.status,
        impact: form.impact.clone(),
        recommendation: form.recommendation.clone(),
        references: String::new(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    let vr = verification::verify(&tmp, ec);
    let vc = verification_color(vr.status);
    let status_line = format!(
        " Verification: {} | Evidence: {} linked",
        vr.status.label(),
        ec
    );
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            status_line,
            Style::default().fg(vc).add_modifier(Modifier::BOLD),
        ))),
        layout[7],
    );

    let hint = Paragraph::new(" Tab:Next  Enter:Save  Esc:Cancel ")
        .style(Style::default().fg(Color::Gray));
    f.render_widget(hint, layout[8]);
}

fn render_text_row(
    f: &mut Frame,
    area: Rect,
    label: &str,
    value: &str,
    cursor: usize,
    focused: bool,
    w: usize,
) {
    let style = if focused {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };
    let lt = format!("  {:.<width$} ", label, width = w);
    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(&lt, style),
            Span::styled(value, style),
        ])),
        area,
    );
    if focused {
        f.set_cursor_position((
            area.x + lt.len() as u16 + cursor.min(value.len()) as u16,
            area.y,
        ));
    }
}
fn render_select_row(f: &mut Frame, area: Rect, label: &str, value: &str, focused: bool, w: usize) {
    let style = if focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    };
    let lt = format!("  {:.<width$} ", label, width = w);
    let l = Line::from(vec![
        Span::styled(&lt, style),
        Span::styled(format!("< {} >", value), style),
    ]);
    f.render_widget(Paragraph::new(l), area);
}

fn severity_color(severity: crate::models::finding::Severity) -> Color {
    match severity {
        crate::models::finding::Severity::Critical => Color::Red,
        crate::models::finding::Severity::High => Color::LightRed,
        crate::models::finding::Severity::Medium => Color::Yellow,
        crate::models::finding::Severity::Low => Color::Blue,
        crate::models::finding::Severity::Informational => Color::Gray,
    }
}

fn verification_color(status: verification::VerificationStatus) -> Color {
    match status {
        verification::VerificationStatus::Incomplete => Color::Red,
        verification::VerificationStatus::ReadyForReview => Color::Yellow,
        verification::VerificationStatus::Verified => Color::Green,
    }
}
