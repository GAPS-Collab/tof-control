use ratatui::{
    prelude::*,
    widgets::*,
};

use crate::tui::rat_app::App;

pub fn draw_ltb_tab(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .direction(Direction::Horizontal)
        .split(area);

    // draw_ltb_info(f, chunks[0]);
    draw_ltb_temp(f, app, chunks[0]);
    draw_ltb_threshold(f, app, chunks[1]);
}

fn draw_ltb_temp(f: &mut Frame, app: &mut App, area: Rect) {
    let ltb_temp = &app.ltb_data.temp;
    let mut ltb_temp_list_items = Vec::<ListItem>::new();
    ltb_temp_list_items.push(ListItem::new(Line::from(Span::styled(format!("Trenz Temp: {:.3}[°C]", ltb_temp.trenz_temp), Style::default()))));
    ltb_temp_list_items.push(ListItem::new(Line::from(Span::styled(format!("Board Temp: {:.3}[°C]", ltb_temp.board_temp), Style::default()))));

    let ltb_temp_list = List::new(ltb_temp_list_items)
    .block(Block::default().borders(Borders::ALL).title(Span::styled("LTB Temp", 
    Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))));

    f.render_widget(ltb_temp_list, area)
}

fn draw_ltb_threshold(f: &mut Frame, app: &mut App, area: Rect) {
    let ltb_threshold = &app.ltb_data.threshold;
    let mut ltb_threshold_list_items = Vec::<ListItem>::new();
    ltb_threshold_list_items.push(ListItem::new(Line::from(Span::styled(format!("Threshold 0: {:.3}[mV]", ltb_threshold.thresh_0), Style::default()))));
    ltb_threshold_list_items.push(ListItem::new(Line::from(Span::styled(format!("Threshold 1: {:.3}[mV]", ltb_threshold.thresh_1), Style::default()))));
    ltb_threshold_list_items.push(ListItem::new(Line::from(Span::styled(format!("Threshold 2: {:.3}[mV]", ltb_threshold.thresh_2), Style::default()))));

    let ltb_threshold_list = List::new(ltb_threshold_list_items)
        .block(Block::default().borders(Borders::ALL).title(Span::styled("LTB Threshold", 
        Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))));

    f.render_widget(ltb_threshold_list, area)
}