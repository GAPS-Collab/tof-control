#[derive(Debug)]
pub struct PreampMoniData {
    // Preamp Temperature Sensors
    pub temperature: PreampTemp,
    // Preamp Bias Voltages
    pub bias: PreampBias,
}

#[derive(Debug)]
pub struct PreampTemp {
    pub preamp_temps: [f32; 16],
}

#[derive(Debug)]
pub struct PreampBias {
    pub preamp_biases: [f32; 16],
}

/// Preamp Error Type
#[derive(Debug)]
pub enum PreampError {
    /// Temp Error
    Temp(PreampTempError),
    /// Bias Error
    Bias(PreampBiasError),
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
}

impl From<i2cdev::linux::LinuxI2CError> for PreampBiasError {
    fn from(e: i2cdev::linux::LinuxI2CError) -> Self {
        PreampBiasError::I2C(e)
    }
}