use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::state::{EvidenceFormField, EvidencePanel, EvidencePanelMode};

pub fn render(f: &mut Frame, area: Rect, panel: &mut EvidencePanel) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Evidence ")
        .title_alignment(Alignment::Center);

    let inner = block.inner(area);
    f.render_widget(block, area);

    match panel.mode {
        EvidencePanelMode::List | EvidencePanelMode::Detail(_) => {
            render_list_view(f, inner, panel);
        }
        EvidencePanelMode::Add | EvidencePanelMode::Edit(_) => {
            render_form(f, inner, panel);
        }
    }
}

fn render_list_view(f: &mut Frame, area: Rect, panel: &mut EvidencePanel) {
    if panel.items.is_empty() {
        let msg = Paragraph::new("No evidence.\n\n[A] Add  [D] Delete")
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

    for (i, ev) in panel
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

        let global = panel.scroll_offset + i;
        let selected = global == panel.selected_index;
        let style = if selected {
            Style::default().fg(Color::Black).bg(Color::Cyan)
        } else {
            Style::default()
        };

        let marker = if selected { ">" } else { " " };
        let line = Line::from(vec![
            Span::styled(
                format!("{} {}  ", marker, ev.evidence_type),
                style.add_modifier(Modifier::BOLD),
            ),
            Span::styled(&ev.title, style),
        ]);
        f.render_widget(Paragraph::new(line), row);
    }

    let hint = Paragraph::new(" A:Add  Enter:View  E:Edit  D:Delete  Left:Back ")
        .style(Style::default().fg(Color::Gray));
    f.render_widget(hint, hint_area);

    // Detail preview
    if let EvidencePanelMode::Detail(idx) = panel.mode {
        if let Some(ev) = panel.items.get(idx) {
            let preview_area = Rect {
                x: area.x + 1,
                y: area.y + area.height.saturating_sub(5).min(area.height / 2),
                width: area.width.saturating_sub(2),
                height: 5,
            };
            let detail_block = Block::default().borders(Borders::TOP);
            let d_inner = detail_block.inner(preview_area);
            f.render_widget(detail_block, preview_area);

            let p = Paragraph::new(format!(
                "{}{}",
                ev.title,
                if ev.content.is_empty() { "" } else { "\n\n" }
            ))
            .style(Style::default().fg(Color::Gray));
            f.render_widget(p, d_inner);

            if let Some(c) = ev
                .content
                .get(..d_inner.width as usize * (d_inner.height as usize - 1))
            {
                let cp = Paragraph::new(c.to_string()).style(Style::default().fg(Color::White));
                f.render_widget(
                    cp,
                    Rect {
                        x: d_inner.x,
                        y: d_inner.y + 1,
                        width: d_inner.width,
                        height: d_inner.height.saturating_sub(1),
                    },
                );
            }
        }
    }
}

fn render_form(f: &mut Frame, area: Rect, panel: &mut EvidencePanel) {
    let form = &mut panel.form;

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .split(area);

    let focused = form.focused_field;

    // Type
    render_select_row(
        f,
        layout[0],
        "Type",
        &form.evidence_type.to_string(),
        focused == EvidenceFormField::Type,
    );

    // Title
    render_text_row(
        f,
        layout[1],
        "Title",
        &form.title,
        form.title_cursor,
        focused == EvidenceFormField::Title,
    );

    // Content
    render_text_row(
        f,
        layout[2],
        "Content",
        &form.content,
        form.content_cursor,
        focused == EvidenceFormField::Content,
    );

    // Spacer
    f.render_widget(Paragraph::new(""), layout[3]);

    // Submit hint
    let sub = if matches!(panel.mode, EvidencePanelMode::Add) {
        " [ Save Evidence ] "
    } else {
        " [ Update Evidence ] "
    };
    let style = if focused == EvidenceFormField::Content {
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::UNDERLINED)
    } else {
        Style::default()
    };

    f.render_widget(
        Paragraph::new(sub)
            .style(style)
            .alignment(Alignment::Center),
        layout[4],
    );

    let hint = " Tab:Next  Enter:Submit/Save  Esc:Cancel ";
    f.render_widget(
        Paragraph::new(hint).style(Style::default().fg(Color::Gray)),
        layout[5],
    );
}

fn render_text_row(
    f: &mut Frame,
    area: Rect,
    label: &str,
    value: &str,
    cursor: usize,
    focused: bool,
) {
    let style = if focused {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let label_text = format!("  {:.<12} ", label);
    let label_span = Span::styled(&label_text, style);
    let value_span = Span::styled(value, style);
    f.render_widget(
        Paragraph::new(Line::from(vec![label_span, value_span])),
        area,
    );

    if focused {
        let x = area.x + label_text.len() as u16 + cursor.min(value.len()) as u16;
        f.set_cursor_position((x, area.y));
    }
}

fn render_select_row(f: &mut Frame, area: Rect, label: &str, value: &str, focused: bool) {
    let style = if focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    };

    let label_text = format!("  {:.<12} ", label);
    let value_text = format!("< {} >", value);
    let line = Line::from(vec![
        Span::styled(&label_text, style),
        Span::styled(value_text, style),
    ]);
    f.render_widget(Paragraph::new(line), area);
}
