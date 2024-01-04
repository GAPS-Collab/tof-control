use clap::{Parser, Subcommand, ValueEnum};

use tof_control::helper::cpc_type::CPCTemp;

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
    Temp,
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
                    }
                }
                None => {
                    print_temp();
                }
            }
        }
    }
}

fn print_temp() {
    let cpc_temp = CPCTemp::new();

    println!("CPC Temperature");
    println!("\tCPC Temp            : {:.3}[°C]", cpc_temp.cpc_temp);
}