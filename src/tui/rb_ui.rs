use ratatui::{
    prelude::*,
    widgets::*,
};

use crate::tui::rat_app::App;

pub fn draw_rb_tab(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .direction(Direction::Horizontal)
        .split(area);

    let sub_chunks_1 = Layout::default()
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(chunks[1]);

    let sub_chunks_2 = Layout::default()
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .direction(Direction::Horizontal)
        .split(sub_chunks_1[0]);

    let sub_chunks_3 = Layout::default()
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(sub_chunks_2[1]);

    let sub_chunks_4 = Layout::default()
        .constraints([
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ])
        .split(chunks[0]);


    draw_rb_info(f, app, sub_chunks_4[0]);
    draw_rb_mode(f, app, sub_chunks_4[1]);
    draw_rb_temp(f, app, sub_chunks_2[0]);
    draw_rb_ph(f, app, sub_chunks_3[0]);
    draw_rb_mag(f, app, sub_chunks_3[1]);
    draw_rb_vcp(f, app, sub_chunks_1[1]);
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

fn draw_rb_info(f: &mut Frame, app: &mut App, area: Rect) {
    let rb_info = &app.rb_data.info;

    let mut rb_info_list_items = Vec::<ListItem>::new();
    rb_info_list_items.push(ListItem::new(Line::from(Span::styled(format!("Board ID:                {}", rb_info.board_id), Style::default()))));
    rb_info_list_items.push(ListItem::new(Line::from(Span::styled(format!("Firmware Version:        {}", rb_info.fw_version), Style::default()))));
    rb_info_list_items.push(ListItem::new(Line::from(Span::styled(format!("Firmware Hash:           {}", rb_info.fw_hash), Style::default()))));
    rb_info_list_items.push(ListItem::new(Line::from(Span::styled(format!("Sub Board:               {}", if rb_info.sub_board == 1 { "LTB" } else if rb_info.sub_board == 2 { "PB/Preamp" } else if rb_info.sub_board == 3 {"LTB/PB/Preamp"} else { "NC" }), Style::default()))));
    rb_info_list_items.push(ListItem::new(Line::from(Span::styled(format!("RAT Number:              {}", if let 1..=22 = rb_info.rat_num {rb_info.rat_num.to_string()} else { "Not in RAT".to_string() }), Style::default()))));
    rb_info_list_items.push(ListItem::new(Line::from(Span::styled(format!("RAT Position:            {}", match rb_info.rat_pos { 0 => { "Not Flight RAT" }, 1 => { "CBE" }, 2 => { "UMB" }, 3 => { "COR" }, 4 => { "CBE/COR" }, _ => { "Not in RAT" } }), Style::default()))));
    rb_info_list_items.push(ListItem::new(Line::from(Span::styled(format!("RB Position in RAT:      {}", match rb_info.rb_pos { 1 => { "RB1" }, 2 => { "RB2" }, _ => { "Not in RAT" } }), Style::default()))));
    rb_info_list_items.push(ListItem::new(Line::from(Span::styled(format!("Loss-of-Lock:            {}", if rb_info.lol == 0x01 { "Unlocked" } else { "Locked" }), Style::default()))));
    rb_info_list_items.push(ListItem::new(Line::from(Span::styled(format!("Loss-of-Lock (Stable):   {}", if rb_info.lol_stable == 0x01 { "Unlocked Past Second" } else { "Locked Past Second" }), Style::default()))));
    rb_info_list_items.push(ListItem::new(Line::from(Span::styled(format!("Uptime:                  {}[s]", rb_info.uptime), Style::default()))));
    // rb_info_list_items.push(ListItem::new(Line::from(Span::styled(format!("Input Mode:              {}", rb_info.input_mode), Style::default()))));
    
    let rb_info_list = List::new(rb_info_list_items)
    .block(Block::default().borders(Borders::ALL).title(Span::styled("RB Information", 
    Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))));

    f.render_widget(rb_info_list, area)
}

fn draw_rb_mode(f: &mut Frame, app: &mut App, area: Rect) {
    let rb_mode = &app.rb_data.info.input_mode;

    let mut rb_mode_list_items = Vec::<ListItem>::new();
    rb_mode_list_items.push(ListItem::new(Line::from(Span::styled(format!("Input Mode:              {}", rb_mode), Style::default()))));

    let rb_mode_list = List::new(rb_mode_list_items)
    .block(Block::default().borders(Borders::ALL).title(Span::styled("RB Input Mode", 
    Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))));

    f.render_widget(rb_mode_list, area)
}

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

