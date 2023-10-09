use i2cdev::core::*;
use i2cdev::linux::LinuxI2CDevice;

use crate::constant::*;
use crate::helper::ltb_type::{LTBTemp, LTBTempError};
use crate::device::tmp112;

impl LTBTemp {
    pub fn new() -> Self {
        let trenz_temp: f32;
        match Self::trenz_temp() {
            Ok(v) => trenz_temp = v,
            Err(_) => trenz_temp = f32::MAX,
        }

        let ltb_temp: f32;
        match Self::ltb_temp() {
            Ok(v) => ltb_temp = v,
            Err(_) => ltb_temp = f32::MAX,
        }

        Self {
            trenz_temp,
            ltb_temp,
        }
    }
    pub fn ltb_temp() -> Result<f32, LTBTempError> {
        let ltb_tmp112 = tmp112::TMP112::new(I2C_BUS, LTB_TMP112_ADDRESS);
        ltb_tmp112.config()?;
        let ltb_temp = ltb_tmp112.read()?;

        Ok(ltb_temp)
    }
    pub fn trenz_temp() -> Result<f32, LTBTempError> {
        let mut dev = LinuxI2CDevice::new(&format!("/dev/i2c-{}", I2C_BUS), LTB_TRENZ_ADDRESS)?;
        let trenz_temp_raw = dev.smbus_read_i2c_block_data(LTB_TRENZ_TEMP_OFFSET as u8, 2)?;
        let trenz_temp_adc =
            (((trenz_temp_raw[0] as u16) << 4) | ((trenz_temp_raw[1] as u16) >> 4)) & 0xFFF;
        let trenz_temp = (((trenz_temp_adc & 4095) as f32 * 503.975) / 4096.0) - 273.15;

        Ok(trenz_temp)
    }
}
