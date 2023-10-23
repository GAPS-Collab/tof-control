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
    pub p3v6_preamp_vcp:    [f32; 3],
    pub n1v6_preamp_vcp:    [f32; 3],
    pub p3v4f_ltb_vcp:      [f32; 3],
    pub p3v4d_ltb_vcp:      [f32; 3],
    pub p3v6_ltb_vcp:       [f32; 3],
    pub n1v6_ltb_vcp:       [f32; 3],
}

/// PB Error Type
#[derive(Debug)]
pub enum PBError {
    /// Init Error
    Init(PBInitError),
    /// Temp Error
    Temp(PBTempError),
    /// VCP (Voltage, Current and Power) Error
    Vcp(PBVcpError),
}

impl From<PBInitError> for PBError {
    fn from(e: PBInitError) -> Self {
        PBError::Init(e)
    }
}

impl From<PBTempError> for PBError {
    fn from(e: PBTempError) -> Self {
        PBError::Temp(e)
    }
}

impl From<PBVcpError> for PBError {
    fn from(e: PBVcpError) -> Self {
        PBError::Vcp(e)
    }
}

#[derive(Debug)]
pub enum PBInitError {
    /// Temp Error
    Temp(PBTempError),
    /// VCP (Voltage, Current and Power) Error
    Vcp(PBVcpError),
}

impl std::fmt::Display for PBInitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PBInitError")
    }
}

impl From<PBTempError> for PBInitError {
    fn from(e: PBTempError) -> Self {
        PBInitError::Temp(e)
    }
}

impl From<PBVcpError> for PBInitError {
    fn from(e: PBVcpError) -> Self {
        PBInitError::Vcp(e)
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
pub enum PBVcpError {
    /// I2C Error
    I2C(i2cdev::linux::LinuxI2CError),
}

impl From<i2cdev::linux::LinuxI2CError> for PBVcpError {
    fn from(e: i2cdev::linux::LinuxI2CError) -> Self {
        PBVcpError::I2C(e)
    }
}