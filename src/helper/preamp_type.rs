#[derive(Debug)]
pub struct PreampMoniData {
    // Preamp Temperature Sensors
    pub temperature: PreampTemp,
    // Preamp Read Bias Voltages
    pub read_bias: PreampReadBias,
}

#[derive(Debug)]
pub struct PreampTemp {
    pub preamp_temps: [f32; 16],
}

#[derive(Debug)]
pub struct PreampReadBias {
    pub read_biases: [f32; 16],
}

#[derive(Debug)]
pub struct PreampSetBias {
    pub set_biases: [f32; 16],
}

/// Preamp Error Type
#[derive(Debug)]
pub enum PreampError {
    /// Init Error
    Init(PreampInitError),
    /// Temp Error
    Temp(PreampTempError),
    /// Bias Error
    Bias(PreampBiasError),
}

impl From<PreampInitError> for PreampError {
    fn from(e: PreampInitError) -> Self {
        PreampError::Init(e)
    }
}

impl From<PreampTempError> for PreampError {
    fn from(e: PreampTempError) -> Self {
        PreampError::Temp(e)
    }
}

impl From<PreampBiasError> for PreampError {
    fn from(e: PreampBiasError) -> Self {
        PreampError::Bias(e)
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
pub enum PreampTempError {
    /// I2C Error
    I2C(i2cdev::linux::LinuxI2CError),
}

impl From<i2cdev::linux::LinuxI2CError> for PreampTempError {
    fn from(e: i2cdev::linux::LinuxI2CError) -> Self {
        PreampTempError::I2C(e)
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