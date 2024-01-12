use ratatui::{
    prelude::*,
    widgets::*,
};

use crate::tui::rat_app::App;

pub fn draw_rb_tab(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .constraints([
            Constraint::Length(9),
            Constraint::Min(8),
            Constraint::Length(7),
        ])
        .split(area);

    // draw_rb_info(f, chunks[0]);
    draw_rb_temp(f, app, chunks[0]);
}

// fn draw_rb_info(f: &mut Frame, area: Rect) {
//     let text = vec![
//         text::Line::from("RB Info"),
//         text::Line::from("Text"),
//     ];

//     let block = Block::default().borders(Borders::ALL).title(Span::styled(
//         "RB Info",
//         Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
//     ));

//     let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
//     f.render_widget(paragraph, area);
// }

fn draw_rb_temp(f: &mut Frame, app: &mut App, area: Rect) {
    let rb_temp = &app.rb_data.temp;

    let mut rb_temp_list_items = Vec::<ListItem>::new();
    rb_temp_list_items.push(ListItem::new(Line::from(Span::styled(format!("ZYNQ Temp:      {:.3}[°C]", rb_temp.zynq_temp), Style::default()))));
    rb_temp_list_items.push(ListItem::new(Line::from(Span::styled(format!("DRS Temp:       {:.3}[°C]", rb_temp.drs_temp), Style::default()))));
    rb_temp_list_items.push(ListItem::new(Line::from(Span::styled(format!("CLK Temp:       {:.3}[°C]", rb_temp.clk_temp), Style::default()))));
    rb_temp_list_items.push(ListItem::new(Line::from(Span::styled(format!("ADC Temp:       {:.3}[°C]", rb_temp.adc_temp), Style::default()))));
    rb_temp_list_items.push(ListItem::new(Line::from(Span::styled(format!("BME280 Temp:    {:.3}[°C]", rb_temp.bme280_temp), Style::default()))));
    rb_temp_list_items.push(ListItem::new(Line::from(Span::styled(format!("LIS3MDLTR Temp: {:.3}[°C]", rb_temp.lis3mdltr_temp), Style::default()))));
    
    let rb_temp_list = List::new(rb_temp_list_items)
    .block(Block::default().borders(Borders::ALL).title(Span::styled("RB Temp", 
    Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))));

    f.render_widget(rb_temp_list, area)
}