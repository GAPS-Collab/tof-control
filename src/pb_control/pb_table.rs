use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

use crate::pb_control::*;


pub fn pb_table() {
    println!("PB Environmental Status");
    pb_env_table();
    println!("");
    println!("PB Power Status");
    pb_vcp_table();
}

pub fn pb_env_table() {
    let mut table = Table::new();

    let pb_temp = pb_temp::PBtemp::new();

    table.load_preset(UTF8_FULL)
        .set_header(vec![
            Cell::new("Measurement").add_attribute(Attribute::Bold),
            Cell::new("Value").add_attribute(Attribute::Bold),
            Cell::new("Unit").add_attribute(Attribute::Bold),
        ])
        .add_row(vec![
            Cell::new("3.4V Line Temperature"),
            Cell::new(&format!("{:.3}", pb_temp.pds_temp)),
            Cell::new("°C"),
        ])
        .add_row(vec![
            Cell::new("3.6V Line Temperature"),
            Cell::new(&format!("{:.3}", pb_temp.pds_temp)),
            Cell::new("°C"),
        ])
        .add_row(vec![
            Cell::new("-1.6V Line Temperature"),
            Cell::new(&format!("{:.3}", pb_temp.nas_temp)),
            Cell::new("°C"),
        ])
        .add_row(vec![
            Cell::new("SiPM High Voltage Line Temperature"),
            Cell::new(&format!("{:.3}", pb_temp.shv_temp)),
            Cell::new("°C"),
        ]);

    println!("{table}");
}

pub fn pb_vcp_table() {
    let mut table = Table::new();

    let pb_vcp = pb_vcp::PBvcp::new();

    table.load_preset(UTF8_FULL)
        .set_header(vec![
            Cell::new("Measurement").add_attribute(Attribute::Bold),
            Cell::new("Voltage [V]").add_attribute(Attribute::Bold),
            Cell::new("Current [A]").add_attribute(Attribute::Bold),
            Cell::new("Power [W]").add_attribute(Attribute::Bold),
        ])
        .add_row(vec![
            Cell::new("LTB 3.4V FPGA Power"),
            Cell::new(&format!("{:.3}", pb_vcp.p3v4f_ltb_voltage)),
            Cell::new(&format!("{:.3}", pb_vcp.p3v4f_ltb_current)),
            Cell::new(&format!("{:.3}", pb_vcp.p3v4f_ltb_power)),
        ])
        .add_row(vec![
            Cell::new("LTB 3.4V DAC Power"),
            Cell::new(&format!("{:.3}", pb_vcp.p3v4d_ltb_voltage)),
            Cell::new(&format!("{:.3}", pb_vcp.p3v4d_ltb_current)),
            Cell::new(&format!("{:.3}", pb_vcp.p3v4d_ltb_power)),
        ])
        .add_row(vec![
            Cell::new("LTB 3.6V Power"),
            Cell::new(&format!("{:.3}", pb_vcp.p3v6_ltb_voltage)),
            Cell::new(&format!("{:.3}", pb_vcp.p3v6_ltb_current)),
            Cell::new(&format!("{:.3}", pb_vcp.p3v6_ltb_power)),
        ])
        .add_row(vec![
            Cell::new("LTB -1.6V Power"),
            Cell::new(&format!("{:.3}", pb_vcp.n1v6_ltb_voltage)),
            Cell::new(&format!("{:.3}", pb_vcp.n1v6_ltb_current)),
            Cell::new(&format!("{:.3}", pb_vcp.n1v6_ltb_power)),
        ])
        .add_row(vec![
            Cell::new("Preamp 3.6V Power"),
            Cell::new(&format!("{:.3}", pb_vcp.p3v6_preamp_voltage)),
            Cell::new(&format!("{:.3}", pb_vcp.p3v6_preamp_current)),
            Cell::new(&format!("{:.3}", pb_vcp.p3v6_preamp_power)),
        ])
        .add_row(vec![
            Cell::new("Preamp -1.6V Power"),
            Cell::new(&format!("{:.3}", pb_vcp.n1v6_preamp_voltage)),
            Cell::new(&format!("{:.3}", pb_vcp.n1v6_preamp_current)),
            Cell::new(&format!("{:.3}", pb_vcp.n1v6_preamp_power)),
        ]);

    println!("{table}");
}