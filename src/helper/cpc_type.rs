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
    // IO Error
    IO(std::io::Error),
}

impl From<i2cdev::linux::LinuxI2CError> for CPCError {
    fn from(e: i2cdev::linux::LinuxI2CError) -> Self {
        CPCError::I2C(e)
    }
}

impl From<std::io::Error> for CPCError {
    fn from(e: std::io::Error) -> Self {
        CPCError::IO(e)
    }
}