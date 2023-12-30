use ratatui::{
    prelude::*,
    widgets::*,
};

use crate::tui::rat_app::App;

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(f.size());

    let titles = app
        .tabs
        .titles
        .iter()
        .map(|t| text::Line::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect();
    
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(app.title))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.index);

    f.render_widget(tabs, chunks[0]);

    let inner = match app.tabs.index {
        0 => Block::default().title("Test 0").borders(Borders::ALL),
        1 => Block::default().title("Test 1").borders(Borders::ALL),
        2 => Block::default().title("Test 2").borders(Borders::ALL),
        3 => Block::default().title("Test 3").borders(Borders::ALL),
        _ => unreachable!(),
    };
    f.render_widget(inner, chunks[1]);
}