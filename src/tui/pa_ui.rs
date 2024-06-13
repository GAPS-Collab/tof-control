use ratatui::{
    prelude::*,
    widgets::*,
};

use crate::tui::rat_app::App;

pub fn draw_pa_tab(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .direction(Direction::Horizontal)
        .split(area);

        draw_pa_temp(f, app, chunks[0]);
        draw_pa_bias(f, app, chunks[1]);
}

fn draw_pa_temp(f: &mut Frame, app: &mut App, area: Rect) {
    let pa_temp = &app.pa_data.temp.pa_temps;
    let mut pa_temp_list_items = Vec::<ListItem>::new();
    pa_temp_list_items.push(ListItem::new(Line::from(Span::styled(format!("Preamp 1 Temp:  {:.3}[°C]", pa_temp[0]), Style::default()))));
    pa_temp_list_items.push(ListItem::new(Line::from(Span::styled(format!("Preamp 2 Temp:  {:.3}[°C]", pa_temp[1]), Style::default()))));
    pa_temp_list_items.push(ListItem::new(Line::from(Span::styled(format!("Preamp 3 Temp:  {:.3}[°C]", pa_temp[2]), Style::default()))));
    pa_temp_list_items.push(ListItem::new(Line::from(Span::styled(format!("Preamp 4 Temp:  {:.3}[°C]", pa_temp[3]), Style::default()))));
    pa_temp_list_items.push(ListItem::new(Line::from(Span::styled(format!("Preamp 5 Temp:  {:.3}[°C]", pa_temp[4]), Style::default()))));
    pa_temp_list_items.push(ListItem::new(Line::from(Span::styled(format!("Preamp 6 Temp:  {:.3}[°C]", pa_temp[5]), Style::default()))));
    pa_temp_list_items.push(ListItem::new(Line::from(Span::styled(format!("Preamp 7 Temp:  {:.3}[°C]", pa_temp[6]), Style::default()))));
    pa_temp_list_items.push(ListItem::new(Line::from(Span::styled(format!("Preamp 8 Temp:  {:.3}[°C]", pa_temp[7]), Style::default()))));
    pa_temp_list_items.push(ListItem::new(Line::from(Span::styled(format!("Preamp 9 Temp:  {:.3}[°C]", pa_temp[8]), Style::default()))));
    pa_temp_list_items.push(ListItem::new(Line::from(Span::styled(format!("Preamp 10 Temp: {:.3}[°C]", pa_temp[9]), Style::default()))));
    pa_temp_list_items.push(ListItem::new(Line::from(Span::styled(format!("Preamp 11 Temp: {:.3}[°C]", pa_temp[10]), Style::default()))));
    pa_temp_list_items.push(ListItem::new(Line::from(Span::styled(format!("Preamp 12 Temp: {:.3}[°C]", pa_temp[11]), Style::default()))));
    pa_temp_list_items.push(ListItem::new(Line::from(Span::styled(format!("Preamp 13 Temp: {:.3}[°C]", pa_temp[12]), Style::default()))));
    pa_temp_list_items.push(ListItem::new(Line::from(Span::styled(format!("Preamp 14 Temp: {:.3}[°C]", pa_temp[13]), Style::default()))));
    pa_temp_list_items.push(ListItem::new(Line::from(Span::styled(format!("Preamp 15 Temp: {:.3}[°C]", pa_temp[14]), Style::default()))));
    pa_temp_list_items.push(ListItem::new(Line::from(Span::styled(format!("Preamp 16 Temp: {:.3}[°C]", pa_temp[15]), Style::default()))));

    let pa_temp_list = List::new(pa_temp_list_items)
    .block(Block::default().borders(Borders::ALL).title(Span::styled("Preamp Temp", 
    Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))));

    f.render_widget(pa_temp_list, area)
}

fn draw_pa_bias(f: &mut Frame, app: &mut App, area: Rect) {
    let pa_bias = &app.pa_data.bias.read_biases;
    let mut pa_bias_list_items = Vec::<ListItem>::new();
    pa_bias_list_items.push(ListItem::new(Line::from(Span::styled(format!("Preamp 1 SiPM Bias:  {:.3}[V]", pa_bias[0]), Style::default()))));
    pa_bias_list_items.push(ListItem::new(Line::from(Span::styled(format!("Preamp 2 SiPM Bias:  {:.3}[V]", pa_bias[1]), Style::default()))));
    pa_bias_list_items.push(ListItem::new(Line::from(Span::styled(format!("Preamp 3 SiPM Bias:  {:.3}[V]", pa_bias[2]), Style::default()))));
    pa_bias_list_items.push(ListItem::new(Line::from(Span::styled(format!("Preamp 4 SiPM Bias:  {:.3}[V]", pa_bias[3]), Style::default()))));
    pa_bias_list_items.push(ListItem::new(Line::from(Span::styled(format!("Preamp 5 SiPM Bias:  {:.3}[V]", pa_bias[4]), Style::default()))));
    pa_bias_list_items.push(ListItem::new(Line::from(Span::styled(format!("Preamp 6 SiPM Bias:  {:.3}[V]", pa_bias[5]), Style::default()))));
    pa_bias_list_items.push(ListItem::new(Line::from(Span::styled(format!("Preamp 7 SiPM Bias:  {:.3}[V]", pa_bias[6]), Style::default()))));
    pa_bias_list_items.push(ListItem::new(Line::from(Span::styled(format!("Preamp 8 SiPM Bias:  {:.3}[V]", pa_bias[7]), Style::default()))));
    pa_bias_list_items.push(ListItem::new(Line::from(Span::styled(format!("Preamp 9 SiPM Bias:  {:.3}[V]", pa_bias[8]), Style::default()))));
    pa_bias_list_items.push(ListItem::new(Line::from(Span::styled(format!("Preamp 10 SiPM Bias: {:.3}[V]", pa_bias[9]), Style::default()))));
    pa_bias_list_items.push(ListItem::new(Line::from(Span::styled(format!("Preamp 11 SiPM Bias: {:.3}[V]", pa_bias[10]), Style::default()))));
    pa_bias_list_items.push(ListItem::new(Line::from(Span::styled(format!("Preamp 12 SiPM Bias: {:.3}[V]", pa_bias[11]), Style::default()))));
    pa_bias_list_items.push(ListItem::new(Line::from(Span::styled(format!("Preamp 13 SiPM Bias: {:.3}[V]", pa_bias[12]), Style::default()))));
    pa_bias_list_items.push(ListItem::new(Line::from(Span::styled(format!("Preamp 14 SiPM Bias: {:.3}[V]", pa_bias[13]), Style::default()))));
    pa_bias_list_items.push(ListItem::new(Line::from(Span::styled(format!("Preamp 15 SiPM Bias: {:.3}[V]", pa_bias[14]), Style::default()))));
    pa_bias_list_items.push(ListItem::new(Line::from(Span::styled(format!("Preamp 16 SiPM Bias: {:.3}[V]", pa_bias[15]), Style::default()))));

    let pa_bias_list = List::new(pa_bias_list_items)
    .block(Block::default().borders(Borders::ALL).title(Span::styled("Preamp SiPM Bias", 
    Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))));

    f.render_widget(pa_bias_list, area)
}