#![allow(unused)]
use crate::constant::*;

use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

const READBACK_ENABLE: u16 = 0x90;

pub struct AD5675 {
    bus: u8,
    address: u16,
}

impl AD5675 {
    pub fn new(bus: u8, address: u16) -> Self {
        Self { bus, address }
    }
    pub fn read_dac(&self, channel: u8) -> Result<u16, LinuxI2CError> {
        let mut dev = LinuxI2CDevice::new(&format!("/dev/i2c-{}", self.bus), self.address)?;
        let dac_buf = dev.smbus_read_i2c_block_data((READBACK_ENABLE as u8) | channel, 2)?;
        let dac = ((dac_buf[0] as u16) << 8) | (dac_buf[1] as u16);

        Ok(dac)
    }
}