use std::{error::Error, fs::File, path::Path};

use clap::Parser;
use openssh::{Session, KnownHosts};
use serde::{Deserialize, Serialize};
use chrono::{Utc, DateTime};
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

use tof_control::helper::{
    rb_type::{RBInfoDebug, RBTempDebug, RBVcp, RBPh, RBMag},
    ltb_type::{LTBTemp, LTBThreshold},
    pb_type::{PBTemp, PBVcp},
    preamp_type::{PreampTemp, PreampReadBias},
    cpu_type::{CPUInfoDebug, CPUTempDebug},
};

const ST_DIR: &str = "/home/gaps/test/stress_testing/";

#[derive(Parser, Debug)]
#[command(author = "Takeru Hayashi", version, about, long_about = None)]
struct Cli {
    #[clap(short, long, help = "Take data from all RATs")]
    all: bool,
    #[clap(short, long, help = "Take data from specific RAT")]
    rat: Option<u8>,
    #[clap(short, long, help = "Take data from System Box")]
    sys: bool,
}

#[tokio::main]
async fn main() {

    let dt = Utc::now();

    let cli = Cli::parse();

    if cli.all {
        get_sys_json(dt, true);
        get_rat_json_all(dt).await;

        std::process::exit(0);
    }

    if cli.sys {
        get_sys_json(dt, false);

        std::process::exit(0);
    }

    if let Some(rat) = cli.rat {
        if rat > 22 {
            println!("RAT need to be between 1 to 20 (21 and 22 for UCLA Test Stand).");
            std::process::exit(1);
        }

        get_rat_json(rat, dt, false).await;
    }
}

async fn get_rat_json_all(dt: DateTime<Utc>) {
    let rats: [u8; 20] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    // let rats: [u8; 2] = [21, 22];

    for rat in rats {
        get_rat_json(rat, dt, true).await;
    }
}

async fn get_rat_json(rat: u8, dt: DateTime<Utc>, all: bool) {
    let rbs = match_rb_from_rat(rat);

    let rb1 = format!("gaps@tof-rb{:02}", rbs[0]);
    let rb2 = format!("gaps@tof-rb{:02}", rbs[1]);

    let session_rb1 = Session::connect(rb1, KnownHosts::Strict).await.unwrap();
    let session_rb2 = Session::connect(rb2, KnownHosts::Strict).await.unwrap();

    let json_rb1 = session_rb1.command("rat-control").arg("--json").output().await.unwrap();
    let json_rb2 = session_rb2.command("rat-control").arg("--json").output().await.unwrap();

    let json_rb1_str = String::from_utf8(json_rb1.stdout).unwrap();
    let json_rb2_str = String::from_utf8(json_rb2.stdout).unwrap();

    let json_rb1_deserialized: RATJsonRB1 = serde_json::from_str(&json_rb1_str).unwrap();
    let json_rb2_deserialized: RATJsonRB2 = serde_json::from_str(&json_rb2_str).unwrap();

    // let dt = Utc::now();
    let dt_fmt = format!("{:?}", &dt);
    let dt_json = dt.format("%Y%m%d%H%M%S").to_string();

    let rat_struct = RATJson {
        rat: rat,
        dt: dt_fmt,
        rb1: json_rb1_deserialized.rb,
        rb2: json_rb2_deserialized.rb,
        ltb: json_rb1_deserialized.ltb,
        pb: json_rb2_deserialized.pb,
        preamp: json_rb2_deserialized.preamp,
    };

    // Save Json to File
    let mut json_file = Default::default();
    if all {
        json_file = format!("rat{}_{}_all.json", rat, dt_json);
    } else {
        json_file = format!("rat{}_{}.json", rat, dt_json);
    }

    let json_file_path = ST_DIR.to_owned() + &json_file.to_owned();
    match save_rat_json_file(json_file_path, &rat_struct) {
        Ok(_) => {
            println!("Saved json file for RAT{}.\n", rat);
        }
        Err(err) => {
            eprintln!("Failed to save json file for RAT{}: {}", rat, err);
        }
    }

    // Print Json if RAT is specified
    if !all {
        print_rat_json(&rat_struct);
    }

    // let json_rat = serde_json::to_string_pretty(&rat_struct).unwrap();

    // println!("{}", json_rat);

}


