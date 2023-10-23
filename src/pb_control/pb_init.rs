use crate::helper::pb_type::PBInitError;
use crate::pb_control::{pb_temp, pb_vcp};

pub fn initialize() -> Result<(), PBInitError> {
    // Initialize Temp Sensor
    initialize_temp()?;
    // Initialize VCP Sensor
    initialize_vcp()?;

    Ok(())
}

fn initialize_temp() -> Result<(), PBInitError> {
    pb_temp::config_temp()?;

    Ok(())
}

fn initialize_vcp() -> Result<(), PBInitError> {
    pb_vcp::config_vcp()?;

    Ok(())
}