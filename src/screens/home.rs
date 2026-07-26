use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::models::assessment::Assessment;

pub fn render(
    f: &mut Frame,
    area: Rect,
    assessments: &[Assessment],
    active_assessment: &Option<Assessment>,
    show_resume_prompt: bool,
    session_name: Option<&str>,
) {
    let count = assessments.len();
    let active_line = match active_assessment {
        Some(a) => format!("Active Assessment: {} ({})", a.name, a.status),
        None => String::new(),
    };

    if show_resume_prompt {
        let v = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(6)])
            .split(area);
        let content = format!(
            "Sentinel\nWeb Application Security Assessment\n\n\
            [1] Home\n\
            [2] Create Assessment\n\
            [3] Open Assessment\n\
            [4] Settings\n\
            [5] Generate Report\n\n\
            Assessments: {}\n\
            {}\n\n\
            Press 1-5 to navigate or q to quit.",
            count, active_line
        );
        let paragraph = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Home ")
                    .title_alignment(Alignment::Center),
            )
            .alignment(Alignment::Center);
        f.render_widget(paragraph, v[0]);

        let name = session_name.unwrap_or("previous session");
        let prompt = format!(
            " Resume {}? \n\n [Y] Yes, resume     [N] No, start fresh ",
            name
        );
        let p = Paragraph::new(prompt)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Yellow))
                    .title(" Session "),
            )
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center);
        f.render_widget(p, v[1]);
    } else {
        let content = format!(
            "Sentinel\nWeb Application Security Assessment\n\n\
            [1] Home\n\
            [2] Create Assessment\n\
            [3] Open Assessment\n\
            [4] Settings\n\
            [5] Generate Report\n\n\
            Assessments: {}\n\
            {}\n\n\
            Press 1-5 to navigate or q to quit.",
            count, active_line
        );
        let paragraph = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Home ")
                    .title_alignment(Alignment::Center),
            )
            .alignment(Alignment::Center);
        f.render_widget(paragraph, area);
    }
}
