use crate::ltb_control::{ltb_temp, ltb_threshold};
use crate::helper::ltb_type::LTBInitError;

pub fn initialize() -> Result<(), LTBInitError> {
    // Set Default Threshold Voltages
    initialize_threshold()?;
    // Initialize Temp Sensor
    initialize_temp()?;
    
    Ok(())
}

fn initialize_threshold() -> Result<(), LTBInitError> {
    ltb_threshold::set_default_threshold()?;

    Ok(())
}

fn initialize_temp() -> Result<(), LTBInitError> {
    // Configure Temp Sensors (TMP112)
    ltb_temp::config_temp()?;

    Ok(())
}