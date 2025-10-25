#[derive(Debug)]
pub struct CPCTemp {
    pub cpc_temp: f32,
}

#[derive(Debug)]
pub struct CPCVcp {
    pub cpc_vcp: [f32; 3],
}

#[derive(Debug)]
pub enum CPCError {
    /// I2C Error
    I2C(i2cdev::linux::LinuxI2CError),
}

impl From<i2cdev::linux::LinuxI2CError> for CPCError {
    fn from(e: i2cdev::linux::LinuxI2CError) -> Self {
        CPCError::I2C(e)
    }
}