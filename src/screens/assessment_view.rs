use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};

use crate::models::workflow::{SectionNode, TaskStatus};
use crate::screens;
use crate::state::{FocusPath, TaskPanelTab, WorkflowState};

pub fn render(f: &mut Frame, area: Rect, state: &mut WorkflowState, assessment_name: &str) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" Assessment: {} ", assessment_name))
        .title_alignment(Alignment::Center);

    let inner = block.inner(area);
    f.render_widget(block, area);

    if state.task_panel.is_some() {
        let mut panel = state.task_panel.take().unwrap();
        let split = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(inner);

        render_tree(f, split[0], state);

        let right_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(1)])
            .split(split[1]);

        let tab_titles = vec![" Evidence ", " Findings ", " Knowledge ", " HTTP "];
        let tab_idx = match panel.active_tab {
            TaskPanelTab::Evidence => 0,
            TaskPanelTab::Findings => 1,
            TaskPanelTab::Knowledge => 2,
            TaskPanelTab::Http => 3,
        };
        let tabs = Tabs::new(tab_titles)
            .select(tab_idx)
            .style(Style::default().fg(Color::Gray))
            .highlight_style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            );
        f.render_widget(tabs, right_layout[0]);

        match panel.active_tab {
            TaskPanelTab::Evidence => {
                screens::evidence_view::render(f, right_layout[1], &mut panel.evidence);
            }
            TaskPanelTab::Findings => {
                screens::findings_view::render(f, right_layout[1], &mut panel.findings);
            }
            TaskPanelTab::Knowledge => {
                if let Some(ref entry) = panel.knowledge_entry {
                    screens::knowledge_view::render(f, right_layout[1], entry);
                } else {
                    let p = Paragraph::new("No knowledge entry for this task.\n\nSelect a different task or methodology.")
                        .style(Style::default().fg(Color::Gray))
                        .alignment(Alignment::Center);
                    f.render_widget(p, right_layout[1]);
                }
            }
            TaskPanelTab::Http => {
                screens::http_view::render(f, right_layout[1], &mut panel.http);
            }
        }

        state.task_panel = Some(panel);
    } else {
        render_tree(f, inner, state);
    }
}

fn render_tree(f: &mut Frame, area: Rect, state: &mut WorkflowState) {
    let list = build_visible(&state.sections);

    if list.is_empty() {
        let p = Paragraph::new("No workflow items.")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);
        f.render_widget(p, area);
        return;
    }

    let visible_rows = area.height as usize;
    if state.scroll_offset > list.len().saturating_sub(visible_rows) {
        state.scroll_offset = list.len().saturating_sub(visible_rows);
    }

    let focused = match state.focus {
        FocusPath::Section(si) => Some(FocusId::Section(si)),
        FocusPath::Activity(si, ai) => Some(FocusId::Activity(si, ai)),
        FocusPath::Task(si, ai, ti) => Some(FocusId::Task(si, ai, ti)),
    };

    let selected_global = list.iter().position(|item| Some(item.id) == focused);

    if let Some(pos) = selected_global {
        if pos < state.scroll_offset {
            state.scroll_offset = pos;
        } else if pos >= state.scroll_offset + visible_rows {
            state.scroll_offset = pos - visible_rows + 1;
        }
    }

    for (i, item) in list
        .iter()
        .skip(state.scroll_offset)
        .take(visible_rows)
        .enumerate()
    {
        let y = area.y + i as u16;
        let row_area = Rect {
            x: area.x,
            y,
            width: area.width,
            height: 1,
        };
        render_item(
            f,
            row_area,
            item,
            Some(item.id) == focused,
            &state.sections,
            &state.evidence_counts,
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FocusId {
    Section(usize),
    Activity(usize, usize),
    Task(usize, usize, usize),
}

struct VisibleItem<'a> {
    id: FocusId,
    indent: u16,
    kind: &'a str,
    label: &'a str,
    status: Option<TaskStatus>,
    task_id: Option<uuid::Uuid>,
}

fn build_visible(sections: &[SectionNode]) -> Vec<VisibleItem<'_>> {
    let mut items = Vec::new();
    for (si, section) in sections.iter().enumerate() {
        items.push(VisibleItem {
            id: FocusId::Section(si),
            indent: 0,
            kind: "sec",
            label: &section.section.title,
            status: None,
            task_id: None,
        });
        if section.expanded {
            for (ai, activity) in section.activities.iter().enumerate() {
                items.push(VisibleItem {
                    id: FocusId::Activity(si, ai),
                    indent: 2,
                    kind: "act",
                    label: &activity.activity.title,
                    status: None,
                    task_id: None,
                });
                if activity.expanded {
                    for (ti, task) in activity.tasks.iter().enumerate() {
                        items.push(VisibleItem {
                            id: FocusId::Task(si, ai, ti),
                            indent: 4,
                            kind: "tsk",
                            label: &task.title,
                            status: Some(task.status),
                            task_id: Some(task.id),
                        });
                    }
                }
            }
        }
    }
    items
}

