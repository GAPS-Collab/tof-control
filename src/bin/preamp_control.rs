use clap::{Parser, Subcommand, ValueEnum};

use tof_control::helper::preamp_type::{PreampTemp, PreampBias};
use tof_control::preamp_control::preamp_bias;

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
        // #[arg(short = 'b', long = "bias")]
        bias: Option<f32>,
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
    Bias,
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
                        Sensor::Bias => {
                            read_bias();
                        },
                    }
                },
                None => {
                    read_temp();
                    read_bias();
                }
            }
        },
        Commands::Set { channel, bias } => {
            match (channel, bias) {
                (Some(c), Some(b)) => {
                    todo!();
                },
                (Some(_), None) => {
                    todo!();
                },
                (None, Some(_)) => {
                    todo!();
                },
                (None, None) => {
                    set_default_bias();
                }
            }
        },
        Commands::Reset { channel } => {
            match channel {
                Some(c) => {
                    todo!();
                }
                None => {
                    reset_bias();
                }
            }
        },
        Commands::Initialize {  } => {
            set_default_bias();
        }
    }
}

fn read_temp() {
    let temperatures = PreampTemp::new().preamp_temps;

    if temperatures == [f32::MAX; 16] {
        println!("Preamps are not connected");
    } else {
        println!("Preamp Temperature");
        // for i in 1..=16 {
        for (i, temp) in temperatures.iter().enumerate() {
            if *temp == f32::MAX {
                println!("\tPreamp {} Temperature: NC", i+1);
            } else {
                println!("\tPreamp {} Temperature: {:.3}[°C]", i+1, temp);
            }
        }
    }
}

fn read_bias() {
    let bias_voltages = PreampBias::new().preamp_biases;

    if bias_voltages == [f32::MAX; 16] {
        println!("Preamps are not connected");
    } else {
        println!("Preamp Bias Voltages");
        for (i, bias) in bias_voltages.iter().enumerate() {
            if *bias == f32::MAX {
                println!("\tPreamp {} Bias Voltage: NC", i+1);
            } else {
                println!("\tPreamp {} Bias Voltage: {:.3}[V]", i+1, bias);
            }
        }
    }
}

fn set_default_bias() {
    match preamp_bias::set_default_bias() {
        Ok(_) => {},
        Err(e) => eprintln!("{:?}", e),
    }
}

fn reset_bias() {
    match preamp_bias::reset_bias() {
        Ok(_) => {},
        Err(e) => eprintln!("{:?}", e),
     }
}