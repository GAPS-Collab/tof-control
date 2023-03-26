#![allow(unused)]
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

use crate::rb_control::*;


pub fn rb_table() {
    println!("RB Information");
    rb_info_table();
    println!("");
    println!("RB Environmental Status");
    rb_env_table();
    println!("");
    println!("RB Power Status");
    rb_vcp_table();
}

pub fn rb_info_table() {
    let mut table = Table::new();

    let rb_info = rb_info::RBinfo::new();

    table.load_preset(UTF8_FULL)
        .set_header(vec![
            Cell::new("Information").add_attribute(Attribute::Bold),
            Cell::new("Value").add_attribute(Attribute::Bold),
        ])
        .add_row(vec![
            Cell::new("Board ID"),
            Cell::new(&format!("{}", rb_info.board_id)),
        ])
        .add_row(vec![
            Cell::new("FPGA Firmware Version"),
            Cell::new(&format!("{}", rb_info.global_ver)),
        ])
        .add_row(vec![
            Cell::new("FPGA Firmware Hash"),
            Cell::new(&format!("{:02X}", rb_info.global_sha)),
        ])
        .add_row(vec![
            Cell::new("Loss of Lock Status"),
            Cell::new(&format!("{}", rb_info.loss_of_lock)),
        ])
        .add_row(vec![
            Cell::new("Loss of Lock Stability"),
            Cell::new(&format!("{}", rb_info.loss_of_lock_stable)),
        ]);
    
    println!("{table}");

}

pub fn rb_env_table() {
    let mut table = Table::new();

    let rb_temp = rb_temp::RBtemp::new();
    let rb_ph = rb_ph::RBph::new();
    let rb_mag = rb_mag::RBmag::new();

    table.load_preset(UTF8_FULL)
        .set_header(vec![
            Cell::new("Measurement").add_attribute(Attribute::Bold),
            Cell::new("Value").add_attribute(Attribute::Bold),
            Cell::new("Unit").add_attribute(Attribute::Bold),
        ])
        .add_row(vec![
            Cell::new("DRS Temperature"),
            Cell::new(&format!("{:.3}", rb_temp.drs_temp)),
            // temp_color(rb_temp.drs_temp),
            Cell::new("°C"),
        ])
        .add_row(vec![
            Cell::new("CLK Temperature"),
            Cell::new(&format!("{:.3}", rb_temp.clk_temp)),
            Cell::new("°C"),
        ])
        .add_row(vec![
            Cell::new("ADC Temperature"),
            Cell::new(&format!("{:.3}", rb_temp.adc_temp)),
            Cell::new("°C"),
        ])
        .add_row(vec![
            Cell::new("ZYNQ Temperature"),
            Cell::new(&format!("{:.3}", rb_temp.zynq_temp)),
            Cell::new("°C"),
        ])
        .add_row(vec![
            Cell::new("LIS3MDLTR Temperature"),
            Cell::new(&format!("{:.3}", rb_temp.lis3mdltr_temp)),
            Cell::new("°C"),
        ])
        .add_row(vec![
            Cell::new("BME280 Temperature"),
            Cell::new(&format!("{:.3}", rb_temp.bme280_temp)),
            Cell::new("°C"),
        ])
        .add_row(vec![
            Cell::new("Pressure"),
            Cell::new(&format!("{:.3}", rb_ph.pressure)),
            Cell::new("hPa"),
        ])
        .add_row(vec![
            Cell::new("Humidity"),
            Cell::new(&format!("{:.3}", rb_ph.humidity)),
            Cell::new("%"),
        ])
        .add_row(vec![
            Cell::new("Magnetic Field (X)"),
            Cell::new(&format!("{:.3}", rb_mag.magnetic_x)),
            Cell::new("G"),
        ])
        .add_row(vec![
            Cell::new("Magnetic Field (Y)"),
            Cell::new(&format!("{:.3}", rb_mag.magnetic_y)),
            Cell::new("G"),
        ])
        .add_row(vec![
            Cell::new("Magnetic Field (Z)"),
            Cell::new(&format!("{:.3}", rb_mag.magnetic_z)),
            Cell::new("G"),
        ])
        .add_row(vec![
            Cell::new("Magnetic Field (Total)"),
            Cell::new(&format!("{:.3}", rb_mag.magnetic_t)),
            Cell::new("G"),
        ]);

    println!("{table}");
}

