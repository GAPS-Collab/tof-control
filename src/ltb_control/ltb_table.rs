use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

use crate::ltb_control::*;

pub fn ltb_table() {
    println!("LTB Environmental Status");
    ltb_env_table();
    println!("");
    println!("LTB Threshold Status");
    ltb_dac_table();
}

pub fn ltb_env_table() {
    let mut table = Table::new();

    let ltb_temp = ltb_temp::LTBtemp::new();

    table.load_preset(UTF8_FULL)
        .set_header(vec![
            Cell::new("Measurement").add_attribute(Attribute::Bold),
            Cell::new("Value").add_attribute(Attribute::Bold),
            Cell::new("Unit").add_attribute(Attribute::Bold),
        ])
        .add_row(vec![
            Cell::new("LTB Temperature"),
            Cell::new(&format!("{:.3}", ltb_temp.ltb_temp)),
            Cell::new("°C")
        ])
        .add_row(vec![
            Cell::new("Trenz Temperature"),
            Cell::new(&format!("{:.3}", ltb_temp.trenz_temp)),
            Cell::new("°C")
        ]);

    println!("{table}");
}

pub fn ltb_dac_table() {
    let mut table = Table::new();

    let ltb_threshold = ltb_dac::LTBdac::new();

    table.load_preset(UTF8_FULL)
        .set_header(vec![
            Cell::new("Threshold Channel").add_attribute(Attribute::Bold),
            Cell::new("Value").add_attribute(Attribute::Bold),
            Cell::new("Unit").add_attribute(Attribute::Bold),
        ])
        .add_row(vec![
            Cell::new("Threshold 0"),
            Cell::new(&format!("{:.3}", ltb_threshold.dac0)),
            Cell::new("mV")
        ])
        .add_row(vec![
            Cell::new("Threshold 1"),
            Cell::new(&format!("{:.3}", ltb_threshold.dac1)),
            Cell::new("mV")
        ])
        .add_row(vec![
            Cell::new("Threshold 2"),
            Cell::new(&format!("{:.3}", ltb_threshold.dac2)),
            Cell::new("mV")
        ]);

    println!("{table}");
}