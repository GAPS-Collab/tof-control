use clap::{Parser, Subcommand, ValueEnum};
use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

use tof_control::constant::{I2C_BUS, PB_PCA9548A_ADDRESS};
use tof_control::helper::preamp_type::{PreampTemp, PreampReadBias, PreampSetBias};

#[derive(Parser, Debug)]
#[command(author = "Takeru Hayashi", version = "0.2.0", about, long_about = None)]
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

    // Check if PB is connected
    if check_i2c(I2C_BUS, PB_PCA9548A_ADDRESS).is_err() {
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
                        Sensor::Bias => {
                            print_bias();
                        },
                    }
                },
                None => {
                    if cli.verbose {
                        print_all();
                    } else {
                        print_temp();
                        print_bias();
                    }
                }
            }
        },
        Commands::Set { channel, bias } => {
            match (channel, bias) {
                (Some(c), Some(b)) => {
                    set_manual_bias(Some(c), b);
                },
                (Some(_), None) => {
                    println!("Nothing is set");
                },
                (None, Some(b)) => {
                    set_manual_bias(None, b);
                },
                (None, None) => {
                    set_bias();
                }
            }
        },
        Commands::Reset { channel } => {
            match channel {
                Some(c) => {
                    set_manual_bias(Some(c), 0.0);
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

fn print_temp() {
    let temperatures = PreampTemp::new().preamp_temps;

    println!("Preamp Temperature");
    for (i, temp) in temperatures.iter().enumerate() {
            println!("\tPreamp {} Temperature       : {:.3}[°C]", i+1, temp);
    }
    // println!("\tPreamp 1 Temperature    : {:.3}[°C]", temperatures[0]);
    // println!("\tPreamp 2 Temperature    : {:.3}[°C]", temperatures[1]);
    // println!("\tPreamp 3 Temperature    : {:.3}[°C]", temperatures[2]);
    // println!("\tPreamp 4 Temperature    : {:.3}[°C]", temperatures[3]);
    // println!("\tPreamp 5 Temperature    : {:.3}[°C]", temperatures[4]);
    // println!("\tPreamp 6 Temperature    : {:.3}[°C]", temperatures[5]);
    // println!("\tPreamp 7 Temperature    : {:.3}[°C]", temperatures[6]);
    // println!("\tPreamp 8 Temperature    : {:.3}[°C]", temperatures[7]);
    // println!("\tPreamp 9 Temperature    : {:.3}[°C]", temperatures[8]);
    // println!("\tPreamp 10 Temperature   : {:.3}[°C]", temperatures[9]);
    // println!("\tPreamp 11 Temperature   : {:.3}[°C]", temperatures[10]);
    // println!("\tPreamp 12 Temperature   : {:.3}[°C]", temperatures[11]);
    // println!("\tPreamp 13 Temperature   : {:.3}[°C]", temperatures[12]);
    // println!("\tPreamp 14 Temperature   : {:.3}[°C]", temperatures[13]);
    // println!("\tPreamp 15 Temperature   : {:.3}[°C]", temperatures[14]);
    // println!("\tPreamp 16 Temperature   : {:.3}[°C]", temperatures[15]);
}

fn print_bias() {
    let read_biases = PreampReadBias::new().read_biases;

    println!("Preamp Bias Voltages");
    for (i, bias) in read_biases.iter().enumerate() {
            println!("\tPreamp {} Bias       : {:.3}[V]", i+1, bias);
    }
    // println!("\tPreamp 1 Bias           : {:.3}[V]", read_biases[0]);
    // println!("\tPreamp 2 Bias           : {:.3}[V]", read_biases[1]);
    // println!("\tPreamp 3 Bias           : {:.3}[V]", read_biases[2]);
    // println!("\tPreamp 4 Bias           : {:.3}[V]", read_biases[3]);
    // println!("\tPreamp 5 Bias           : {:.3}[V]", read_biases[4]);
    // println!("\tPreamp 6 Bias           : {:.3}[V]", read_biases[5]);
    // println!("\tPreamp 7 Bias           : {:.3}[V]", read_biases[6]);
    // println!("\tPreamp 8 Bias           : {:.3}[V]", read_biases[7]);
    // println!("\tPreamp 9 Bias           : {:.3}[V]", read_biases[8]);
    // println!("\tPreamp 10 Bias          : {:.3}[V]", read_biases[9]);
    // println!("\tPreamp 11 Bias          : {:.3}[V]", read_biases[10]);
    // println!("\tPreamp 12 Bias          : {:.3}[V]", read_biases[11]);
    // println!("\tPreamp 13 Bias          : {:.3}[V]", read_biases[12]);
    // println!("\tPreamp 14 Bias          : {:.3}[V]", read_biases[13]);
    // println!("\tPreamp 15 Bias          : {:.3}[V]", read_biases[14]);
    // println!("\tPreamp 16 Bias          : {:.3}[V]", read_biases[15]);
}

fn print_all() {
    let sipm_temps = PreampTemp::read_temp().unwrap().preamp_temps;
    let read_bias_voltages = PreampReadBias::read_bias().unwrap().read_biases;
    let set_bias_voltages = PreampSetBias::read_set_bias().unwrap().set_biases;

    println!("{:<13}{:<12}{:<12}{}", "TEMP", "SET", "READ", "DELTA");
    // for i in 0..=15 {
    //     println!("{:.3}[°C]   {:.3}[V]   {:.3}[V]", sipm_temps[i], set_bias_voltages[i], read_bias_voltages[i]);
    // }
    for i in [0, 3, 7, 8, 11, 14] {
        let delta = (set_bias_voltages[i] - read_bias_voltages[i]).abs();
        println!("{:.3}[°C]   {:.3}[V]   {:.3}[V]   {:.3}[V]", sipm_temps[i], set_bias_voltages[i], read_bias_voltages[i], delta);
    }
}

fn set_default_bias() {
    match PreampSetBias::set_default_bias() {
        Ok(_) => {},
        Err(e) => eprintln!("{:?}", e),
    }
}

fn set_bias() {
    // match preamp_bias::sipm_temp_comp(ch) {
    //     Ok(b) => println!("{}[V]", b),
    //     Err(e) => eprintln!("{:?}", e),
    // }
    match PreampSetBias::set_bias() {
        Ok(_) => {},
        Err(e) => eprintln!("{:?}", e),
    }
}

fn set_manual_bias(channel: Option<u8>, bias: f32) {
    match PreampSetBias::set_manual_bias(channel, bias) {
        Ok(_) => {},
        Err(e) => eprintln!("{:?}", e),
    }
}

fn reset_bias() {
    match PreampSetBias::reset_bias() {
        Ok(_) => {},
        Err(e) => eprintln!("{:?}", e),
     }
}

fn check_i2c(bus: u8, address: u16) -> Result<(), LinuxI2CError> {
    let mut dev = LinuxI2CDevice::new(&format!("/dev/i2c-{}", bus), address)?;
    dev.smbus_read_byte()?;

    Ok(())
}