fn draw_rb_vcp(f: &mut Frame, app: &mut App, area: Rect) {
    let rb_vcp = &app.rb_data.vcp;

    let mut rb_vcp_list_items = Vec::<ListItem>::new();
    rb_vcp_list_items.push(ListItem::new(Line::from(Span::styled(format!("ZYNQ 3.3V Line:           {:.3}[V] | {:.3}[A] | {:.3}[W]", rb_vcp.zynq_vcp[0], rb_vcp.zynq_vcp[1], rb_vcp.zynq_vcp[2]), Style::default()))));
    rb_vcp_list_items.push(ListItem::new(Line::from(Span::styled(format!("RB 3.3V Line:             {:.3}[V] | {:.3}[A] | {:.3}[W]", rb_vcp.p3v3_vcp[0], rb_vcp.p3v3_vcp[1], rb_vcp.p3v3_vcp[2]), Style::default()))));
    rb_vcp_list_items.push(ListItem::new(Line::from(Span::styled(format!("RB 3.5V Line:             {:.3}[V] | {:.3}[A] | {:.3}[W]", rb_vcp.p3v5_vcp[0], rb_vcp.p3v5_vcp[1], rb_vcp.p3v5_vcp[2]), Style::default()))));
    rb_vcp_list_items.push(ListItem::new(Line::from(Span::styled(format!("RB -1.5V Line:            {:.2}[V] | {:.3}[A] | {:.3}[W]", rb_vcp.n1v5_vcp[0], rb_vcp.n1v5_vcp[1], rb_vcp.n1v5_vcp[2]), Style::default()))));
    rb_vcp_list_items.push(ListItem::new(Line::from(Span::styled(format!("DRS 2.5V Digital Line:    {:.3}[V] | {:.3}[A] | {:.3}[W]", rb_vcp.drs_dvdd_vcp[0], rb_vcp.drs_dvdd_vcp[1], rb_vcp.drs_dvdd_vcp[2]), Style::default()))));
    rb_vcp_list_items.push(ListItem::new(Line::from(Span::styled(format!("DRS 2.5V Analog Line:     {:.3}[V] | {:.3}[A] | {:.3}[W]", rb_vcp.drs_avdd_vcp[0], rb_vcp.drs_avdd_vcp[1], rb_vcp.drs_avdd_vcp[2]), Style::default()))));
    rb_vcp_list_items.push(ListItem::new(Line::from(Span::styled(format!("ADC 2.5V Digital Line:    {:.3}[V] | {:.3}[A] | {:.3}[W]", rb_vcp.adc_dvdd_vcp[0], rb_vcp.adc_dvdd_vcp[1], rb_vcp.adc_dvdd_vcp[2]), Style::default()))));
    rb_vcp_list_items.push(ListItem::new(Line::from(Span::styled(format!("ADC 3.0V Analog Line:     {:.3}[V] | {:.3}[A] | {:.3}[W]", rb_vcp.adc_avdd_vcp[0], rb_vcp.adc_avdd_vcp[1], rb_vcp.adc_avdd_vcp[2]), Style::default()))));

    let rb_vcp_list = List::new(rb_vcp_list_items)
    .block(Block::default().borders(Borders::ALL).title(Span::styled("RB VCP (Voltage, Current & Power)", 
    Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))));

    f.render_widget(rb_vcp_list, area)
}

fn draw_rb_ph(f: &mut Frame, app: &mut App, area: Rect) {
    let rb_ph = &app.rb_data.ph;

    let mut rb_ph_list_items = Vec::<ListItem>::new();
    rb_ph_list_items.push(ListItem::new(Line::from(Span::styled(format!("Pressure:      {:.3}[hPa]", rb_ph.pressure), Style::default()))));
    rb_ph_list_items.push(ListItem::new(Line::from(Span::styled(format!("Humidity:      {:.3}[%]", rb_ph.humidity), Style::default()))));
    
    let rb_ph_list = List::new(rb_ph_list_items)
    .block(Block::default().borders(Borders::ALL).title(Span::styled("RB Pressure & Humidity", 
    Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))));

    f.render_widget(rb_ph_list, area)
}

fn draw_rb_mag(f: &mut Frame, app: &mut App, area: Rect) {
    let rb_mag = &app.rb_data.mag;

    let mut rb_mag_list_items = Vec::<ListItem>::new();
    rb_mag_list_items.push(ListItem::new(Line::from(Span::styled(format!("X-Axis:      {:.3}[G]", rb_mag.mag_xyz[0]), Style::default()))));
    rb_mag_list_items.push(ListItem::new(Line::from(Span::styled(format!("Y-Axis:      {:.3}[G]", rb_mag.mag_xyz[1]), Style::default()))));
    rb_mag_list_items.push(ListItem::new(Line::from(Span::styled(format!("Z-Axis:      {:.3}[G]", rb_mag.mag_xyz[2]), Style::default()))));
    
    let rb_mag_list = List::new(rb_mag_list_items)
    .block(Block::default().borders(Borders::ALL).title(Span::styled("RB Magnetometer", 
    Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))));

    f.render_widget(rb_mag_list, area)
}