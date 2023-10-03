use i2cdev::linux::LinuxI2CError;
use std::process::exit;

use tof_control::constant::*;
use tof_control::device::{cy8c9560a, pca9548a, si5345b};

fn main() {
    let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
    i2c_mux.select(RB_CY8C9560A_CHANNEL).unwrap_or_else(|e| {
        eprintln!("PCA9548A Programming Error: {}", e);
        exit(1);
    });

    enable_si5345b().unwrap_or_else(|e| {
        eprintln!("CY8C9560A Programming Error: {}", e);
        exit(1);
    });

    i2c_mux.select(RB_SI5345B_CHANNEL).unwrap_or_else(|e| {
        eprintln!("PCA9548A Programming Error: {}", e);
        exit(1);
    });

    program_nvm_si5345b().unwrap_or_else(|e| {
        eprintln!("SI5345B Programming Error: {}", e);
        exit(1);
    });

    i2c_mux.reset().unwrap_or_else(|e| {
        eprintln!("Cannot Reset PCA9548A: {}", e);
        exit(1);
    });

    println!("Successfully written clock configuration to SI5345B NVM!");
}

fn enable_si5345b() -> Result<(), LinuxI2CError> {
    let cy8c9560a = cy8c9560a::CY8C9560A::new(I2C_BUS, RB_CY8C9560A_ADDRESS);
    let mut value = cy8c9560a.read_port_status(3)?;
    value = (value & !0x02) | 0 << 1;
    cy8c9560a.set_output_port(3, value)?;

    Ok(())
}

fn program_nvm_si5345b() -> Result<(), LinuxI2CError> {
    let si5345b = si5345b::SI5345B::new(I2C_BUS, RB_SI5345B_ADDRESS);
    si5345b.configure_nvm_si5345b()?;

    Ok(())
}