fn save_rat_json_file<P: AsRef<Path>>(path: P, rat: &RATJson) -> Result<(), Box<dyn Error>> {
    let file = File::create(path)?;

    serde_json::to_writer_pretty(file, rat)?;

    Ok(())
}

fn save_sys_json_file<P: AsRef<Path>>(path: P, sys: &SYSJson) -> Result<(), Box<dyn Error>> {
    let file = File::create(path)?;

    serde_json::to_writer_pretty(file, sys)?;

    Ok(())
}

fn print_rat_json(rat: &RATJson) {
    let mut rb_table = Table::new();
    let mut ltb_table = Table::new();
    let mut pb_table = Table::new();
    let mut preamp_table = Table::new();

    rb_table.load_preset(UTF8_FULL)
        .set_header(vec![
            Cell::new("Key").add_attribute(Attribute::Bold),
            Cell::new("RB1 Value").add_attribute(Attribute::Bold),
            Cell::new("RB2 Value").add_attribute(Attribute::Bold),
        ])
        .add_row(vec![
            Cell::new("Datetime"),
            Cell::new(&format!("{}", rat.dt)),
            Cell::new(&format!("{}", rat.dt)),
        ])
        .add_row(vec![
            Cell::new("RB ID"),
            Cell::new(&format!("{}", rat.rb1.rb_info.board_id)),
            Cell::new(&format!("{}", rat.rb2.rb_info.board_id)),
        ])
        .add_row(vec![
            Cell::new("RB Position in RAT"),
            Cell::new(&format!("{}", rat.rb1.rb_info.rb_pos)),
            Cell::new(&format!("{}", rat.rb2.rb_info.rb_pos)),
        ])
        .add_row(vec![
            Cell::new("RAT Location"),
            Cell::new(&format!("{}", if rat.rb1.rb_info.rat_pos == 0 { "Not Flight RAT" } else if rat.rb1.rb_info.rat_pos == 1 { "CBE" } else if rat.rb1.rb_info.rat_pos == 2 { "UMB" } else if rat.rb1.rb_info.rat_pos == 3 { "COR" } else if rat.rb1.rb_info.rat_pos == 4 { "CBE/COR" } else { "N/A" })),
            Cell::new(&format!("{}", if rat.rb2.rb_info.rat_pos == 0 { "Not Flight RAT" } else if rat.rb2.rb_info.rat_pos == 1 { "CBE" } else if rat.rb2.rb_info.rat_pos == 2 { "UMB" } else if rat.rb2.rb_info.rat_pos == 3 { "COR" } else if rat.rb2.rb_info.rat_pos == 4 { "CBE/COR" } else { "N/A" })),
        ])
        .add_row(vec![
            Cell::new("Connected Boards"),
            Cell::new(&format!("{}", if rat.rb1.rb_info.sub_board == 1 { "LTB" } else if rat.rb1.rb_info.sub_board == 2 { "PB/Preamp" } else { "NC" })),
            Cell::new(&format!("{}", if rat.rb2.rb_info.sub_board == 1 { "LTB" } else if rat.rb2.rb_info.sub_board == 2 { "PB/Preamp" } else { "NC" })),
        ])
        .add_row(vec![
            Cell::new("Loss of Lock"),
            Cell::new(&format!("{}", if rat.rb1.rb_info.lol == 0x01 { "Unlocked" } else { "Locked" })),
            Cell::new(&format!("{}", if rat.rb2.rb_info.lol == 0x01 { "Unlocked" } else { "Locked" })),
        ])
        .add_row(vec![
            Cell::new("Loss of Lock Stable"),
            Cell::new(&format!("{}", if rat.rb1.rb_info.lol_stable == 0x01 { "Unlocked Past Second" } else { "Locked Past Second" })),
            Cell::new(&format!("{}", if rat.rb2.rb_info.lol_stable == 0x01 { "Unlocked Past Second" } else { "Locked Past Second" })),
        ])
        .add_row(vec![
            Cell::new("Firmware Version"),
            Cell::new(&format!("{}", rat.rb1.rb_info.fw_version)),
            Cell::new(&format!("{}", rat.rb2.rb_info.fw_version)),
        ])
        .add_row(vec![
            Cell::new("Firmware Hash"),
            Cell::new(&format!("{}", rat.rb1.rb_info.fw_hash)),
            Cell::new(&format!("{}", rat.rb2.rb_info.fw_hash)),
        ])
        .add_row(vec![
            Cell::new("Uptime"),
            Cell::new(&format!("{}[s]", rat.rb1.rb_info.uptime)),
            Cell::new(&format!("{}[s]", rat.rb2.rb_info.uptime)),
        ])
        .add_row(vec![
            Cell::new("Input Mode"),
            Cell::new(&format!("{}", rat.rb1.rb_info.input_mode)),
            Cell::new(&format!("{}", rat.rb2.rb_info.input_mode)),
        ])
        .add_row(vec![
            Cell::new("ZYNQ Temperature"),
            Cell::new(&format!("{:.3}[°C]", rat.rb1.rb_temp.zynq_temp)),
            Cell::new(&format!("{:.3}[°C]", rat.rb2.rb_temp.zynq_temp)),
        ])
        .add_row(vec![
            Cell::new("DRS Temperature"),
            Cell::new(&format!("{:.3}[°C]", rat.rb1.rb_temp.drs_temp)),
            Cell::new(&format!("{:.3}[°C]", rat.rb2.rb_temp.drs_temp)),
        ])
        .add_row(vec![
            Cell::new("CLK Temperature"),
            Cell::new(&format!("{:.3}[°C]", rat.rb1.rb_temp.clk_temp)),
            Cell::new(&format!("{:.3}[°C]", rat.rb2.rb_temp.clk_temp)),
        ])
        .add_row(vec![
            Cell::new("ADC Temperature"),
            Cell::new(&format!("{:.3}[°C]", rat.rb1.rb_temp.adc_temp)),
            Cell::new(&format!("{:.3}[°C]", rat.rb2.rb_temp.adc_temp)),
        ])
        .add_row(vec![
            Cell::new("BME280 Temperature"),
            Cell::new(&format!("{:.3}[°C]", rat.rb1.rb_temp.bme280_temp)),
            Cell::new(&format!("{:.3}[°C]", rat.rb2.rb_temp.bme280_temp)),
        ])
        .add_row(vec![
            Cell::new("LIS3MDLTR Temperature"),
            Cell::new(&format!("{:.3}[°C]", rat.rb1.rb_temp.lis3mdltr_temp)),
            Cell::new(&format!("{:.3}[°C]", rat.rb2.rb_temp.lis3mdltr_temp)),
        ])
        .add_row(vec![
            Cell::new("ZYNQ Power VCP"),
            Cell::new(&format!("{:.3}[V] | {:.3}[A] | {:.3}[W]", rat.rb1.rb_vcp.zynq_vcp[0], rat.rb1.rb_vcp.zynq_vcp[1], rat.rb1.rb_vcp.zynq_vcp[2])),
            Cell::new(&format!("{:.3}[V] | {:.3}[A] | {:.3}[W]", rat.rb2.rb_vcp.zynq_vcp[0], rat.rb2.rb_vcp.zynq_vcp[1], rat.rb2.rb_vcp.zynq_vcp[2])),
        ])
        .add_row(vec![
            Cell::new("3.3V Power VCP"),
            Cell::new(&format!("{:.3}[V] | {:.3}[A] | {:.3}[W]", rat.rb1.rb_vcp.p3v3_vcp[0], rat.rb1.rb_vcp.p3v3_vcp[1], rat.rb1.rb_vcp.p3v3_vcp[2])),
            Cell::new(&format!("{:.3}[V] | {:.3}[A] | {:.3}[W]", rat.rb2.rb_vcp.p3v3_vcp[0], rat.rb2.rb_vcp.p3v3_vcp[1], rat.rb2.rb_vcp.p3v3_vcp[2])),
        ])
        .add_row(vec![
            Cell::new("3.5V Power VCP"),
            Cell::new(&format!("{:.3}[V] | {:.3}[A] | {:.3}[W]", rat.rb1.rb_vcp.p3v5_vcp[0], rat.rb1.rb_vcp.p3v5_vcp[1], rat.rb1.rb_vcp.p3v5_vcp[2])),
            Cell::new(&format!("{:.3}[V] | {:.3}[A] | {:.3}[W]", rat.rb2.rb_vcp.p3v5_vcp[0], rat.rb2.rb_vcp.p3v5_vcp[1], rat.rb2.rb_vcp.p3v5_vcp[2])),
        ])
        .add_row(vec![
            Cell::new("-1.5V Power VCP"),
            Cell::new(&format!("{:.2}[V] | {:.3}[A] | {:.3}[W]", rat.rb1.rb_vcp.n1v5_vcp[0], rat.rb1.rb_vcp.n1v5_vcp[1], rat.rb1.rb_vcp.n1v5_vcp[2])),
            Cell::new(&format!("{:.2}[V] | {:.3}[A] | {:.3}[W]", rat.rb2.rb_vcp.n1v5_vcp[0], rat.rb2.rb_vcp.n1v5_vcp[1], rat.rb2.rb_vcp.n1v5_vcp[2])),
        ])
        .add_row(vec![
            Cell::new("DRS Digital Power (2.5V) VCP"),
            Cell::new(&format!("{:.3}[V] | {:.3}[A] | {:.3}[W]", rat.rb1.rb_vcp.drs_dvdd_vcp[0], rat.rb1.rb_vcp.drs_dvdd_vcp[1], rat.rb1.rb_vcp.drs_dvdd_vcp[2])),
            Cell::new(&format!("{:.3}[V] | {:.3}[A] | {:.3}[W]", rat.rb2.rb_vcp.drs_dvdd_vcp[0], rat.rb2.rb_vcp.drs_dvdd_vcp[1], rat.rb2.rb_vcp.drs_dvdd_vcp[2])),
        ])
        .add_row(vec![
            Cell::new("DRS Analog Power (2.5V) VCP"),
            Cell::new(&format!("{:.3}[V] | {:.3}[A] | {:.3}[W]", rat.rb1.rb_vcp.drs_avdd_vcp[0], rat.rb1.rb_vcp.drs_avdd_vcp[1], rat.rb1.rb_vcp.drs_avdd_vcp[2])),
            Cell::new(&format!("{:.3}[V] | {:.3}[A] | {:.3}[W]", rat.rb2.rb_vcp.drs_avdd_vcp[0], rat.rb2.rb_vcp.drs_avdd_vcp[1], rat.rb2.rb_vcp.drs_avdd_vcp[2])),
        ])
        .add_row(vec![
            Cell::new("ADC Digital Power (2.5V) VCP"),
            Cell::new(&format!("{:.3}[V] | {:.3}[A] | {:.3}[W]", rat.rb1.rb_vcp.adc_dvdd_vcp[0], rat.rb1.rb_vcp.adc_dvdd_vcp[1], rat.rb1.rb_vcp.adc_dvdd_vcp[2])),
            Cell::new(&format!("{:.3}[V] | {:.3}[A] | {:.3}[W]", rat.rb2.rb_vcp.adc_dvdd_vcp[0], rat.rb2.rb_vcp.adc_dvdd_vcp[1], rat.rb2.rb_vcp.adc_dvdd_vcp[2])),
        ])
        .add_row(vec![
            Cell::new("ADC Analog Power (3.0V) VCP"),
            Cell::new(&format!("{:.3}[V] | {:.3}[A] | {:.3}[W]", rat.rb1.rb_vcp.adc_avdd_vcp[0], rat.rb1.rb_vcp.adc_avdd_vcp[1], rat.rb1.rb_vcp.adc_avdd_vcp[2])),
            Cell::new(&format!("{:.3}[V] | {:.3}[A] | {:.3}[W]", rat.rb2.rb_vcp.adc_avdd_vcp[0], rat.rb2.rb_vcp.adc_avdd_vcp[1], rat.rb2.rb_vcp.adc_avdd_vcp[2])),
        ])
        .add_row(vec![
            Cell::new("Pressure"),
            Cell::new(&format!("{:.3}[hPa]", rat.rb1.rb_ph.pressure)),
            Cell::new(&format!("{:.3}[hPa]", rat.rb2.rb_ph.pressure)),
        ])
        .add_row(vec![
            Cell::new("Humidity"),
            Cell::new(&format!("{:.3}[%]", rat.rb1.rb_ph.humidity)),
            Cell::new(&format!("{:.3}[%]", rat.rb2.rb_ph.humidity)),
        ])
        .add_row(vec![
            Cell::new("Magnetic Field"),
            Cell::new(&format!("X: {:.3}[G] | Y: {:.3}[G] | Z: {:.3}[G] | Total: {:.3}[G]", rat.rb1.rb_mag.mag_xyz[0], rat.rb1.rb_mag.mag_xyz[1], rat.rb1.rb_mag.mag_xyz[2], (rat.rb1.rb_mag.mag_xyz[0].powf(2.0) + rat.rb1.rb_mag.mag_xyz[1].powf(2.0) + rat.rb1.rb_mag.mag_xyz[2].powf(2.0)).sqrt())),
            Cell::new(&format!("X: {:.3}[G] | Y: {:.3}[G] | Z: {:.3}[G] | Total: {:.3}[G]", rat.rb2.rb_mag.mag_xyz[0], rat.rb2.rb_mag.mag_xyz[1], rat.rb2.rb_mag.mag_xyz[2], (rat.rb2.rb_mag.mag_xyz[0].powf(2.0) + rat.rb2.rb_mag.mag_xyz[1].powf(2.0) + rat.rb2.rb_mag.mag_xyz[2].powf(2.0)).sqrt())),
        ]);

    ltb_table.load_preset(UTF8_FULL)
        .set_header(vec![
            Cell::new("Key").add_attribute(Attribute::Bold),
            Cell::new("Value").add_attribute(Attribute::Bold),
        ])
        .add_row(vec![
            Cell::new("Trenz Temperature"),
            Cell::new(&format!("{:.3}[°C]", rat.ltb.ltb_temp.trenz_temp)),
        ])
        .add_row(vec![
            Cell::new("Board Temperature"),
            Cell::new(&format!("{:.3}[°C]", rat.ltb.ltb_temp.board_temp)),
        ])
        .add_row(vec![
            Cell::new("Threshold 0"),
            Cell::new(&format!("{:.3}[mV]", rat.ltb.ltb_threshold.thresholds[0])),
        ])
        .add_row(vec![
            Cell::new("Threshold 1"),
            Cell::new(&format!("{:.3}[mV]", rat.ltb.ltb_threshold.thresholds[1])),
        ])
        .add_row(vec![
            Cell::new("Threshold 2"),
            Cell::new(&format!("{:.3}[mV]", rat.ltb.ltb_threshold.thresholds[2])),
        ]);

    pb_table.load_preset(UTF8_FULL)
        .set_header(vec![
            Cell::new("Key").add_attribute(Attribute::Bold),
            Cell::new("Value").add_attribute(Attribute::Bold),
        ])
        .add_row(vec![
            Cell::new("PDS Temperature"),
            Cell::new(&format!("{:.3}[°C]", rat.pb.pb_temp.pds_temp)),
        ])
        .add_row(vec![
            Cell::new("PAS Temperature"),
            Cell::new(&format!("{:.3}[°C]", rat.pb.pb_temp.pas_temp)),
        ])
        .add_row(vec![
            Cell::new("NAS Temperature"),
            Cell::new(&format!("{:.3}[°C]", rat.pb.pb_temp.nas_temp)),
        ])
        .add_row(vec![
            Cell::new("SHV Temperature"),
            Cell::new(&format!("{:.3}[°C]", rat.pb.pb_temp.shv_temp)),
        ])
        .add_row(vec![
            Cell::new("3.6V Preamp Power VCP"),
            Cell::new(&format!("{:.3}[V] | {:.3}[A] | {:.3}[W]", rat.pb.pb_vcp.p3v6_preamp_vcp[0], rat.pb.pb_vcp.p3v6_preamp_vcp[1], rat.pb.pb_vcp.p3v6_preamp_vcp[2])),
        ])
        .add_row(vec![
            Cell::new("-1.6V Preamp Power VCP"),
            Cell::new(&format!("{:.2}[V] | {:.3}[A] | {:.3}[W]", rat.pb.pb_vcp.n1v6_preamp_vcp[0], rat.pb.pb_vcp.n1v6_preamp_vcp[1], rat.pb.pb_vcp.n1v6_preamp_vcp[2])),
        ])
        .add_row(vec![
            Cell::new("3.4V LTB Digital Power VCP"),
            Cell::new(&format!("{:.3}[V] | {:.3}[A] | {:.3}[W]", rat.pb.pb_vcp.p3v4d_ltb_vcp[0], rat.pb.pb_vcp.p3v4d_ltb_vcp[1], rat.pb.pb_vcp.p3v4d_ltb_vcp[2])),
        ])
        .add_row(vec![
            Cell::new("3.4V LTB Trenz Power VCP"),
            Cell::new(&format!("{:.3}[V] | {:.3}[A] | {:.3}[W]", rat.pb.pb_vcp.p3v4f_ltb_vcp[0], rat.pb.pb_vcp.p3v4f_ltb_vcp[1], rat.pb.pb_vcp.p3v4f_ltb_vcp[2])),
        ])
        .add_row(vec![
            Cell::new("3.6V LTB Power VCP"),
            Cell::new(&format!("{:.3}[V] | {:.3}[A] | {:.3}[W]", rat.pb.pb_vcp.p3v6_ltb_vcp[0], rat.pb.pb_vcp.p3v6_ltb_vcp[1], rat.pb.pb_vcp.p3v6_ltb_vcp[2])),
        ])
        .add_row(vec![
            Cell::new("-1.6V LTB Power VCP"),
            Cell::new(&format!("{:.2}[V] | {:.3}[A] | {:.3}[W]", rat.pb.pb_vcp.n1v6_ltb_vcp[0], rat.pb.pb_vcp.n1v6_ltb_vcp[1], rat.pb.pb_vcp.n1v6_ltb_vcp[2])),
        ]);

    preamp_table.load_preset(UTF8_FULL)
        .set_header(vec![
            Cell::new("Preamp #").add_attribute(Attribute::Bold),
            Cell::new("Temperature").add_attribute(Attribute::Bold),
            Cell::new("SiPM Bias Voltage").add_attribute(Attribute::Bold),
        ])
        .add_row(vec![
            Cell::new("Preamp 1"),
            Cell::new(&format!("{:.3}[°C]", rat.preamp.preamp_temp.preamp_temps[0])),
            Cell::new(&format!("{:.3}[V]", rat.preamp.preamp_bias.read_biases[0])),
        ])
        .add_row(vec![
            Cell::new("Preamp 2"),
            Cell::new(&format!("{:.3}[°C]", rat.preamp.preamp_temp.preamp_temps[1])),
            Cell::new(&format!("{:.3}[V]", rat.preamp.preamp_bias.read_biases[1])),
        ])
        .add_row(vec![
            Cell::new("Preamp 3"),
            Cell::new(&format!("{:.3}[°C]", rat.preamp.preamp_temp.preamp_temps[2])),
            Cell::new(&format!("{:.3}[V]", rat.preamp.preamp_bias.read_biases[2])),
        ])
        .add_row(vec![
            Cell::new("Preamp 4"),
            Cell::new(&format!("{:.3}[°C]", rat.preamp.preamp_temp.preamp_temps[3])),
            Cell::new(&format!("{:.3}[V]", rat.preamp.preamp_bias.read_biases[3])),
        ])
        .add_row(vec![
            Cell::new("Preamp 5"),
            Cell::new(&format!("{:.3}[°C]", rat.preamp.preamp_temp.preamp_temps[4])),
            Cell::new(&format!("{:.3}[V]", rat.preamp.preamp_bias.read_biases[4])),
        ])
        .add_row(vec![
            Cell::new("Preamp 6"),
            Cell::new(&format!("{:.3}[°C]", rat.preamp.preamp_temp.preamp_temps[5])),
            Cell::new(&format!("{:.3}[V]", rat.preamp.preamp_bias.read_biases[5])),
        ])
        .add_row(vec![
            Cell::new("Preamp 7"),
            Cell::new(&format!("{:.3}[°C]", rat.preamp.preamp_temp.preamp_temps[6])),
            Cell::new(&format!("{:.3}[V]", rat.preamp.preamp_bias.read_biases[6])),
        ])
        .add_row(vec![
            Cell::new("Preamp 8"),
            Cell::new(&format!("{:.3}[°C]", rat.preamp.preamp_temp.preamp_temps[7])),
            Cell::new(&format!("{:.3}[V]", rat.preamp.preamp_bias.read_biases[7])),
        ])
        .add_row(vec![
            Cell::new("Preamp 9"),
            Cell::new(&format!("{:.3}[°C]", rat.preamp.preamp_temp.preamp_temps[8])),
            Cell::new(&format!("{:.3}[V]", rat.preamp.preamp_bias.read_biases[8])),
        ])
        .add_row(vec![
            Cell::new("Preamp 10"),
            Cell::new(&format!("{:.3}[°C]", rat.preamp.preamp_temp.preamp_temps[9])),
            Cell::new(&format!("{:.3}[V]", rat.preamp.preamp_bias.read_biases[9])),
        ])
        .add_row(vec![
            Cell::new("Preamp 11"),
            Cell::new(&format!("{:.3}[°C]", rat.preamp.preamp_temp.preamp_temps[10])),
            Cell::new(&format!("{:.3}[V]", rat.preamp.preamp_bias.read_biases[10])),
        ])
        .add_row(vec![
            Cell::new("Preamp 12"),
            Cell::new(&format!("{:.3}[°C]", rat.preamp.preamp_temp.preamp_temps[11])),
            Cell::new(&format!("{:.3}[V]", rat.preamp.preamp_bias.read_biases[11])),
        ])
        .add_row(vec![
            Cell::new("Preamp 13"),
            Cell::new(&format!("{:.3}[°C]", rat.preamp.preamp_temp.preamp_temps[12])),
            Cell::new(&format!("{:.3}[V]", rat.preamp.preamp_bias.read_biases[12])),
        ])
        .add_row(vec![
            Cell::new("Preamp 14"),
            Cell::new(&format!("{:.3}[°C]", rat.preamp.preamp_temp.preamp_temps[13])),
            Cell::new(&format!("{:.3}[V]", rat.preamp.preamp_bias.read_biases[13])),
        ])
        .add_row(vec![
            Cell::new("Preamp 15"),
            Cell::new(&format!("{:.3}[°C]", rat.preamp.preamp_temp.preamp_temps[14])),
            Cell::new(&format!("{:.3}[V]", rat.preamp.preamp_bias.read_biases[14])),
        ])
        .add_row(vec![
            Cell::new("Preamp 16"),
            Cell::new(&format!("{:.3}[°C]", rat.preamp.preamp_temp.preamp_temps[15])),
            Cell::new(&format!("{:.3}[V]", rat.preamp.preamp_bias.read_biases[15])),
        ]);

    println!("RAT{} Information Table:", rat.rat);
    println!("RB Information Table:");
    println!("{rb_table}");
    println!("LTB Information Table:");
    println!("{ltb_table}");
    println!("PB Information Table:");
    println!("{pb_table}");
    println!("Preamp Information Table:");
    println!("{preamp_table}");
}