fn render_item(
    f: &mut Frame,
    area: Rect,
    item: &VisibleItem<'_>,
    selected: bool,
    sections: &[SectionNode],
    evidence_counts: &std::collections::HashMap<uuid::Uuid, usize>,
) {
    let base = if selected {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    let prefix = " ".repeat(item.indent as usize);
    match item.kind {
        "sec" => {
            let (arrow, _) = if let FocusId::Section(si) = item.id {
                if let Some(s) = sections.get(si) {
                    (if s.expanded { "\u{25BE}" } else { "\u{25B8}" }, s.expanded)
                } else {
                    ("\u{25B8}", false)
                }
            } else {
                ("\u{25B8}", false)
            };
            f.render_widget(
                Paragraph::new(Line::from(vec![
                    Span::styled(
                        format!("{}{} ", prefix, arrow),
                        base.add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(item.label, base.add_modifier(Modifier::BOLD)),
                ])),
                area,
            );
        }
        "act" => {
            let (arrow, _) = if let FocusId::Activity(si, ai) = item.id {
                if let Some(s) = sections.get(si) {
                    if let Some(a) = s.activities.get(ai) {
                        (if a.expanded { "\u{25BE}" } else { "\u{25B8}" }, a.expanded)
                    } else {
                        ("\u{25B8}", false)
                    }
                } else {
                    ("\u{25B8}", false)
                }
            } else {
                ("\u{25B8}", false)
            };
            f.render_widget(
                Paragraph::new(Line::from(vec![
                    Span::styled(
                        format!("{}{} ", prefix, arrow),
                        Style::default().fg(Color::Gray),
                    ),
                    Span::styled(item.label, base),
                ])),
                area,
            );
        }
        "tsk" => {
            let status = item.status.unwrap_or(TaskStatus::NotStarted);
            let si = status_icon(status);
            let st = status_style(selected, status);
            let count_text = item
                .task_id
                .and_then(|tid| evidence_counts.get(&tid))
                .map(|c| format!(" [{}]", c))
                .unwrap_or_default();
            f.render_widget(
                Paragraph::new(Line::from(vec![
                    Span::raw(format!("{}  ", prefix)),
                    Span::styled(format!("{} ", si), st),
                    Span::styled(item.label, base),
                    Span::styled(
                        count_text,
                        Style::default().fg(if selected {
                            Color::Black
                        } else {
                            Color::DarkGray
                        }),
                    ),
                ])),
                area,
            );
        }
        _ => {}
    }
}
fn status_icon(status: TaskStatus) -> &'static str {
    match status {
        TaskStatus::NotStarted => "\u{25CB}",
        TaskStatus::InProgress => "\u{25CF}",
        TaskStatus::Completed => "\u{2713}",
        TaskStatus::Skipped => "\u{2298}",
    }
}
fn status_style(selected: bool, status: TaskStatus) -> Style {
    let color = match status {
        TaskStatus::NotStarted => Color::Gray,
        TaskStatus::InProgress => Color::Yellow,
        TaskStatus::Completed => Color::Green,
        TaskStatus::Skipped => Color::DarkGray,
    };
    if selected {
        Style::default().fg(Color::Black).bg(Color::Cyan)
    } else {
        Style::default().fg(color)
    }
}
