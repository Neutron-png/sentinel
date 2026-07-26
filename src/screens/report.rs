use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportFormat {
    Markdown,
    Html,
    Json,
}

impl ReportFormat {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Markdown => "Markdown (.md)",
            Self::Html => "HTML (.html)",
            Self::Json => "JSON (.json)",
        }
    }
    pub fn next(self) -> Self {
        match self {
            Self::Markdown => Self::Html,
            Self::Html => Self::Json,
            Self::Json => Self::Markdown,
        }
    }
    pub fn prev(self) -> Self {
        match self {
            Self::Markdown => Self::Json,
            Self::Html => Self::Markdown,
            Self::Json => Self::Html,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportFocus {
    Format,
    OutputDir,
    Generate,
}

impl ReportFocus {
    pub fn next(self) -> Self {
        match self {
            Self::Format => Self::OutputDir,
            Self::OutputDir => Self::Generate,
            Self::Generate => Self::Format,
        }
    }
    pub fn prev(self) -> Self {
        match self {
            Self::Format => Self::Generate,
            Self::OutputDir => Self::Format,
            Self::Generate => Self::OutputDir,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReportState {
    pub format: ReportFormat,
    pub output_dir: String,
    pub output_dir_cursor: usize,
    pub focus: ReportFocus,
    pub status: Option<String>,
    pub success: bool,
}

impl Default for ReportState {
    fn default() -> Self {
        Self {
            format: ReportFormat::Markdown,
            output_dir: "reports".into(),
            output_dir_cursor: 7,
            focus: ReportFocus::Format,
            status: None,
            success: false,
        }
    }
}

pub fn render(f: &mut Frame, area: Rect, state: &mut ReportState, has_active: bool) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Generate Report ")
        .title_alignment(Alignment::Center);
    let inner = block.inner(area);
    f.render_widget(block, area);

    if !has_active {
        let msg = Paragraph::new(
            "No active assessment.\n\nOpen an assessment first from the Open Assessment screen.",
        )
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
        f.render_widget(msg, inner);
        return;
    }

    if let Some(ref status) = state.status {
        let color = if state.success {
            Color::Green
        } else {
            Color::Red
        };
        let p = Paragraph::new(status.as_str())
            .style(Style::default().fg(color))
            .alignment(Alignment::Center);
        f.render_widget(p, inner);
        return;
    }

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(inner);

    // Format
    let fb = Block::default()
        .borders(Borders::ALL)
        .style(if state.focus == ReportFocus::Format {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default()
        });
    let fi = fb.inner(layout[0]);
    f.render_widget(fb, layout[0]);
    let l = Line::from(vec![
        Span::styled(" Format: ", Style::default().fg(Color::Gray)),
        Span::styled(
            format!("< {} >", state.format.label()),
            Style::default().fg(Color::Yellow),
        ),
    ]);
    f.render_widget(Paragraph::new(l), fi);

    // Output dir
    let ob =
        Block::default()
            .borders(Borders::ALL)
            .style(if state.focus == ReportFocus::OutputDir {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default()
            });
    let oi = ob.inner(layout[1]);
    f.render_widget(ob, layout[1]);
    let ol = Line::from(vec![
        Span::styled(" Output:  ", Style::default().fg(Color::Gray)),
        Span::styled(
            &state.output_dir,
            if state.focus == ReportFocus::OutputDir {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            },
        ),
    ]);
    f.render_widget(Paragraph::new(ol), oi);
    if state.focus == ReportFocus::OutputDir {
        f.set_cursor_position((
            oi.x + 9 + state.output_dir_cursor.min(state.output_dir.len()) as u16,
            oi.y,
        ));
    }

    // Generate button
    let gb =
        Block::default()
            .borders(Borders::ALL)
            .style(if state.focus == ReportFocus::Generate {
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            });
    let gi = gb.inner(layout[2]);
    f.render_widget(gb, layout[2]);
    let gl = Line::from(Span::styled(
        "  [ Generate Report ]  ",
        if state.focus == ReportFocus::Generate {
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        },
    ));
    f.render_widget(Paragraph::new(gl).alignment(Alignment::Center), gi);

    // Hint
    let hint = Paragraph::new(" Tab:Next | Enter:Select/Generate | Esc:Back ")
        .style(Style::default().fg(Color::Gray));
    f.render_widget(hint, layout[3]);
}
