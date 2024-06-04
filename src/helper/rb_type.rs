use serde::{Deserialize, Serialize};

/// RB Data Type
// RB Temperature Sensor Data Type
#[derive(Debug, Serialize, Deserialize)]
pub struct RBTemp {
    pub zynq_temp       : f32,
    pub drs_temp        : f32,
    pub clk_temp        : f32,
    pub adc_temp        : f32,
    pub bme280_temp     : f32,
    pub lis3mdltr_temp  : f32,
}
// RB VCP (Voltage, Current and Power) Sensor Data Type
#[derive(Debug, Serialize, Deserialize)]
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
// RB HP (Humidity and Pressure) Sensor Data Type
#[derive(Debug, Serialize, Deserialize)]
pub struct RBPh {
    pub pressure        : f32,
    pub humidity        : f32,
}
// RB Magnetic Sensor Data Type
#[derive(Debug, Serialize, Deserialize)]
pub struct RBMag {
    pub mag_xyz         : [f32; 3],
}


/// RB Error Type
#[derive(Debug)]
pub enum RBError {
    // Register Error
    Register(crate::memory::RegisterError),
    // I2C Error
    I2C(i2cdev::linux::LinuxI2CError),
    // JSON Error
    JSON(serde_json::Error),
    // ParseInt Error
    ParseInt(std::num::ParseIntError),
    // OsString Error
    OsString,
}

impl std::fmt::Display for RBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RBError")
    }
}

impl From<crate::memory::RegisterError> for RBError {
    fn from(e: crate::memory::RegisterError) -> Self {
        RBError::Register(e)
    }
}

impl From<i2cdev::linux::LinuxI2CError> for RBError {
    fn from(e: i2cdev::linux::LinuxI2CError) -> Self {
        RBError::I2C(e)
    }
}

impl From<serde_json::Error> for RBError {
    fn from(e: serde_json::Error) -> Self {
        RBError::JSON(e)
    }
}

impl From<std::num::ParseIntError> for RBError {
    fn from(e: std::num::ParseIntError) -> Self {
        RBError::ParseInt(e)
    }
}

impl From<std::ffi::OsString> for RBError {
    fn from(_: std::ffi::OsString) -> Self {
        RBError::OsString
    }
}



#[derive(Debug)]
pub enum RBResetError {
    Clk(RBClkError),
}
impl std::fmt::Display for RBResetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RBResetError")
    }
}

impl From<RBClkError> for RBResetError {
    fn from(e: RBClkError) -> Self {
        RBResetError::Clk(e)
    }
}
// impl From<RBDacError> for RBResetError {
//     fn from(e: RBDacError) -> Self {
//         RBResetError::DAC(e)
//     }
// }
// impl From<RBTempError> for RBResetError {
//     fn from(e: RBTempError) -> Self {
//         RBResetError::Temp(e)
//     }
// }
// impl From<RBVcpError> for RBResetError {
//     fn from(e: RBVcpError) -> Self {
//         RBResetError::Vcp(e)
//     }
// }
// impl From<RBPhError> for RBResetError {
//     fn from(e: RBPhError) -> Self {
//         RBResetError::Ph(e)
//     }
// }
// impl From<RBMagError> for RBResetError {
//     fn from(e: RBMagError) -> Self {
//         RBResetError::Mag(e)
//     }
// }
















#[derive(Debug)]
pub struct RBLevel1 {
    pub zynq_temp: f32,
    pub clk_temp: f32,
    pub zynq_vc: [f32; 2],
}

// #[derive(Debug)]
// pub struct RBMoniData {
//     // RB Information
//     pub board_id: u8,
//     pub lol: bool,
//     pub lol_stable: bool,
//     pub trigger_rate: u16,
//     // RB Temperature Sensor
//     pub drs_temp: f32,
//     pub clk_temp: f32,
//     pub adc_temp: f32,
//     pub lis3mdltr_temp: f32,
//     pub bme280_temp: f32,
//     pub zynq_temp: f32,
//     // RB Pressure and Humidity Sensor
//     pub pressure: f32,
//     pub humidity: f32,
//     // RB VCP (Voltage, Current and Power) Sensor
//     pub drs_dvdd_voltage: f32,
//     pub drs_dvdd_current: f32,
//     pub drs_dvdd_power: f32,
//     pub p3v3_voltage: f32,
//     pub p3v3_current: f32,
//     pub p3v3_power: f32,
//     pub zynq_voltage: f32,
//     pub zynq_current: f32,
//     pub zynq_power: f32,
//     pub p3v5_voltage: f32,
//     pub p3v5_current: f32,
//     pub p3v5_power: f32,
//     pub adc_dvdd_voltage: f32,
//     pub adc_dvdd_current: f32,
//     pub adc_dvdd_power: f32,
//     pub adc_avdd_voltage: f32,
//     pub adc_avdd_current: f32,
//     pub adc_avdd_power: f32,
//     pub drs_avdd_voltage: f32,
//     pub drs_avdd_current: f32,
//     pub drs_avdd_power: f32,
//     pub n1v5_voltage: f32,
//     pub n1v5_current: f32,
//     pub n1v5_power: f32,
//     // RB Magnetic Sensor
//     pub mag_x: f32,
//     pub mag_y: f32,
//     pub mag_z: f32,
//     pub mag_t: f32,
// }

