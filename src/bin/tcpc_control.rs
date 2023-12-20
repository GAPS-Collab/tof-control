use clap::{Parser, Subcommand, ValueEnum};
use chrono::prelude::*;
use chrono_tz::America::Los_Angeles;

use tof_control::helper::tcpc_type::{TCPCTemp, TCPCVcp};

#[derive(Parser, Debug)]
#[command(author = "Takeru Hayashi", version = "0.1.0", about, long_about = None)]
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
    #[clap(short_flag = 'd')]
    Debug {}
}

#[derive(ValueEnum, Clone, Debug)]
enum Sensor {
    Temp,
    Vcp,
}

fn main() {
    
    let cli = Cli::parse();

    match cli.command {
        Commands::Read { sensor } => {
            match sensor {
                Some(s) => {
                    match s {
                        Sensor::Temp => {
                            print_temp();
                        }
                        Sensor::Vcp => {
                            print_vcp();
                        }
                    }
                }
                None => {
                    print_temp();
                    print_vcp();
                }
            }
        }
        Commands::Debug {} => {
            if cli.verbose {
                println!("%Y %m %d %H %M %S TCPC_TEMP TCPC_VOL TCPC_CUR TCPC_PWR");
                print_debug_mode();
            } else {
                print_debug_mode();
            }
        }
    }
}

fn print_temp() {
    let tcpc_temp = TCPCTemp::new();

    println!("TCPC Temperature");
    println!("\tTCPC Temp            : {:.3}[°C]", tcpc_temp.tcpc_temp);
}

fn print_vcp() {
    let tcpc_vcp = TCPCVcp::new();

    println!("TCPC VCP");
    println!("\tTCPC Voltage          : {:.3}[V]", tcpc_vcp.tcpc_vcp[0]);
    println!("\tTCPC Current          : {:.3}[A]", tcpc_vcp.tcpc_vcp[1]);
    println!("\tTCPC Power            : {:.3}[W]", tcpc_vcp.tcpc_vcp[2]);
}

fn print_debug_mode() {
    let datetime = Utc::now().naive_utc();
    let datetime_local = Los_Angeles.from_utc_datetime(&datetime);
    let dt_format = datetime_local.format("%Y %m %d %H %M %S");

    let tcpc_temperature = TCPCTemp::new();
    let tcpc_temp = tcpc_temperature.tcpc_temp;

    let tcpc_vcp = TCPCVcp::new();
    let tcpc_voltage = tcpc_vcp.tcpc_vcp[0];
    let tcpc_current = tcpc_vcp.tcpc_vcp[1];
    let tcpc_power = tcpc_vcp.tcpc_vcp[2];


    println!("{} {} {} {} {}",
        dt_format,
        tcpc_temp,
        tcpc_voltage, tcpc_current, tcpc_power,
    );
}