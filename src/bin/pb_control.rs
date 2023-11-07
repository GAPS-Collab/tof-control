use clap::{Parser, Subcommand, ValueEnum};
use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

use tof_control::constant::{I2C_BUS, PB_PCA9548A_ADDRESS};
use tof_control::pb_control::pb_ltb_pwr;
use tof_control::helper::pb_type::{PBTemp, PBVcp};

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
    #[clap(short_flag = 'l')]
    Ltb {
        #[arg(ignore_case = true, value_enum)]
        switch: Option<Switch>,
    }
}

#[derive(ValueEnum, Clone, Debug)]
enum Sensor {
    Temp,
    Vcp,
}

#[derive(ValueEnum, Clone, Debug)]
enum Switch {
    On,
    Off,
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
                        Sensor::Vcp => {
                            print_vcp();
                        },
                    }
                },
                None => {
                    print_temp();
                    print_vcp();
                }
            }
        }
        Commands::Ltb { switch } => {
            match switch {
                Some(s) => {
                    match s{
                        Switch::On => {
                            ltb_pwr_on();
                        },
                        Switch::Off => {
                            ltb_pwr_off();
                        }
                    }
                },
                None => {
                    println!("Error");
                }
            }
        }
    }
}

fn check_i2c(bus: u8, address: u16) -> Result<(), LinuxI2CError> {
    let mut dev = LinuxI2CDevice::new(&format!("/dev/i2c-{}", bus), address)?;
    dev.smbus_read_byte()?;

    Ok(())
}

fn print_temp() {
    let temperature = PBTemp::new();

    println!("PB Temperature");
    println!("\tPDS Temperature     : {:.3}[°C]", temperature.pds_temp);
    println!("\tPAS Temperature     : {:.3}[°C]", temperature.pas_temp);
    println!("\tNAS Temperature     : {:.3}[°C]", temperature.nas_temp);
    println!("\tSHV Temperature     : {:.3}[°C]", temperature.shv_temp);
}

fn print_vcp() {
    let pb_vcp = PBVcp::new();

    println!("PB VCP (Voltage, Current and Power)");
    println!("\tPreamp 3.6V VCP     : {:.3}[V] | {:.3}[A] | {:.3}[W]", pb_vcp.p3v6_preamp_vcp[0], pb_vcp.p3v6_preamp_vcp[1], pb_vcp.p3v6_preamp_vcp[2]);
    println!("\tPreamp -1.6V VCP    : {:.2}[V] | {:.3}[A] | {:.3}[W]", pb_vcp.n1v6_preamp_vcp[0], pb_vcp.n1v6_preamp_vcp[1], pb_vcp.n1v6_preamp_vcp[2]);
    println!("\tLTB Trenz VCP       : {:.3}[V] | {:.3}[A] | {:.3}[W]", pb_vcp.p3v4f_ltb_vcp[0], pb_vcp.p3v4f_ltb_vcp[1], pb_vcp.p3v4f_ltb_vcp[2]);
    println!("\tLTB 3.4V VCP        : {:.3}[V] | {:.3}[A] | {:.3}[W]", pb_vcp.p3v4d_ltb_vcp[0], pb_vcp.p3v4d_ltb_vcp[1], pb_vcp.p3v4d_ltb_vcp[2]);
    println!("\tLTB 3.6V VCP        : {:.3}[V] | {:.3}[A] | {:.3}[W]", pb_vcp.p3v6_ltb_vcp[0], pb_vcp.p3v6_ltb_vcp[1], pb_vcp.p3v6_ltb_vcp[2]);
    println!("\tLTB -1.6V VCP       : {:.2}[V] | {:.3}[A] | {:.3}[W]", pb_vcp.n1v6_ltb_vcp[0], pb_vcp.n1v6_ltb_vcp[1], pb_vcp.n1v6_ltb_vcp[2]);
}

fn ltb_pwr_on() {
    match pb_ltb_pwr::ltb_pwr_switch(true) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{:?}", e);
        },
    }
}

fn ltb_pwr_off() {
    match pb_ltb_pwr::ltb_pwr_switch(false) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{:?}", e);
        },
    }
}