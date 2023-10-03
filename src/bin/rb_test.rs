#![allow(unused)]
use tof_control::constant::*;
use tof_control::device::{cy8c9560a, pca9548a, si5345b};
use tof_control::memory::*;

use tof_control::rb_control::rb_gpioe;

use i2cdev::linux::LinuxI2CError;

fn main() {
    // match enable_clk_output() {
    //     Ok(_) => (),
    //     Err(e) => {
    //         eprintln!("Error enabling Si5345B output: {:?}", e);
    //         std::process::exit(1);
    //     },
    // }

    // match read_temp() {
    //     Ok(zynq_temp) => println!("ZYNQ:    {:.3}[°C]", zynq_temp),
    //     Err(e) =>{
    //         eprintln!("ZYNQ temperature readout error: {:?}", e);
    //         std::process::exit(1);
    //     },
    // }

    // match enable_i2c_cy8c9560a() {
    //     Ok(_) => (),
    //     Err(e) => {
    //         eprintln!("Error enabling CY8C9560A chip: {:?}", e);
    //         std::process::exit(1);
    //     },
    // }
    // check_enable_register_cy8c9560a();
    // enable_eeprom_cy8c9560a();
    // check_enable_register_cy8c9560a();

    // store_config_eeprom_por_cy8c9560a();

    // reset_config_eeprom_por_cy8c9560a();

    // match initialize_cy8c9560a() {
    //     Ok(_) => (),
    //     Err(e) => {
    //         eprintln!("Error initializing CY8C9560A chip: {:?}", e);
    //         std::process::exit(1);
    //     },
    // }

    match enable_tcal_input() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error enabling TCAL input on CY8C9560A chip: {:?}", e);
            std::process::exit(1);
        }
    }
}

fn read_temp() -> Result<f32, RegisterError> {
    let zynq_temp_adc = read_control_reg(RB_TEMP)?;
    let zynq_temp = (((zynq_temp_adc & 4095) as f32 * 503.975) / 4096.0) - 273.15;

    Ok(zynq_temp)
}

fn enable_clk_output() -> Result<(), LinuxI2CError> {
    let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
    i2c_mux.select(RB_CY8C9560A_CHANNEL)?;
    let cy8c9560a = cy8c9560a::CY8C9560A::new(I2C_BUS, RB_CY8C9560A_ADDRESS);
    let mut value = cy8c9560a.read_port_status(3)?;
    value = (value & !0x02) | 0 << 1;
    cy8c9560a.set_output_port(3, value)?;
    i2c_mux.reset()?;

    Ok(())
}

fn enable_i2c_cy8c9560a() -> Result<(), LinuxI2CError> {
    let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
    i2c_mux.select(RB_CY8C9560A_CHANNEL)?;

    Ok(())
}

fn check_enable_register_cy8c9560a() {
    enable_i2c_cy8c9560a();
    let cy8c9560a = cy8c9560a::CY8C9560A::new(I2C_BUS, RB_CY8C9560A_ADDRESS);
    let value = cy8c9560a.read_enable_register();

    match value {
        Ok(enable_register) => println!("Enable Register: {:08b}", enable_register),
        Err(e) => {
            eprintln!("Error reading CY8C9560A enable register: {:?}", e);
            std::process::exit(1);
        }
    }
}

fn enable_eeprom_cy8c9560a() {
    enable_i2c_cy8c9560a();
    let cy8c9560a = cy8c9560a::CY8C9560A::new(I2C_BUS, RB_CY8C9560A_ADDRESS);
    match cy8c9560a.enable_eeprom() {
        Ok(_) => println!("Successfuly enabled EEPROM on CY8C9560A"),
        Err(e) => {
            eprintln!("Error enabling EEPROM on CY8C9560A: {:?}", e);
            std::process::exit(1);
        }
    }
}

