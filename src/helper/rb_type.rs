use crate::device::*;

pub struct RBMoniData {
    // RB Information
    pub board_id: u8,
    pub lol: bool,
    pub lol_stable: bool,
    pub trigger_rate: u16,
    // RB Temperature Sensor
    pub drs_temp: f32,
    pub clk_temp: f32,
    pub adc_temp: f32,
    pub lis3mdltr_temp: f32,
    pub bme280_temp: f32,
    pub zynq_temp: f32,
    // RB Pressure and Humidity Sensor
    pub pressure: f32,
    pub humidity: f32,
    // RB VCP (Voltage, Current and Power) Sensor
    pub drs_dvdd_voltage: f32,
    pub drs_dvdd_current: f32,
    pub drs_dvdd_power: f32,
    pub p3v3_voltage: f32,
    pub p3v3_current: f32,
    pub p3v3_power: f32,
    pub zynq_voltage: f32,
    pub zynq_current: f32,
    pub zynq_power: f32,
    pub p3v5_voltage: f32,
    pub p3v5_current: f32,
    pub p3v5_power: f32,
    pub adc_dvdd_voltage: f32,
    pub adc_dvdd_current: f32,
    pub adc_dvdd_power: f32,
    pub adc_avdd_voltage: f32,
    pub adc_avdd_current: f32,
    pub adc_avdd_power: f32,
    pub drs_avdd_voltage: f32,
    pub drs_avdd_current: f32,
    pub drs_avdd_power: f32,
    pub n1v5_voltage: f32,
    pub n1v5_current: f32,
    pub n1v5_power: f32,
    // RB Magnetic Sensor
    pub mag_x: f32,
    pub mag_y: f32,
    pub mag_z: f32,
    pub mag_t: f32,
}

/// RB Error Type
#[derive(Debug)]
pub enum RBError {
    Clk(RBClkError),
    GPIOe(RBGPIOeError),
}

impl From<RBClkError> for RBError {
    fn from(e: RBClkError) -> Self {
        RBError::Clk(e)
    }
}

#[derive(Debug)]
pub enum RBClkError {
    /// I2C Error
    I2C(i2cdev::linux::LinuxI2CError),
}

impl From<i2cdev::linux::LinuxI2CError> for RBClkError {
    fn from(e: i2cdev::linux::LinuxI2CError) -> Self {
        RBClkError::I2C(e)
    }
}

#[derive(Debug)]
pub enum RBGPIOeError {
    /// I2C Error
    I2C(i2cdev::linux::LinuxI2CError),
}

impl From<i2cdev::linux::LinuxI2CError> for RBGPIOeError {
    fn from(e: i2cdev::linux::LinuxI2CError) -> Self {
        RBGPIOeError::I2C(e)
    }
}