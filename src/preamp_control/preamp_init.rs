use crate::helper::preamp_type::PreampInitError;
use crate::preamp_control::preamp_bias;

pub fn initialize() -> Result<(), PreampInitError> {
    initialize_bias()?;

    Ok(())
}

fn initialize_bias() -> Result<(), PreampInitError> {
    preamp_bias::set_default_bias()?;

    Ok(())
}