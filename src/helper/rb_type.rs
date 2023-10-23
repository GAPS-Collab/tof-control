#[derive(Debug)]
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

#[derive(Debug)]
pub struct RBInfo {
    pub board_id        : u8,
    pub lol             : u8,
    pub lol_stable      : u8,
    pub trig_rate       : u16,
    // Additional Info
    pub fw_version      : String,
    pub readout_mask    : u16,
}

#[derive(Debug)]
pub struct RBTemp {
    pub zynq_temp       : f32,
    pub drs_temp        : f32,
    pub clk_temp        : f32,
    pub adc_temp        : f32,
}

#[derive(Debug)]
pub struct RBVcp {
    pub zynq_vcp        : [f32; 3],
    pub p3v3_vcp        : [f32; 3],
    pub p3v5_vcp        : [f32; 3],
    pub n1v5_vcp        : [f32; 3],
    pub drs_dvdd_vcp    : [f32; 3],
    pub drs_avdd_vcp    : [f32; 3],
    pub adc_dvdd_vcp    : [f32; 3],
    pub adc_avdd_vcp    : [f32; 3],
}

/// RB Error Type
#[derive(Debug)]
pub enum RBError {
    Info(RBInfoError),
    Clk(RBClkError),
    Dac(RBDacError),
    GPIOe(RBGPIOeError),
    Temp(RBTempError),
    Vcp(RBVcpError),
    Ph(RBPhError),
    Mag(RBMagError),
}

impl From<RBInfoError> for RBError {
    fn from(e: RBInfoError) -> Self {
        RBError::Info(e)
    }
}

impl From<RBClkError> for RBError {
    fn from(e: RBClkError) -> Self {
        RBError::Clk(e)
    }
}

impl From<RBDacError> for RBError {
    fn from(e: RBDacError) -> Self {
        RBError::Dac(e)
    }
}

impl From<RBTempError> for RBError {
    fn from(e: RBTempError) -> Self {
        RBError::Temp(e)
    }
}

impl From<RBVcpError> for RBError {
    fn from(e: RBVcpError) -> Self {
        RBError::Vcp(e)
    }
}

impl From<RBPhError> for RBError {
    fn from(e: RBPhError) -> Self {
        RBError::Ph(e)
    }
}

impl From<RBMagError> for RBError {
    fn from(e: RBMagError) -> Self {
        RBError::Mag(e)
    }
}

#[derive(Debug)]
pub enum RBInfoError {
    // Register Error
    Register(crate::memory::RegisterError),
    // Parse Integer Error
    ParseInt(std::num::ParseIntError),
}

impl From<crate::memory::RegisterError> for RBInfoError {
    fn from(e: crate::memory::RegisterError) -> Self {
        RBInfoError::Register(e)
    }
}

impl From<std::num::ParseIntError> for RBInfoError {
    fn from(e: std::num::ParseIntError) -> Self {
        RBInfoError::ParseInt(e)
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
pub enum RBDacError {
    /// I2C Error
    I2C(i2cdev::linux::LinuxI2CError),
    // TryFrom Error
    // TryFrom(<u64 as TryFrom<u64>>::Error),
}

impl From<i2cdev::linux::LinuxI2CError> for RBDacError {
    fn from(e: i2cdev::linux::LinuxI2CError) -> Self {
        RBDacError::I2C(e)
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

#[derive(Debug)]
pub enum RBInitError {
    DAC(RBDacError),
    OsString,
    ParseInt(std::num::ParseIntError),
    Register(crate::memory::RegisterError),
    Temp(RBTempError),
    Vcp(RBVcpError),
    Ph(RBPhError),
    Mag(RBMagError),
}

impl std::fmt::Display for RBInitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RBInitError")
    }
}

impl From<RBDacError> for RBInitError {
    fn from(e: RBDacError) -> Self {
        RBInitError::DAC(e)
    }
}

impl From<std::ffi::OsString> for RBInitError {
    fn from(_: std::ffi::OsString) -> Self {
        RBInitError::OsString
    }
}

impl From<std::num::ParseIntError> for RBInitError {
    fn from(e: std::num::ParseIntError) -> Self {
        RBInitError::ParseInt(e)
    }
}

impl From<crate::memory::RegisterError> for RBInitError {
    fn from(e: crate::memory::RegisterError) -> Self {
        RBInitError::Register(e)
    }
}

impl From<RBTempError> for RBInitError {
    fn from(e: RBTempError) -> Self {
        RBInitError::Temp(e)
    }
}

impl From<RBVcpError> for RBInitError {
    fn from(e: RBVcpError) -> Self {
        RBInitError::Vcp(e)
    }
}

impl From<RBPhError> for RBInitError {
    fn from(e: RBPhError) -> Self {
        RBInitError::Ph(e)
    }
}

impl From<RBMagError> for RBInitError {
    fn from(e: RBMagError) -> Self {
        RBInitError::Mag(e)
    }
}

#[derive(Debug)]
pub enum RBTempError {
    // Register Error
    Register(crate::memory::RegisterError),
    /// I2C Error
    I2C(i2cdev::linux::LinuxI2CError),
}

impl From<crate::memory::RegisterError> for RBTempError {
    fn from(e: crate::memory::RegisterError) -> Self {
        RBTempError::Register(e)
    }
}

impl From<i2cdev::linux::LinuxI2CError> for RBTempError {
    fn from(e: i2cdev::linux::LinuxI2CError) -> Self {
        RBTempError::I2C(e)
    }
}

#[derive(Debug)]
pub enum RBVcpError {
    /// I2C Error
    I2C(i2cdev::linux::LinuxI2CError),
}

impl From<i2cdev::linux::LinuxI2CError> for RBVcpError {
    fn from(e: i2cdev::linux::LinuxI2CError) -> Self {
        RBVcpError::I2C(e)
    }
}

#[derive(Debug)]
pub enum RBPhError {
    /// I2C Error
    I2C(i2cdev::linux::LinuxI2CError),
}

impl From<i2cdev::linux::LinuxI2CError> for RBPhError {
    fn from(e: i2cdev::linux::LinuxI2CError) -> Self {
        RBPhError::I2C(e)
    }
}

#[derive(Debug)]
pub enum RBMagError {
    /// I2C Error
    I2C(i2cdev::linux::LinuxI2CError),
}

impl From<i2cdev::linux::LinuxI2CError> for RBMagError {
    fn from(e: i2cdev::linux::LinuxI2CError) -> Self {
        RBMagError::I2C(e)
    }
}