#[derive(Debug)]
pub struct RBInfo {
    pub board_id        : u8,
    pub sub_board       : u8,
    pub lol             : u8,
    pub lol_stable      : u8,
    pub trig_rate       : u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RBInfoDebug {
    pub board_id        : u8,
    pub sub_board       : u8,
    pub lol             : u8,
    pub lol_stable      : u8,
    // pub trig_rate       : u16,
    pub fw_version      : String,
    pub fw_hash         : String,
    pub uptime          : u32,
    // pub sd_usage        : u8,
    pub input_mode      : String,
    pub rat_num         : u8,
    pub rat_pos         : u8,
    pub rb_pos          : u8, 
}

/// RB Error Type

#[derive(Debug)]
pub enum RBLevel1Error {
    I2C(i2cdev::linux::LinuxI2CError),
    Register(crate::memory::RegisterError),
}

impl From<i2cdev::linux::LinuxI2CError> for RBLevel1Error {
    fn from(e: i2cdev::linux::LinuxI2CError) -> Self {
        RBLevel1Error::I2C(e)
    }
}

impl From<crate::memory::RegisterError> for RBLevel1Error {
    fn from(e: crate::memory::RegisterError) -> Self {
        RBLevel1Error::Register(e)
    }
}

// #[derive(Debug)]
// pub enum RBError {
//     Info(RBInfoError),
//     Clk(RBClkError),
//     Dac(RBDacError),
//     GPIOe(RBGPIOeError),
//     Temp(RBTempError),
//     Vcp(RBVcpError),
//     Ph(RBPhError),
//     Mag(RBMagError),
//     Input(RBInputError),
//     Mode(RBModeError),
// }

// impl From<RBInfoError> for RBError {
//     fn from(e: RBInfoError) -> Self {
//         RBError::Info(e)
//     }
// }

// impl From<RBClkError> for RBError {
//     fn from(e: RBClkError) -> Self {
//         RBError::Clk(e)
//     }
// }

// impl From<RBDacError> for RBError {
//     fn from(e: RBDacError) -> Self {
//         RBError::Dac(e)
//     }
// }

// impl From<RBTempError> for RBError {
//     fn from(e: RBTempError) -> Self {
//         RBError::Temp(e)
//     }
// }

// impl From<RBVcpError> for RBError {
//     fn from(e: RBVcpError) -> Self {
//         RBError::Vcp(e)
//     }
// }

// impl From<RBPhError> for RBError {
//     fn from(e: RBPhError) -> Self {
//         RBError::Ph(e)
//     }
// }

// impl From<RBMagError> for RBError {
//     fn from(e: RBMagError) -> Self {
//         RBError::Mag(e)
//     }
// }

// impl From<RBInputError> for RBError {
//     fn from(e: RBInputError) -> Self {
//         RBError::Input(e)
//     }
// }

// impl From<RBModeError> for RBError {
//     fn from(e: RBModeError) -> Self {
//         RBError::Mode(e)
//     }
// }

#[derive(Debug)]
pub enum RBInfoError {
    // Register Error
    Register(crate::memory::RegisterError),
    // I2C Error
    I2C(i2cdev::linux::LinuxI2CError),
    // Parse Integer Error
    ParseInt(std::num::ParseIntError),
    // GPIOe Error
    // GPIOe(RBGPIOeError),
    // Mode Error
    Mode(RBModeError),
}

impl From<crate::memory::RegisterError> for RBInfoError {
    fn from(e: crate::memory::RegisterError) -> Self {
        RBInfoError::Register(e)
    }
}

impl From<i2cdev::linux::LinuxI2CError> for RBInfoError {
    fn from(e: i2cdev::linux::LinuxI2CError) -> Self {
        RBInfoError::I2C(e)
    }
}

impl From<std::num::ParseIntError> for RBInfoError {
    fn from(e: std::num::ParseIntError) -> Self {
        RBInfoError::ParseInt(e)
    }
}

// impl From<RBGPIOeError> for RBInfoError {
//     fn from(e: RBGPIOeError) -> Self {
//         RBInfoError::GPIOe(e)
//     }
// }

impl From<RBModeError> for RBInfoError {
    fn from(e: RBModeError) -> Self {
        RBInfoError::Mode(e)
    }
}


#[derive(Debug)]
pub enum RBInputError {
    /// I2C Error
    I2C(i2cdev::linux::LinuxI2CError),
    // RB GPIOe Error
    // GPIOe(RBGPIOeError),
}

impl From<i2cdev::linux::LinuxI2CError> for RBInputError {
    fn from(e: i2cdev::linux::LinuxI2CError) -> Self {
        RBInputError::I2C(e)
    }
}

// impl From<RBGPIOeError> for RBInputError {
//     fn from(e: RBGPIOeError) -> Self {
//         RBInputError::GPIOe(e)
//     }
// }

#[derive(Debug)]
pub enum RBModeError {
    /// RB DAC Error
    // Dac(RBDacError),
    /// RB Input Error
    Input(RBInputError),
    // GPIO Expander Error
    // GPIOe(RBGPIOeError),
}

// impl From<RBDacError> for RBModeError {
//     fn from(e: RBDacError) -> Self {
//         RBModeError::Dac(e)
//     }
// }

impl From<RBInputError> for RBModeError {
    fn from(e: RBInputError) -> Self {
        RBModeError::Input(e)
    }
}

// impl From<RBGPIOeError> for RBModeError {
//     fn from(e: RBGPIOeError) -> Self {
//         RBModeError::GPIOe(e)
//     }
// }