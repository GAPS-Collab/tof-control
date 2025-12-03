#[derive(Debug)]
pub struct TCPCTemp {
    pub tcpc_temp: f32,
}

#[derive(Debug)]
pub struct TCPCVcp {
    pub tcpc_vcp: [f32; 3],
}

pub enum TCPCError {
    /// I2C Error
    I2C(i2cdev::linux::LinuxI2CError),
    // IO Error
    IO(std::io::Error),
}

impl From<i2cdev::linux::LinuxI2CError> for TCPCError {
    fn from(e: i2cdev::linux::LinuxI2CError) -> Self {
        TCPCError::I2C(e)
    }
}

impl From<std::io::Error> for TCPCError {
    fn from(e: std::io::Error) -> Self {
        TCPCError::IO(e)
    }
}