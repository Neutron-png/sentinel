use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportMode {
    Menu,
    Export,
    Import,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFocus {
    ExportBtn,
    ImportBtn,
    FilePath,
}

#[allow(dead_code)]
impl ExportFocus {
    pub fn next(self) -> Self {
        match self {
            Self::ExportBtn => Self::ImportBtn,
            Self::ImportBtn => Self::FilePath,
            Self::FilePath => Self::ExportBtn,
        }
    }
    pub fn prev(self) -> Self {
        match self {
            Self::ExportBtn => Self::FilePath,
            Self::ImportBtn => Self::ExportBtn,
            Self::FilePath => Self::ImportBtn,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExportImportState {
    pub mode: ExportMode,
    pub file_path: String,
    pub file_path_cursor: usize,
    pub focus: ExportFocus,
    pub status: Option<(String, bool)>,
}

impl Default for ExportImportState {
    fn default() -> Self {
        Self {
            mode: ExportMode::Menu,
            file_path: "sentinel_export.sentinel".into(),
            file_path_cursor: 0,
            focus: ExportFocus::ExportBtn,
            status: None,
        }
    }
}

pub fn render(f: &mut Frame, area: Rect, state: &mut ExportImportState, has_active: bool) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Import / Export ")
        .title_alignment(Alignment::Center);
    let inner = block.inner(area);
    f.render_widget(block, area);

    if !has_active {
        let msg = Paragraph::new("No active assessment.\n\nOpen an assessment first.")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);
        f.render_widget(msg, inner);
        return;
    }

    if let Some((ref status, success)) = state.status {
        let c = if success { Color::Green } else { Color::Red };
        f.render_widget(
            Paragraph::new(status.as_str())
                .style(Style::default().fg(c))
                .alignment(Alignment::Center),
            inner,
        );
        return;
    }

    match state.mode {
        ExportMode::Menu => render_menu(f, inner, state),
        ExportMode::Export | ExportMode::Import => render_file_input(f, inner, state),
    }
}

fn render_menu(f: &mut Frame, area: Rect, state: &ExportImportState) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(area);

    let es = if state.focus == ExportFocus::ExportBtn {
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    let is = if state.focus == ExportFocus::ImportBtn {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let eb = Block::default().borders(Borders::ALL).style(es);
    f.render_widget(eb, layout[0]);
    f.render_widget(
        Paragraph::new(" [ Export Assessment ] ")
            .style(es)
            .alignment(Alignment::Center),
        Block::default().borders(Borders::ALL).inner(layout[0]),
    );

    let ib = Block::default().borders(Borders::ALL).style(is);
    f.render_widget(ib, layout[1]);
    f.render_widget(
        Paragraph::new(" [ Import Project ] ")
            .style(is)
            .alignment(Alignment::Center),
        Block::default().borders(Borders::ALL).inner(layout[1]),
    );

    let hint = Paragraph::new(" Up/Down:Select  Enter:Confirm  Esc:Back ")
        .style(Style::default().fg(Color::Gray));
    f.render_widget(hint, layout[2]);
}

fn render_file_input(f: &mut Frame, area: Rect, state: &mut ExportImportState) {
    let label = if state.mode == ExportMode::Export {
        "Export Path"
    } else {
        "Import Path"
    };
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(area);

    let fb =
        Block::default()
            .borders(Borders::ALL)
            .style(if state.focus == ExportFocus::FilePath {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            });
    let fi = fb.inner(layout[0]);
    f.render_widget(fb, layout[0]);
    let l = Line::from(vec![
        Span::styled(format!(" {}: ", label), Style::default().fg(Color::Gray)),
        Span::styled(
            &state.file_path,
            if state.focus == ExportFocus::FilePath {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            },
        ),
    ]);
    f.render_widget(Paragraph::new(l), fi);
    if state.focus == ExportFocus::FilePath {
        f.set_cursor_position((
            fi.x + 15 + state.file_path_cursor.min(state.file_path.len()) as u16,
            fi.y,
        ));
    }

    let btn_text = if state.mode == ExportMode::Export {
        " [ Export ] "
    } else {
        " [ Import ] "
    };
    let gb = Block::default().borders(Borders::ALL).style(
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    );
    f.render_widget(gb, layout[1]);
    f.render_widget(
        Paragraph::new(btn_text)
            .style(
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center),
        Block::default().borders(Borders::ALL).inner(layout[1]),
    );

    let hint =
        Paragraph::new(" Enter:Execute  Esc:Cancel ").style(Style::default().fg(Color::Gray));
    f.render_widget(hint, layout[2]);
}
