use clap::{Parser, Subcommand, ValueEnum};

use tof_control::helper::ltb_type::{LTBTemp, LTBThreshold, LTBError};
use tof_control::ltb_control::ltb_threshold;

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
    #[clap(short_flag = 's')]
    Set {
        #[arg(short = 'c', long = "channel")]
        channel: Option<u8>,
        #[arg(short = 't', long = "threshold")]
        threshold: Option<f32>,
    },
    #[clap(short_flag = 'R')]
    Reset {
        #[arg(short = 'c', long = "channel")]
        channel: Option<u8>,
    }
}

#[derive(ValueEnum, Clone, Debug)]
enum Sensor {
    Temp,
    Threshold,
}

fn main() -> Result<(), LTBError> {

    let cli = Cli::parse();

    match cli.command {
        Commands::Read { sensor } => {
            match sensor {
                Some(s) => {
                    match s {
                        Sensor::Temp => {
                            read_temp();
                        },
                        Sensor::Threshold => {
                            read_threshold();
                        },
                    }
                },
                None => {
                    read_temp();
                    read_threshold();
                }
            }
        },
        Commands::Set { channel, threshold } => {
            match (channel, threshold) {
                (Some(c), Some(t)) => {
                    // match ltb_threshold::set_threshold(c, t) {
                    //     Ok(_) => {},
                    //     Err(e) => println!("Unable to set threshold: {:?}", e),
                    // }
                    ltb_threshold::set_threshold(c, t)?;
                },
                (Some(_), None) => {
                    println!("Missing threshold argument")
                },
                (None, Some(_)) => {
                    println!("Missing channel argument")
                },
                (None, None) => {
                    ltb_threshold::set_default_threshold()?;
                },
            }
        },
        Commands::Reset { channel } => {
            match channel {
                Some(c) => {
                    // match ltb_threshold::set_threshold(c, 0.0) {
                    //     Ok(_) => {},
                    //     Err(e) => println!("Unable to resetset threshold on channel {}: {:?}", c, e),
                    // }
                    ltb_threshold::set_threshold(c, 0.0)?;
                }
                None => {
                    ltb_threshold::reset_threshold()?;
                }
            }
        }
    }

    Ok(())

}

fn read_temp() {
    let temperature = LTBTemp::new();
    let ltb_temp = temperature.ltb_temp;
    let trenz_temp = temperature.trenz_temp;

    println!("LTB Temperature");
    println!("\tBoard Temperature: {}", ltb_temp);
    println!("\tTrenz Temperature: {}", trenz_temp);
}

fn read_threshold() {
    let threshold = LTBThreshold::new();
    let thresholds = [
        threshold.threshold_0,
        threshold.threshold_1,
        threshold.threshold_2,
    ];

    println!("LTB Threshold");
    for (i, threshold) in thresholds.iter().enumerate() {
        println!("\tThreshold {}: {}", i, threshold);
    }
}