#[derive(Debug, Serialize, Deserialize)]
struct RATJson {
    rat: u8,
    dt: String,
    rb1: RBJson,
    rb2: RBJson,
    ltb: LTBJson,
    pb: PBJson,
    preamp: PreampJson,
}

#[derive(Debug, Serialize, Deserialize)]
struct RATJsonRB1 {
    rat: u8,
    dt: String,
    rb: RBJson,
    ltb: LTBJson,
}
#[derive(Debug, Serialize, Deserialize)]
struct RATJsonRB2 {
    rat: u8,
    dt: String,
    rb: RBJson,
    pb: PBJson,
    preamp: PreampJson,
}
#[derive(Debug, Serialize, Deserialize)]
struct RBJson {
    rb_info: RBInfoDebug,
    rb_temp: RBTempDebug,
    rb_vcp: RBVcp,
    rb_ph: RBPh,
    rb_mag: RBMag,
}
#[derive(Debug, Serialize, Deserialize)]
struct LTBJson {
    ltb_temp: LTBTemp,
    ltb_threshold: LTBThreshold,
}
#[derive(Debug, Serialize, Deserialize)]
struct PBJson {
    pb_temp: PBTemp,
    pb_vcp: PBVcp,
}
#[derive(Debug, Serialize, Deserialize)]
struct PreampJson {
    preamp_temp: PreampTemp,
    preamp_bias: PreampReadBias,
}

