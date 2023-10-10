use clap::{Parser, Subcommand, ValueEnum};

use tof_control::helper::preamp_type::{PreampTemp, PreampBias, PreampError};

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
    Bias,
}

fn main() -> Result<(), PreampError> {

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
        }
    }

    Ok(())
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
