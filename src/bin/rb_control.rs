use clap::{Parser, Subcommand, ValueEnum};
use std::fs::OpenOptions;
use std::io::prelude::*;
use chrono::Utc;

use tof_control::helper::rb_type::{RBInfo, RBTemp, RBVcp, RBPh, RBMag, RBInitError};
use tof_control::rb_control::{rb_info, rb_init, rb_clk, rb_gpioe, rb_dac};
use tof_control::constant::*;

#[derive(Parser, Debug)]
#[command(author = "Takeru Hayashi", version = "0.1.0", about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(short_flag = 'r')]
    Read {
        #[arg(ignore_case = true, value_enum)]
        sensor: Option<Sensor>,
    },
    // #[clap(short_flag = 's')]
    // Set {
    //     // #[arg(short = 'c', long = "channel")]
    //     channel: Option<u8>,
    //     // #[arg(short = 'b', long = "bias")]
    //     bias: Option<f32>,
    // },
    // // #[clap(short_flag = 'R')]
    // Reset {
    //     // #[arg(short = 'c', long = "channel")]
    //     channel: Option<u8>,
    // },
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
        // Commands::Set { channel, bias } => {
        //     match (channel, bias) {
        //         (Some(c), Some(b)) => {
        //             todo!();
        //         },
        //         (Some(_), None) => {
        //             todo!();
        //         },
        //         (None, Some(_)) => {
        //             todo!();
        //         },
        //         (None, None) => {
        //             set_default_bias();
        //         }
        //     }
        // },
        // Commands::Reset { channel } => {
        //     match channel {
        //         Some(c) => {
        //             todo!();
        //         }
        //         None => {
        //             reset_bias();
        //         }
        //     }
        // },
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
    let rb_info = RBInfo::new();

    println!("RB Information");
    println!("\tBoard ID:           {}", rb_info.board_id);
    println!("\tLOL:                {}", if rb_info.lol == 0x01 { "Unlocked" } else { "Locked" });
    println!("\tLOL Stable:         {}", if rb_info.lol_stable == 0x01 { "Unlocked Past Second" } else { "Locked Past Second" });
    println!("\tTrigger Rate (MTB): {}[Hz]", rb_info.trig_rate);
    // Additional Info
    println!("\tFirmware Version:   {}", rb_info.fw_version);
    println!("\tReadout Mask:       {:#X}", rb_info.readout_mask);
}

fn print_temp() {
    let rb_temp = RBTemp::new();

    println!("RB Temperature");
    println!("\tZYNQ Temp:          {:.3}[°C]", rb_temp.zynq_temp);
    println!("\tDRS Temp:           {:.3}[°C]", rb_temp.drs_temp);
    println!("\tCLK Temp:           {:.3}[°C]", rb_temp.clk_temp);
    println!("\tADC Temp:           {:.3}[°C]", rb_temp.adc_temp);
}

fn print_vcp() {
    let rb_vcp = RBVcp::new();

    println!("RB VCP (Voltage, Current and Power)");
    println!("\tZYNQ VCP:           {:.3}[V] | {:.3}[A] | {:.3}[W]", rb_vcp.zynq_vcp[0], rb_vcp.zynq_vcp[1], rb_vcp.zynq_vcp[2]);
    println!("\t3.3V VCP:           {:.3}[V] | {:.3}[A] | {:.3}[W]", rb_vcp.p3v3_vcp[0], rb_vcp.p3v3_vcp[1], rb_vcp.p3v3_vcp[2]);
    println!("\t3.5V VCP:           {:.3}[V] | {:.3}[A] | {:.3}[W]", rb_vcp.p3v5_vcp[0], rb_vcp.p3v5_vcp[1], rb_vcp.p3v5_vcp[2]);
    println!("\t-1.5V VCP:         {:.3}[V] | {:.3}[A] | {:.3}[W]", rb_vcp.n1v5_vcp[0], rb_vcp.n1v5_vcp[1], rb_vcp.n1v5_vcp[2]);
    println!("\tDRS DVDD VCP:       {:.3}[V] | {:.3}[A] | {:.3}[W]", rb_vcp.drs_dvdd_vcp[0], rb_vcp.drs_dvdd_vcp[1], rb_vcp.drs_dvdd_vcp[2]);
    println!("\tDRS AVDD VCP:       {:.3}[V] | {:.3}[A] | {:.3}[W]", rb_vcp.drs_avdd_vcp[0], rb_vcp.drs_avdd_vcp[1], rb_vcp.drs_avdd_vcp[2]);
    println!("\tADC DVDD VCP:       {:.3}[V] | {:.3}[A] | {:.3}[W]", rb_vcp.adc_dvdd_vcp[0], rb_vcp.adc_dvdd_vcp[1], rb_vcp.adc_dvdd_vcp[2]);
    println!("\tADC AVDD VCP:       {:.3}[V] | {:.3}[A] | {:.3}[W]", rb_vcp.adc_avdd_vcp[0], rb_vcp.adc_avdd_vcp[1], rb_vcp.adc_avdd_vcp[2]);
}

fn print_ph() {
    let rb_ph = RBPh::new();

    println!("RB PH (Pressure and Humidity)");
    println!("\tPressure:           {:.3}[hPa]", rb_ph.pressure);
    println!("\tHumidity:           {:.3}[%]", rb_ph.humidity);
}

fn print_mag() {
    let rb_mag = RBMag::new();

    println!("RB Magnetic Sensor");
    println!("\tX-Axis:             {:.3}[G]", rb_mag.mag_xyz[0]);
    println!("\tY-Axis:             {:.3}[G]", rb_mag.mag_xyz[1]);
    println!("\tZ-Axis:             {:.3}[G]", rb_mag.mag_xyz[2]);
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

// fn initialize_rb() {
//     match rb_dac::set_dac() {
//         Ok(_) => {},
//         Err(e) => eprintln!("{:?}", e),
//     }

//     match set_board_id() {
//         Ok(_) => {},
//         Err(e) => eprintln!("{:?}", e),
//     }
    
//     match initialize_daq() {
//         Ok(_) => {},
//         Err(e) => eprintln!("{:?}", e),
//     }
// }

// fn set_board_id() -> Result<(), RBInitError> {
//     let hostname = gethostname().into_string()?;
//     let board_id = hostname.replace("tof-rb", "").parse::<u32>()?;

//     write_control_reg(BOARD_ID, board_id)?;

//     Ok(())
// }

// fn initialize_daq() -> Result<(), RBInitError> {
//     let mut value: u32;
//     // Disable DAQ Fragment
//     value = read_control_reg(DAQ_FRAGMENT_EN)?;
//     value = value | 0x00;
//     write_control_reg(DAQ_FRAGMENT_EN, value)?;

//     // Enable Spike Clean
//     value = read_control_reg(EN_SPIKE_REMOVAL)?;
//     value = value | 0x400000;
//     write_control_reg(EN_SPIKE_REMOVAL, value)?;

//     // Enable All Channels
//     value = read_control_reg(READOUT_MASK)?;
//     value = value | 0x1FF;
//     write_control_reg(READOUT_MASK, value)?;

//     // Start DRS Chip
//     write_control_reg(START, 0x01)?;

//     Ok(())
// }