fn match_rb_from_rat(rat: u8) -> [u8; 2] {
    let mut rbs: [u8; 2] = Default::default();
    match rat {
        1 => rbs = [15, 3],
        2 => rbs = [14, 32],
        3 => rbs = [29, 31],
        4 => rbs = [13, 35],
        5 => rbs = [21, 23],
        6 => rbs = [24, 27],
        7 => rbs = [19, 20],
        8 => rbs = [25, 16],
        9 => rbs = [30, 8],
        10 => rbs = [11, 1],
        11 => rbs = [22, 26],
        12 => rbs = [40, 39],
        13 => rbs = [18, 9],
        14 => rbs = [42, 41],
        15 => rbs = [4, 2],
        16 => rbs = [44, 46],
        17 => rbs = [17, 7],
        18 => rbs = [34, 33],
        19 => rbs = [6, 36],
        20 => rbs = [5, 28],
        21 => rbs = [48, 47],
        22 => rbs = [49, 37],
        _ => rbs = [0, 0],
    }

    rbs
}

#[derive(Debug, Serialize, Deserialize)]
struct SYSJson {
    dt: String,
    cpu: CPUJson,
}


fn get_sys_json(dt: DateTime<Utc>, all: bool) {

    let cpu = CPUJson {
        info: CPUInfoDebug::new(),
        temp: CPUTempDebug::new(),
    };

    // let dt = Utc::now();
    let dt_fmt = format!("{:?}", &dt);
    let dt_json = dt.format("%Y%m%d%H%M%S").to_string();

    let sys_struct = SYSJson {
        dt: dt_fmt,
        cpu: cpu,
    };

    // Save Json to File
    let mut json_file = Default::default();
    if all {
        json_file = format!("sys_{}_all.json", dt_json);
    } else {
        json_file = format!("sys_{}.json", dt_json);
    }

    let json_file_path = ST_DIR.to_owned() + &json_file.to_owned();
    match save_sys_json_file(json_file_path, &sys_struct) {
        Ok(_) => {
            println!("Saved json file for System Box.\n");
        }
        Err(err) => {
            eprintln!("Failed to save json file for System Box: {}", err);
        }
    }

    // Print Json
    if !all {
        print_sys_json(&sys_struct);
    }

    // let json_rat = serde_json::to_string_pretty(&rat_struct).unwrap();

    // println!("{}", json_rat);

}

