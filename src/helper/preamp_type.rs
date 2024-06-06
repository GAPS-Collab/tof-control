use serde::{Deserialize, Serialize};

/// PA Data Type
// All PA Monitoring Types
#[derive(Debug, Serialize, Deserialize)]
pub struct PAMoniData {
    pub pa_temp: PATemp,
    pub pa_bias: PABias,
}
// PA Temperature Data Type
#[derive(Debug, Serialize, Deserialize)]
pub struct PATemp {
    pub pa_temps: [f32; 16],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreampReadBias {
    pub read_biases: [f32; 16],
}

#[derive(Debug)]
pub struct PreampSetBias {
    pub set_biases: [f32; 16],
}

/// PA Error Type
#[derive(Debug)]
pub enum PAError {
    // I2C Error
    I2C(i2cdev::linux::LinuxI2CError),
    // /// Init Error
    // Init(PreampInitError),
    // /// Bias Error
    // Bias(PreampBiasError),
}

impl From<i2cdev::linux::LinuxI2CError> for PAError {
    fn from(e: i2cdev::linux::LinuxI2CError) -> Self {
        PAError::I2C(e)
    }
}









#[derive(Debug)]
pub enum PreampInitError {
    /// Bias Error
    Bias(PreampBiasError),
}

impl std::fmt::Display for PreampInitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PreampInitError")
    }
}

impl From<PreampBiasError> for PreampInitError {
    fn from(e: PreampBiasError) -> Self {
        PreampInitError::Bias(e)
    }
}


#[derive(Debug)]
pub enum PreampBiasError {
    /// I2C Error
    I2C(i2cdev::linux::LinuxI2CError),
    /// Temp Error
    Temp(PreampTempError),
    /// PB Temp Error
    PBTemp(crate::helper::pb_type::PBTempError),
}

impl From<i2cdev::linux::LinuxI2CError> for PreampBiasError {
    fn from(e: i2cdev::linux::LinuxI2CError) -> Self {
        PreampBiasError::I2C(e)
    }
}

impl From<PreampTempError> for PreampBiasError {
    fn from(e: PreampTempError) -> Self {
        PreampBiasError::Temp(e)
    }
}

impl From<crate::helper::pb_type::PBTempError> for PreampBiasError {
    fn from(e: crate::helper::pb_type::PBTempError) -> Self {
        PreampBiasError::PBTemp(e)
    }
}