use clap::{Parser, Subcommand, ValueEnum};

use tof_control::helper::cpu_type::{CPUInfo, CPUTemp};

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
    }
}

#[derive(ValueEnum, Clone, Debug)]
enum Sensor {
    Info,
    Temp,
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
                    }
                }
                None => {
                    print_info();
                    print_temp();
                }
            }
        }
    }
}

fn print_info() {
    let cpu_info = CPUInfo::new();

    println!("CPU Information");
    println!("\tUptime              : {}[s]", cpu_info.uptime);
}

fn print_temp() {
    let cpu_temp = CPUTemp::new();

    println!("CPU Temperature");
    println!("\tCPU0 Temp           : {:.3}[°C]", cpu_temp.cpu0_temp);
    println!("\tCPU1 Temp           : {:.3}[°C]", cpu_temp.cpu1_temp);
}