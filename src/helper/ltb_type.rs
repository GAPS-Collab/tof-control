use std::{num::{ParseFloatError, ParseIntError}, string::ParseError};

#[derive(Debug)]
pub struct LTBMoniData {
    pub temperature: LTBTemp,
    pub threshold: LTBThreshold,
}

// LTB Temperature Sensor
#[derive(Debug)]
pub struct LTBTemp {
    pub trenz_temp: f32,
    pub ltb_temp: f32,
}

// LTB Threshold Voltages
#[derive(Debug)]
pub struct LTBThreshold {
    pub threshold_0: f32,
    pub threshold_1: f32,
    pub threshold_2: f32,
}

/// LTB Error Type
#[derive(Debug)]
pub enum LTBError {
    /// Temp Error
    Temp(LTBTempError),
    /// Threshold Error
    Threshold(LTBThresholdError),
}

impl From<LTBTempError> for LTBError {
    fn from(e: LTBTempError) -> Self {
        LTBError::Temp(e)
    }
}

impl From<LTBThresholdError> for LTBError {
    fn from(e: LTBThresholdError) -> Self {
        LTBError::Threshold(e)
    }
}

#[derive(Debug)]
pub enum LTBTempError {
    /// I2C Error
    I2C(i2cdev::linux::LinuxI2CError),
}

#[derive(Debug)]
pub enum LTBThresholdError {
    /// I2C Error
    I2C(i2cdev::linux::LinuxI2CError),
}

impl From<i2cdev::linux::LinuxI2CError> for LTBTempError {
    fn from(e: i2cdev::linux::LinuxI2CError) -> Self {
        LTBTempError::I2C(e)
    }
}

impl From<i2cdev::linux::LinuxI2CError> for LTBThresholdError {
    fn from(e: i2cdev::linux::LinuxI2CError) -> Self {
        LTBThresholdError::I2C(e)
    }
}