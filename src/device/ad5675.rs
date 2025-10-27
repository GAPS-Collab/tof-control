#![allow(unused)]
use crate::constant::*;

use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CMessage, LinuxI2CError};

const WRITE_AND_UPDATE_DAC: u8 = 0x30;
const READBACK_ENABLE: u8 = 0x90;
const POWER_DOWN_UP: u8 = 0x40;

pub struct AD5675 {
    bus: u8,
    address: u16,
}
impl AD5675 {
    pub fn new(bus: u8, address: u16) -> Self {
        Self { bus, address }
    }
    
    pub fn write_dac(&self, channel:u8, value: u16) -> Result<(), LinuxI2CError> {
        let mut dev = LinuxI2CDevice::new(&format!("/dev/i2c-{}", self.bus), self.address)?;
        
        let dac_value_msb = (value & 0xFF00) >> 8;
        let dac_value_lsb = (value & 0x00FF);
        let dac_value = [dac_value_msb as u8, dac_value_lsb as u8];

        dev.smbus_write_i2c_block_data(WRITE_AND_UPDATE_DAC+channel, &dac_value);
        
        Ok(())
    }

    pub fn read_dac(&self, channel: u8) -> Result<u16, LinuxI2CError>{
        let mut dev = LinuxI2CDevice::new(&format!("/dev/i2c-{}", self.bus), self.address)?;
        let mut read_data = [0; 2];
        let mut msgs = [
            LinuxI2CMessage::write(&[READBACK_ENABLE+channel, 0x00, 0x00]),
            LinuxI2CMessage::read(&mut read_data)
        ];
        
        dev.transfer(&mut msgs)?;

        let dac_value = ((read_data[0] as u16 & 0xFF) << 8) | (read_data[1] as u16 & 0xFF);

        Ok((dac_value))
    }
}