fn print_sys_json(sys: &SYSJson) {
    let mut table = Table::new();

    table.load_preset(UTF8_FULL)
        .set_header(vec![
            Cell::new("Key").add_attribute(Attribute::Bold),
            Cell::new("Value").add_attribute(Attribute::Bold),
        ])
        .add_row(vec![
            Cell::new("Datetime"),
            Cell::new(&format!("{}", sys.dt)),
        ])
        .add_row(vec![
            Cell::new("CPU Uptime"),
            Cell::new(&format!("{}[s]", sys.cpu.info.uptime)),
        ])
        .add_row(vec![
            Cell::new("CPU Disk Usage (/)"),
            Cell::new(&format!("{}[%]", sys.cpu.info.disk_usage)),
        ])
        .add_row(vec![
            Cell::new("CPU Frequencies"),
            Cell::new(&format!("CPU0: {}[Hz] | CPU1: {}[Hz] | CPU2: {}[Hz] | CPU3: {}[Hz]", sys.cpu.info.cpu_freq[0], sys.cpu.info.cpu_freq[1], sys.cpu.info.cpu_freq[2], sys.cpu.info.cpu_freq[3])),
        ])
        .add_row(vec![
            Cell::new("CPU Package Temperature"),
            Cell::new(&format!("{:.3}[°C]", sys.cpu.temp.cpu_temp)),
        ])
        .add_row(vec![
            Cell::new("CPU0 Temperature"),
            Cell::new(&format!("{:.3}[°C]", sys.cpu.temp.cpu0_temp)),
        ])
        .add_row(vec![
            Cell::new("CPU1 Temperature"),
            Cell::new(&format!("{:.3}[°C]", sys.cpu.temp.cpu1_temp)),
        ])
        .add_row(vec![
            Cell::new("Motherboard Temperature"),
            Cell::new(&format!("{:.3}[°C]", sys.cpu.temp.mb_temp)),
        ]);

    println!("System Box Information Table:");
    println!("{table}");
}


#[derive(Debug, Serialize, Deserialize)]
struct CPUJson {
    info: CPUInfoDebug,
    temp: CPUTempDebug,
}