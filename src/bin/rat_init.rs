use std::fs::OpenOptions;
use std::io::prelude::*;
use clap::Parser;
use chrono::Utc;
use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

use tof_control::constant::{I2C_BUS, LTB_TRENZ_ADDRESS, PB_PCA9548A_ADDRESS};
use tof_control::rb_control::rb_init;
use tof_control::ltb_control::ltb_init;
use tof_control::pb_control::pb_init;
use tof_control::pa_control::pa_init;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {}

fn main() {
    let _args = Args::parse();

    // Initialize RB
    initialize_rb();

    // Initialize LTB if available
    if !check_i2c(I2C_BUS, LTB_TRENZ_ADDRESS).is_err() {
        initialize_ltb();
    }

    // Initialize PB and Preamp if available
    if !check_i2c(I2C_BUS, PB_PCA9548A_ADDRESS).is_err() {
        initialize_pb();
        initialize_preamp();
    }
}

fn initialize_rb() {
    match rb_init::initialize() {
        Ok(_) => {},
        Err(e) => {
            write_err_log(e.to_string()).unwrap();
            std::process::exit(1);
        }
    }
}

fn initialize_ltb() {
    match ltb_init::initialize() {
        Ok(_) => {},
        Err(e) => {
            write_err_log(e.to_string()).unwrap();
            std::process::exit(1);
        }
    }
}

fn initialize_pb() {
    match pb_init::initialize() {
        Ok(_) => {},
        Err(e) => {
            write_err_log(e.to_string()).unwrap();
            std::process::exit(1);
        }
    }
}

fn initialize_preamp() {
    match pa_init::initialize() {
        Ok(_) => {},
        Err(e) => {
            write_err_log(e.to_string()).unwrap();
            std::process::exit(1);
        }
    }
}

fn write_err_log(error: String) -> Result<(), std::io::Error> {

    let now = Utc::now().to_rfc3339();
    let err_msg = format!("{}: {}", now, error);
    
    let log_path = "/home/gaps/log/rat-init.log";
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
        ?;

    writeln!(file, "{}", err_msg)?;

    Ok(())
}

fn check_i2c(bus: u8, address: u16) -> Result<(), LinuxI2CError> {
    let mut dev = LinuxI2CDevice::new(&format!("/dev/i2c-{}", bus), address)?;
    dev.smbus_read_byte()?;

    Ok(())
}