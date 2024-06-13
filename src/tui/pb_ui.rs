use ratatui::{
    prelude::*,
    widgets::*,
};

use crate::tui::rat_app::App;

pub fn draw_pb_tab(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .direction(Direction::Horizontal)
        .split(area);

        draw_pb_temp(f, app, chunks[0]);
        draw_pb_vcp(f, app, chunks[1]);
        
}

fn draw_pb_temp(f: &mut Frame, app: &mut App, area: Rect) {
    let pb_temp = &app.pb_data.temp;
    let mut pb_temp_list_items = Vec::<ListItem>::new();
    pb_temp_list_items.push(ListItem::new(Line::from(Span::styled(format!("PDS Temp: {:.3}[°C]", pb_temp.pds_temp), Style::default()))));
    pb_temp_list_items.push(ListItem::new(Line::from(Span::styled(format!("PAS Temp: {:.3}[°C]", pb_temp.pas_temp), Style::default()))));
    pb_temp_list_items.push(ListItem::new(Line::from(Span::styled(format!("NAS Temp: {:.3}[°C]", pb_temp.nas_temp), Style::default()))));
    pb_temp_list_items.push(ListItem::new(Line::from(Span::styled(format!("SHV Temp: {:.3}[°C]", pb_temp.shv_temp), Style::default()))));

    let pb_temp_list = List::new(pb_temp_list_items)
    .block(Block::default().borders(Borders::ALL).title(Span::styled("PB Temp", 
    Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))));

    f.render_widget(pb_temp_list, area)
}

fn draw_pb_vcp(f: &mut Frame, app: &mut App, area: Rect) {
    let pb_vcp = &app.pb_data.vcp;
    let mut pb_vcp_list_items = Vec::<ListItem>::new();
    pb_vcp_list_items.push(ListItem::new(Line::from(Span::styled(format!("PA 3.6V Line:          {:.3}[V] | {:.3}[A] | {:.3}[W]", pb_vcp.p3v6_pa_vcp[0], pb_vcp.p3v6_pa_vcp[1], pb_vcp.p3v6_pa_vcp[2]), Style::default()))));
    pb_vcp_list_items.push(ListItem::new(Line::from(Span::styled(format!("PA -1.6V Line:         {:.2}[V] | {:.3}[A] | {:.3}[W]", pb_vcp.n1v6_pa_vcp[0], pb_vcp.n1v6_pa_vcp[1], pb_vcp.n1v6_pa_vcp[2]), Style::default()))));
    pb_vcp_list_items.push(ListItem::new(Line::from(Span::styled(format!("LTB (Trenz) 3.4V Line: {:.3}[V] | {:.3}[A] | {:.3}[W]", pb_vcp.p3v4f_ltb_vcp[0], pb_vcp.p3v4f_ltb_vcp[1], pb_vcp.p3v4f_ltb_vcp[2]), Style::default()))));
    pb_vcp_list_items.push(ListItem::new(Line::from(Span::styled(format!("LTB 3.4V Line:         {:.3}[V] | {:.3}[A] | {:.3}[W]", pb_vcp.p3v4d_ltb_vcp[0], pb_vcp.p3v4d_ltb_vcp[1], pb_vcp.p3v4d_ltb_vcp[2]), Style::default()))));
    pb_vcp_list_items.push(ListItem::new(Line::from(Span::styled(format!("LTB 3.6V Line:         {:.3}[V] | {:.3}[A] | {:.3}[W]", pb_vcp.p3v6_ltb_vcp[0], pb_vcp.p3v6_ltb_vcp[1], pb_vcp.p3v6_ltb_vcp[2]), Style::default()))));
    pb_vcp_list_items.push(ListItem::new(Line::from(Span::styled(format!("LTB -1.6V Line:        {:.2}[V] | {:.3}[A] | {:.3}[W]", pb_vcp.n1v6_ltb_vcp[0], pb_vcp.n1v6_ltb_vcp[1], pb_vcp.n1v6_ltb_vcp[2]), Style::default()))));

    let pb_vcp_list = List::new(pb_vcp_list_items)
    .block(Block::default().borders(Borders::ALL).title(Span::styled("PB VCP (Voltage, Current & Power)", 
    Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))));

    f.render_widget(pb_vcp_list, area)
}