fn temp_color(temp: f32) -> comfy_table::Cell {
    let mut cell = Cell::new(&format!("{:.3}", temp));
    if temp > 40.0 {
        cell = cell.fg(Color::Red);
    } else {
        cell = cell.fg(Color::Green);
    }
    cell
}

pub fn rb_vcp_table() {
    let mut table = Table::new();

    let rb_vcp = rb_vcp::RBvcp::new();

    table.load_preset(UTF8_FULL)
        .set_header(vec![
            Cell::new("Measurement").add_attribute(Attribute::Bold),
            Cell::new("Voltage [V]").add_attribute(Attribute::Bold),
            Cell::new("Current [A]").add_attribute(Attribute::Bold),
            Cell::new("Power [W]").add_attribute(Attribute::Bold),
        ])
        .add_row(vec![
            Cell::new("ZYNQ 3.3V Power"),
            Cell::new(&format!("{:.3}", rb_vcp.zynq_voltage)),
            Cell::new(&format!("{:.3}", rb_vcp.zynq_current)),
            Cell::new(&format!("{:.3}", rb_vcp.zynq_power)),
        ])
        .add_row(vec![
            Cell::new("3.3V Power"),
            Cell::new(&format!("{:.3}", rb_vcp.p3v3_voltage)),
            Cell::new(&format!("{:.3}", rb_vcp.p3v3_current)),
            Cell::new(&format!("{:.3}", rb_vcp.p3v3_power)),
        ])
        .add_row(vec![
            Cell::new("3.5V Power"),
            Cell::new(&format!("{:.3}", rb_vcp.p3v5_voltage)),
            Cell::new(&format!("{:.3}", rb_vcp.p3v5_current)),
            Cell::new(&format!("{:.3}", rb_vcp.p3v5_power)),
        ])
        .add_row(vec![
            Cell::new("-1.5V Power"),
            Cell::new(&format!("{:.3}", rb_vcp.n1v5_voltage)),
            Cell::new(&format!("{:.3}", rb_vcp.n1v5_current)),
            Cell::new(&format!("{:.3}", rb_vcp.n1v5_power)),
        ])
        .add_row(vec![
            Cell::new("DRS4 Digital 2.5V Power"),
            Cell::new(&format!("{:.3}", rb_vcp.drs_dvdd_voltage)),
            Cell::new(&format!("{:.3}", rb_vcp.drs_dvdd_current)),
            Cell::new(&format!("{:.3}", rb_vcp.drs_dvdd_power)),
        ])
        .add_row(vec![
            Cell::new("DRS4 Analog 2.5V Power"),
            Cell::new(&format!("{:.3}", rb_vcp.drs_avdd_voltage)),
            Cell::new(&format!("{:.3}", rb_vcp.drs_avdd_current)),
            Cell::new(&format!("{:.3}", rb_vcp.drs_avdd_power)),
        ])
        .add_row(vec![
            Cell::new("ADC Digital 2.5V Power"),
            Cell::new(&format!("{:.3}", rb_vcp.adc_dvdd_voltage)),
            Cell::new(&format!("{:.3}", rb_vcp.adc_dvdd_current)),
            Cell::new(&format!("{:.3}", rb_vcp.adc_dvdd_power)),
        ])
        .add_row(vec![
            Cell::new("ADC Analog 3.0V Power"),
            Cell::new(&format!("{:.3}", rb_vcp.adc_avdd_voltage)),
            Cell::new(&format!("{:.3}", rb_vcp.adc_avdd_current)),
            Cell::new(&format!("{:.3}", rb_vcp.adc_avdd_power)),
        ]);

    println!("{table}");
}