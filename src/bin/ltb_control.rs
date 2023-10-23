use clap::{Parser, Subcommand, ValueEnum};

use tof_control::helper::ltb_type::{LTBTemp, LTBThreshold};
use tof_control::ltb_control::ltb_threshold;

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
    #[clap(short_flag = 's')]
    Set {
        // #[arg(short = 'c', long = "channel")]
        channel: Option<u8>,
        // #[arg(short = 't', long = "threshold")]
        threshold: Option<f32>,
    },
    #[clap(short_flag = 'R')]
    Reset {
        // #[arg(short = 'c', long = "channel")]
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

fn read_temp() {
    let temperature = LTBTemp::new();
    let ltb_temp = temperature.ltb_temp;
    let trenz_temp = temperature.trenz_temp;

    println!("LTB Temperature");
    println!("\tBoard Temperature: {:.3}[°C]", ltb_temp);
    println!("\tTrenz Temperature: {:.3}[°C]", trenz_temp);
}

fn read_threshold() {
    let thresholds = LTBThreshold::new().thresholds;

    println!("LTB Threshold");
    for (i, threshold) in thresholds.iter().enumerate() {
        println!("\tThreshold {}: {:.3}[mV]", i, threshold);
    }
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

// No error even with RB without LTB
fn reset_threshold() {
    match ltb_threshold::reset_threshold() {
        Ok(_) => {},
        Err(e) => eprintln!("{:?}", e),
    }
}