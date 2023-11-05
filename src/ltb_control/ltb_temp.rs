use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

use crate::constant::*;
use crate::device::tmp112;

use log::error;

pub struct LTBtemp {
    pub trenz_temp: f32,
    pub ltb_temp: f32,
}

impl LTBtemp {
    pub fn new() -> Self {
        let mut trenz_temp = f32::MAX;
        match Self::trenz_temp() {
          Err(err) => {
            error!("cannot read Trenz board (0x3C), {err}");
          },
          Ok(_trenz_temp) => {
            trenz_temp = _trenz_temp;
          }
        }
        let ltb_temp = Self::ltb_temp();
        Self {
            trenz_temp,
            ltb_temp,
        }
    }
    fn ltb_temp() -> f32 {
        let ltb_tmp112 = tmp112::TMP112::new(I2C_BUS, LTB_TMP112_ADDRESS);
        ltb_tmp112.config().expect("cannot configure TMP112");
        let ltb_temp = ltb_tmp112.read().expect("cannot read TMP112");
        ltb_temp
    }
    fn trenz_temp() -> Result<f32, LinuxI2CError> {
        let mut dev = LinuxI2CDevice::new(&format!("/dev/i2c-{}", I2C_BUS), LTB_TRENZ_ADDRESS)?;
        let trenz_temp_raw = dev.smbus_read_i2c_block_data(LTB_TRENZ_TEMP_OFFSET as u8, 2)?;
        let trenz_temp_adc = (((trenz_temp_raw[0] as u16) << 4) | ((trenz_temp_raw[1] as u16) >> 4)) & 0xFFF;
        let trenz_temp = (((trenz_temp_adc & 4095) as f32 * 503.975) / 4096.0) - 273.15;

        Ok(trenz_temp)
    }
    pub fn print_ltb_temp() {
        let ltb_temp = LTBtemp::new();
        println!("Trenz Temperature:        {:.3} [°C]", ltb_temp.trenz_temp);
        println!("LTB Temperature:          {:.3} [°C]", ltb_temp.ltb_temp);
    }
}
