use clap::{Parser, Subcommand, ValueEnum};

use tof_control::helper::pb_type::{PBTemp, PBError};

#[derive(Parser, Debug)]
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
    //     // #[arg(short = 't', long = "threshold")]
    //     threshold: Option<f32>,
    // },
    // #[clap(short_flag = 'R')]
    // Reset {
    //     // #[arg(short = 'c', long = "channel")]
    //     channel: Option<u8>,
    // }
}

#[derive(ValueEnum, Clone, Debug)]
enum Sensor {
    Temp,
    Vpc,
}

fn main() -> Result<(), PBError> {

    let cli = Cli::parse();

    match cli.command {
        Commands::Read { sensor } => {
            match sensor {
                Some(s) => {
                    match s {
                        Sensor::Temp => {
                            read_temp();
                        },
                        Sensor::Vpc => {
                            todo!();
                        },
                    }
                },
                None => {
                    read_temp();
                    // todo!();
                }
            }
        }
    }

    Ok(())
}

fn read_temp() {
    let temperature = PBTemp::new();
    let pds_temp = temperature.pds_temp;
    let pas_temp = temperature.pas_temp;
    let nas_temp = temperature.nas_temp;
    let shv_temp = temperature.shv_temp;

    println!("PB Temperature");
    println!("\tPDS Temperature: {}", pds_temp);
    println!("\tPAS Temperature: {}", pas_temp);
    println!("\tNAS Temperature: {}", nas_temp);
    println!("\tSHV Temperature: {}", shv_temp);
}
