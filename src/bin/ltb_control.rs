use clap::{Parser, Subcommand, ValueEnum};
use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

use tof_control::constant::{I2C_BUS, LTB_TRENZ_ADDRESS};
use tof_control::helper::ltb_type::{LTBTemp, LTBThreshold};
use tof_control::ltb_control::ltb_threshold;

#[derive(Parser, Debug)]
#[command(author = "Takeru Hayashi", version = "0.2.0", about, long_about = None)]
struct Cli {
    #[clap(short, long, global = true, help = "Verbose mode")]
    verbose: bool,
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
    #[clap(short_flag = 's')]
    Set {
        channel: Option<u8>,
        threshold: Option<f32>,
    },
    #[clap(short_flag = 'R')]
    Reset {
        channel: Option<u8>,
    },
    #[clap(short_flag = 'i')]
    Initialize {
    },
}

#[derive(ValueEnum, Clone, Debug)]
enum Sensor {
    Temp,
    Threshold,
}

fn main() {

    // Check if LTB is connected
    if check_i2c(I2C_BUS, LTB_TRENZ_ADDRESS).is_err() {
        std::process::exit(0);
    }

    let cli = Cli::parse();

    match cli.command {
        Commands::Read { sensor } => {
            match sensor {
                Some(s) => {
                    match s {
                        Sensor::Temp => {
                            print_temp();
                        },
                        Sensor::Threshold => {
                            print_thresholds();
                        },
                    }
                },
                None => {
                    print_temp();
                    print_thresholds();
                }
            }
        },
        Commands::Set { channel, threshold } => {
            match (channel, threshold) {
                (Some(c), Some(t)) => {
                    set_threshold(c, t);
                },
                (Some(_), None) => {
                    println!("Missing threshold argument")
                },
                (None, Some(_)) => {
                    println!("Missing channel argument")
                },
                (None, None) => {
                    set_default_threshold();
                },
            }
        },
        Commands::Reset { channel } => {
            match channel {
                Some(c) => {
                    set_threshold(c, 0.0);
                }
                None => {
                    reset_threshold();
                }
            }
        },
        Commands::Initialize {  } => {
            set_default_threshold();
        }
    }

}

fn check_i2c(bus: u8, address: u16) -> Result<(), LinuxI2CError> {
    let mut dev = LinuxI2CDevice::new(&format!("/dev/i2c-{}", bus), address)?;
    dev.smbus_read_byte()?;

    Ok(())
}

fn print_temp() {
    let temperature = LTBTemp::new();
    let trenz_temp = temperature.trenz_temp;
    let board_temp = temperature.board_temp;

    println!("LTB Temperature");
    println!("\tTrenz Temperature       : {:.3}[°C]", trenz_temp);
    println!("\tBoard Temperature       : {:.3}[°C]", board_temp);
}

fn print_thresholds() {
    let thresholds = LTBThreshold::new().thresholds;

    println!("LTB Threshold");
    // for (i, threshold) in thresholds.iter().enumerate() {
    //     println!("\tThreshold {}             : {:.3}[mV]", i, threshold);
    // }
    println!("\tThreshold 0             : {:.3}[mV]", thresholds[0]);
    println!("\tThreshold 1             : {:.3}[mV]", thresholds[1]);
    println!("\tThreshold 2             : {:.3}[mV]", thresholds[2]);
}

fn set_default_threshold() {
    match ltb_threshold::set_default_threshold() {
        Ok(_) => {},
        Err(e) => eprintln!("{:?}", e),
    }
}

fn set_threshold(channel: u8, threshold: f32) {
    match ltb_threshold::set_threshold(channel, threshold) {
        Ok(_) => {},
        Err(e) => eprintln!("{:?}", e),
    }
}

fn reset_threshold() {
    match ltb_threshold::reset_threshold() {
        Ok(_) => {},
        Err(e) => eprintln!("{:?}", e),
    }
}