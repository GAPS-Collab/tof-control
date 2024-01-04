use clap::{Parser, Subcommand, ValueEnum};
use std::fs::OpenOptions;
use std::io::prelude::*;
use chrono::Utc;

use tof_control::helper::rb_type::{RBInfoDebug, RBTemp, RBVcp, RBPh, RBMag};
use tof_control::rb_control::{rb_init, rb_mode};

#[derive(Parser, Debug)]
#[command(author = "Takeru Hayashi", version = "0.2.0", about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
    #[arg(short, long, help = "Verbose mode")]
    verbose: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(short_flag = 'r')]
    Read {
        #[arg(ignore_case = true, value_enum)]
        sensor: Option<Sensor>,
    },
    #[clap(short_flag = 'm')]
    Mode {
        #[arg(ignore_case = true, value_enum)]
        mode: Mode,
    },
    #[clap(short_flag = 'i')]
    Initialize {
    },
}

#[derive(ValueEnum, Clone, Debug)]
enum Sensor {
    Info,
    Temp,
    Vcp,
    Ph,
    Mag,
}

#[derive(ValueEnum, Clone, Debug)]
enum Mode {
    Noi,
    Vcal,
    Tcal,
    Sma,
}


fn main() {

    let cli = Cli::parse();

    match cli.command {
        Commands::Read { sensor } => {
            match sensor {
                Some(s) => {
                    match s {
                        Sensor::Info => {
                            print_info();
                        }
                        Sensor::Temp => {
                            print_temp();
                        }
                        Sensor::Vcp => {
                            print_vcp();
                        }
                        Sensor::Ph => {
                            print_ph();
                        }
                        Sensor::Mag => {
                            print_mag();
                        }
                    }
                },
                None => {
                    print_info();
                    print_temp();
                    print_vcp();
                    print_ph();
                    print_mag();
                }
            }
        },
        Commands::Mode { mode } => {
            match mode {
                Mode::Noi => {
                    println!("NOI mode");
                    select_mode(0);
                }
                Mode::Vcal => {
                    println!("VCAL mode");
                    select_mode(1);
                }
                Mode::Tcal => {
                    println!("TCAL mode");
                    select_mode(2);
                }
                Mode::Sma => {
                    println!("SMA mode");
                    select_mode(3);
                }
            }
        }
        Commands::Initialize {  } => {
            initialize_rb();
        }
    }
}

fn initialize_rb() {
    match rb_init::initialize() {
        Ok(_) => {},
        Err(e) => {
            write_err_log(e.to_string()).unwrap();
            std::process::exit(1);
        }
    }
}

fn print_info() {
    let rb_info = RBInfoDebug::new();

    println!("RB Information");
    println!("\tBoard ID            : {}", rb_info.board_id);
    println!("\tSub Board           : {}", if rb_info.sub_board == 1 { "LTB" } else if rb_info.sub_board == 2 { "PB/Preamp" } else { "NC" });
    println!("\tLOL                 : {}", if rb_info.lol == 0x01 { "Unlocked" } else { "Locked" });
    println!("\tLOL Stable          : {}", if rb_info.lol_stable == 0x01 { "Unlocked Past Second" } else { "Locked Past Second" });
    println!("\tTrigger Rate (MTB)  : {}[Hz]", rb_info.trig_rate);
    // Additional Info
    println!("\tFirmware Version    : {}", rb_info.fw_version);
    println!("\tUptime              : {}[s]", rb_info.uptime);
    println!("\tSD Usage            : {}[%]", rb_info.sd_usage);
    println!("\tInput Mode          : {}", rb_info.input_mode);
}

fn print_temp() {
    let rb_temp = RBTemp::new();

    println!("RB Temperature");
    println!("\tZYNQ Temp           : {:.3}[°C]", rb_temp.zynq_temp);
    println!("\tDRS Temp            : {:.3}[°C]", rb_temp.drs_temp);
    println!("\tCLK Temp            : {:.3}[°C]", rb_temp.clk_temp);
    println!("\tADC Temp            : {:.3}[°C]", rb_temp.adc_temp);
}

