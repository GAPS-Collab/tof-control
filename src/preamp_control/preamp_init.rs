use crate::helper::preamp_type::{PreampSetBias, PreampInitError};

pub fn initialize() -> Result<(), PreampInitError> {
    // initialize_bias()?;
    initialize_bias_manual()?;

    Ok(())
}

// fn initialize_bias() -> Result<(), PreampInitError> {
//     PreampSetBias::set_bias()?;

//     Ok(())
// }

fn initialize_bias_manual() -> Result<(), PreampInitError> {
    PreampSetBias::set_manual_bias(None, 58.0)?;

    Ok(())
}