use crate::constant::*;
use crate::helper::rb_type::RBError;
use crate::rb_control::{rb_dac, rb_input, rb_gpioe};

use std::thread::sleep;
use std::time::Duration;

pub fn select_noi_mode() -> Result<(), RBError> {
    rb_dac::dac_noi_mode()?;
    rb_input::disable_rf_input()?;

    while (verify_input_mode("NOI")?) == false {
        sleep(Duration::from_millis(10));
        select_noi_mode()?;
    }

    Ok(())
}

pub fn select_vcal_mode() -> Result<(), RBError> {
    rb_dac::dac_vcal_mode()?;
    rb_input::disable_rf_input()?;

    while (verify_input_mode("VCAL")?) == false {
        sleep(Duration::from_millis(10));
        select_noi_mode()?;
    }

    Ok(())
}

pub fn select_tcal_mode() -> Result<(), RBError> {
    rb_dac::dac_tcal_mode()?;
    rb_input::enable_tca_input()?;

    while (verify_input_mode("TCAL")?) == false {
        sleep(Duration::from_millis(10));
        select_noi_mode()?;
    }

    Ok(())
}

pub fn select_sma_mode() -> Result<(), RBError> {
    rb_dac::dac_sma_mode()?;
    println!("DAC_SMA_MODE");
    rb_input::enable_sma_input()?;
    println!("RB INPUT SMA MODE");
    while (verify_input_mode("SMA")?) == false {
        sleep(Duration::from_millis(10));
        println!("SLEEPING");
        select_noi_mode()?;
        println!("NOI MODE SELECTED");
    }

    Ok(())
}

pub fn read_input_mode() -> Result<String, RBError> {
    
    let mut rf_input_ports: [u8; 9] = Default::default();
    for i in 0..9 {
        let rf_input_port = rb_gpioe::read_rf_input_port(i+1 as u8)?;
        rf_input_ports[i as usize] = rf_input_port;
    }

    let input_mode: &str;
    match rf_input_ports {
        [0, 0, 0, 0, 0, 0, 0, 0, 0] => {
            input_mode = "SMA";
        }
        [1, 1, 1, 1, 1, 1, 1, 1, 1] => {
            input_mode = "TCAL";
        }
        [3, 3, 3, 3, 3, 3, 3, 3, 3] => {
            input_mode = "NOI";
        }
        _ => {
            input_mode = "Input Mode Error";
        }
    }

    Ok(input_mode.to_string())
}

pub fn verify_input_mode(mode: &str) -> Result<bool, RBError> {
    let expected_input_mode: String = mode.to_uppercase();

    let current_input_mode = read_input_mode()?;
    let current_input_dac = rb_dac::read_single_dac(1)?;

    let mut mode_bool: bool = false;

    match expected_input_mode.as_str() {
        "NOI" => {
            if current_input_mode == "NOI" && current_input_dac == RB_AD5675_DAC1 {
                mode_bool = true;
            }
        }
        "VCAL" => {
            if current_input_mode == "NOI" && current_input_dac == RB_AD5675_DAC1_VCAL {
                mode_bool = true;
            }
        }
        "TCAL" => {
            if current_input_mode == "TCAL" && current_input_dac == RB_AD5675_DAC1 {
                mode_bool = true;
            }
        }
        "SMA" => {
            if current_input_mode == "SMA" && current_input_dac == RB_AD5675_DAC1 {
                mode_bool = true;
            }
        }
        _ => {
            return Err(RBError::InvalidInputMode);
        }
    }

    return Ok(mode_bool);
}
