/// PB Data Type
#[derive(Debug)]
pub struct PBMonData {
    pub temperature: PBTemp,
    pub vcp: PBVcp,
}

// PB Temperature Sensor
#[derive(Debug)]
pub struct PBTemp {
    pub pds_temp: f32,
    pub pas_temp: f32,
    pub nas_temp: f32,
    pub shv_temp: f32,
}

// PB VCP (Voltage, Current and Power) Sensor
#[derive(Debug)]
pub struct PBVcp {
    pub p3v6_preamp_voltage: f32,
    pub p3v6_preamp_current: f32,
    pub p3v6_preamp_power: f32,
    pub n1v6_preamp_voltage: f32,
    pub n1v6_preamp_current: f32,
    pub n1v6_preamp_power: f32,
    pub p3v4f_ltb_voltage: f32,
    pub p3v4f_ltb_current: f32,
    pub p3v4f_ltb_power: f32,
    pub p3v4d_ltb_voltage: f32,
    pub p3v4d_ltb_current: f32,
    pub p3v4d_ltb_power: f32,
    pub p3v6_ltb_voltage: f32,
    pub p3v6_ltb_current: f32,
    pub p3v6_ltb_power: f32,
    pub n1v6_ltb_voltage: f32,
    pub n1v6_ltb_current: f32,
    pub n1v6_ltb_power: f32,
}

/// PB Error Type
#[derive(Debug)]
pub enum PBError {
    /// Temp Error
    Temp(PBTempError),
    /// VCP (Voltage, Current and Power) Error
    Vpc(PBVpcError),
}

impl From<PBTempError> for PBError {
    fn from(e: PBTempError) -> Self {
        PBError::Temp(e)
    }
}

impl From<PBVpcError> for PBError {
    fn from(e: PBVpcError) -> Self {
        PBError::Vpc(e)
    }
}

#[derive(Debug)]
pub enum PBTempError {
    /// I2C Error
    I2C(i2cdev::linux::LinuxI2CError),
}

impl From<i2cdev::linux::LinuxI2CError> for PBTempError {
    fn from(e: i2cdev::linux::LinuxI2CError) -> Self {
        PBTempError::I2C(e)
    }
}

#[derive(Debug)]
pub enum PBVpcError {
    /// I2C Error
    I2C(i2cdev::linux::LinuxI2CError),
}

impl From<i2cdev::linux::LinuxI2CError> for PBVpcError {
    fn from(e: i2cdev::linux::LinuxI2CError) -> Self {
        PBVpcError::I2C(e)
    }
}