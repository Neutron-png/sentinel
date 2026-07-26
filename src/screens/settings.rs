use ratatui::{
    layout::{Alignment, Rect},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, area: Rect, plugin_count: usize) {
    let content = format!(
        "Sentinel Core v{}\n\n\
        Database: sentinel.db\n\
        Plugins: {}\n\n\
        API Version: {}\n\n\
        Navigation:\n\
        1-6 Navigate | q Quit",
        env!("CARGO_PKG_VERSION"),
        if plugin_count > 0 {
            format!("{} discovered", plugin_count)
        } else {
            "none".into()
        },
        crate::plugins::API_VERSION,
    );

    let paragraph = Paragraph::new(content)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Settings ")
                .title_alignment(Alignment::Center),
        )
        .alignment(Alignment::Center);

    f.render_widget(paragraph, area);
}
