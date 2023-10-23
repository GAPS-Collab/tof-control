use clap::{Parser, Subcommand, ValueEnum};
use std::fs::OpenOptions;
use std::io::prelude::*;
use chrono::Utc;

use tof_control::helper::rb_type::{RBInfo, RBInitError};
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
    // Temp,
    // Bias,
}

fn main() {

    let cli = Cli::parse();

    match cli.command {
        Commands::Read { sensor } => {
            match sensor {
                Some(s) => {
                    match s {
                        Sensor::Info => {
                            read_info();
                        },
                    }
                },
                None => {
                    read_info();
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

fn read_info() {
    let rb_info = RBInfo::new();

    println!("RB Information");
    println!("\tBoard ID:           {}", rb_info.board_id);
    println!("\tLOL:                {}", if rb_info.lol == 0x01 { "Unlocked" } else { "Locked" });
    println!("\tLOL Stable:         {}", if rb_info.lol_stable == 0x01 { "Unlocked Past Second" } else { "Locked Past Second" });
    println!("\tTrigger Rate (MTB): {}[Hz]", rb_info.trig_rate);
    // Additional Info
    println!("\tFirmware Version:   {}", rb_info.fw_version);
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