fn print_vcp() {
    let rb_vcp = RBVcp::new();

    println!("RB VCP (Voltage, Current and Power)");
    println!("\tZYNQ VCP            : {:.3}[V] | {:.3}[A] | {:.3}[W]", rb_vcp.zynq_vcp[0], rb_vcp.zynq_vcp[1], rb_vcp.zynq_vcp[2]);
    println!("\t3.3V VCP            : {:.3}[V] | {:.3}[A] | {:.3}[W]", rb_vcp.p3v3_vcp[0], rb_vcp.p3v3_vcp[1], rb_vcp.p3v3_vcp[2]);
    println!("\t3.5V VCP            : {:.3}[V] | {:.3}[A] | {:.3}[W]", rb_vcp.p3v5_vcp[0], rb_vcp.p3v5_vcp[1], rb_vcp.p3v5_vcp[2]);
    println!("\t-1.5V VCP           : {:.2}[V] | {:.3}[A] | {:.3}[W]", rb_vcp.n1v5_vcp[0], rb_vcp.n1v5_vcp[1], rb_vcp.n1v5_vcp[2]);
    println!("\tDRS DVDD VCP        : {:.3}[V] | {:.3}[A] | {:.3}[W]", rb_vcp.drs_dvdd_vcp[0], rb_vcp.drs_dvdd_vcp[1], rb_vcp.drs_dvdd_vcp[2]);
    println!("\tDRS AVDD VCP        : {:.3}[V] | {:.3}[A] | {:.3}[W]", rb_vcp.drs_avdd_vcp[0], rb_vcp.drs_avdd_vcp[1], rb_vcp.drs_avdd_vcp[2]);
    println!("\tADC DVDD VCP        : {:.3}[V] | {:.3}[A] | {:.3}[W]", rb_vcp.adc_dvdd_vcp[0], rb_vcp.adc_dvdd_vcp[1], rb_vcp.adc_dvdd_vcp[2]);
    println!("\tADC AVDD VCP        : {:.3}[V] | {:.3}[A] | {:.3}[W]", rb_vcp.adc_avdd_vcp[0], rb_vcp.adc_avdd_vcp[1], rb_vcp.adc_avdd_vcp[2]);
}

fn print_ph() {
    let rb_ph = RBPh::new();

    println!("RB PH (Pressure and Humidity)");
    println!("\tPressure            : {:.3}[hPa]", rb_ph.pressure);
    println!("\tHumidity            : {:.3}[%]", rb_ph.humidity);
}

fn print_mag() {
    let rb_mag = RBMag::new();

    println!("RB Magnetic Sensor");
    println!("\tX-Axis              : {:.3}[G]", rb_mag.mag_xyz[0]);
    println!("\tY-Axis              : {:.3}[G]", rb_mag.mag_xyz[1]);
    println!("\tZ-Axis              : {:.3}[G]", rb_mag.mag_xyz[2]);
}

fn select_mode(mode: u8) {
    match mode {
        0 => {
            match rb_mode::select_noi_mode() {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("{:?}", e);
                    std::process::exit(1);
                }
            }
        }
        1 => {
            match rb_mode::select_vcal_mode() {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("{:?}", e);
                    std::process::exit(1);
                }
            }
        }
        2 => {
            match rb_mode::select_tcal_mode() {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("{:?}", e);
                    std::process::exit(1);
                }
            }
        }
        _ => {
            match rb_mode::select_sma_mode() {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("{:?}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}

fn write_err_log(error: String) -> Result<(), std::io::Error> {

    let now = Utc::now().to_rfc3339();
    let err_msg = format!("{}: {}", now, error);
    
    let log_path = "/home/gaps/log/rb-init.log";
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
        ?;

    writeln!(file, "{}", err_msg)?;

    Ok(())
}