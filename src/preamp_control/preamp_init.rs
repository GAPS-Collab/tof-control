use crate::helper::preamp_type::{PreampSetBias, PreampInitError};

pub fn initialize() -> Result<(), PreampInitError> {
    initialize_bias()?;

    Ok(())
}

fn initialize_bias() -> Result<(), PreampInitError> {
    PreampSetBias::set_bias()?;

    Ok(())
}