fn store_config_eeprom_por_cy8c9560a() {
    enable_i2c_cy8c9560a();
    let cy8c9560a = cy8c9560a::CY8C9560A::new(I2C_BUS, RB_CY8C9560A_ADDRESS);
    match cy8c9560a.store_config_eeprom_por() {
        Ok(_) => println!("Successfuly stored current configuration to EEPROM on CY8C9560A"),
        Err(e) => {
            eprintln!(
                "Error storing current configuration to EEPROM on CY8C9560A: {:?}",
                e
            );
            std::process::exit(1);
        }
    }
}

fn reset_config_eeprom_por_cy8c9560a() {
    enable_i2c_cy8c9560a();
    let cy8c9560a = cy8c9560a::CY8C9560A::new(I2C_BUS, RB_CY8C9560A_ADDRESS);
    match cy8c9560a.reset_config_eeprom_por() {
        Ok(_) => println!("Successfuly reset configuration to EEPROM on CY8C9560A"),
        Err(e) => {
            eprintln!(
                "Error resetting configuration to EEPROM on CY8C9560A: {:?}",
                e
            );
            std::process::exit(1);
        }
    }
}

fn initialize_cy8c9560a() -> Result<(), LinuxI2CError> {
    enable_i2c_cy8c9560a();
    let cy8c9560a = cy8c9560a::CY8C9560A::new(I2C_BUS, RB_CY8C9560A_ADDRESS);
    // Port 0
    cy8c9560a.select_port(0)?;
    cy8c9560a.set_interrupt_mask_port(0x00)?;
    cy8c9560a.set_pin_direction(0x00)?;
    cy8c9560a.set_drive_mode(4)?;

    // Port 1
    cy8c9560a.select_port(1)?;
    cy8c9560a.set_interrupt_mask_port(0x00)?;
    cy8c9560a.set_pin_direction(0x00)?;
    cy8c9560a.set_drive_mode(1)?;

    // Port 2
    cy8c9560a.select_port(2)?;
    cy8c9560a.set_interrupt_mask_port(0x00)?;
    cy8c9560a.set_pin_direction(0x00)?;
    cy8c9560a.set_drive_mode(4)?;

    // Port 3
    cy8c9560a.select_port(3)?;
    cy8c9560a.set_interrupt_mask_port(0x00)?;
    cy8c9560a.set_pin_direction(0x00)?;
    cy8c9560a.set_drive_mode(4)?;

    // Port 4
    cy8c9560a.select_port(4)?;
    cy8c9560a.set_interrupt_mask_port(0x00)?;
    cy8c9560a.set_pin_direction(0x00)?;
    cy8c9560a.set_drive_mode(4)?;

    // Port 5
    cy8c9560a.select_port(5)?;
    cy8c9560a.set_interrupt_mask_port(0x00)?;
    cy8c9560a.set_pin_direction(0x00)?;
    cy8c9560a.set_drive_mode(4)?;

    // Port 6
    cy8c9560a.select_port(6)?;
    cy8c9560a.set_interrupt_mask_port(0x00)?;
    cy8c9560a.set_pin_direction(0x00)?;
    cy8c9560a.set_drive_mode(1)?;

    // Port 7
    cy8c9560a.select_port(7)?;
    cy8c9560a.set_interrupt_mask_port(0x00)?;
    cy8c9560a.set_pin_direction(0x00)?;
    cy8c9560a.set_drive_mode(4)?;

    // Set ouput ports
    cy8c9560a.set_output_port(0, 0x00)?;
    cy8c9560a.set_output_port(1, 0xFF)?;
    cy8c9560a.set_output_port(2, 0x03)?;
    cy8c9560a.set_output_port(3, 0x13)?;
    cy8c9560a.set_output_port(4, 0xFC)?;
    cy8c9560a.set_output_port(5, 0x33)?;
    cy8c9560a.set_output_port(6, 0xFF)?;
    cy8c9560a.set_output_port(7, 0x3F)?;

    Ok(())
}

fn enable_tcal_input() -> Result<(), LinuxI2CError> {
    rb_gpioe::enable_nb3v9312c();
    rb_gpioe::rf_input_select(1);

    Ok(())
}
