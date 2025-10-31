use ratatui::{
    prelude::*,
    widgets::*,
};

use crate::tui::rat_app::App;
use crate::tui::rb_ui::*;
use crate::tui::ltb_ui::*;
use crate::tui::pb_ui::*;
use crate::tui::pa_ui::*;

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(4)
        ])
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

    match app.tabs.titles.len() {
        1 => {
            draw_rb_tab(f, app, chunks[1])
        }
        2 => {
            match app.tabs.index {
                0 => draw_rb_tab(f, app, chunks[1]),
                1 => draw_ltb_tab(f, app, chunks[1]),
                _ => {},
            }
        }
        3 => {
            match app.tabs.index {
                0 => draw_rb_tab(f, app, chunks[1]),
                1 => draw_pb_tab(f, app, chunks[1]),
                2 => draw_pa_tab(f, app, chunks[1]),
                _ => {},
            }
        }
        4 => {
            match app.tabs.index {
                0 => draw_rb_tab(f, app, chunks[1]),
                1 => draw_ltb_tab(f, app, chunks[1]),
                2 => draw_pb_tab(f, app, chunks[1]),
                3 => draw_pa_tab(f, app, chunks[1]),
                _ => {},
            }
        }
        _ => {}
    }

    let footer = Paragraph::new(format!(
        "Press `q` to stop running.\n\
        Press `r` to reload the sensor values."
    ))
    .block(
        Block::default()
            .title("Operation")
            .title_alignment(Alignment::Left)
            .borders(Borders::ALL)
    );

    f.render_widget(footer, chunks[2]);

}