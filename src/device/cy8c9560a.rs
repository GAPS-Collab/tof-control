#![allow(unused)]
use crate::constant::*;

use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

// Register
const INPUT_PORT_0: u16 = 0x00;
const INPUT_PORT_1: u16 = 0x01;
const INPUT_PORT_2: u16 = 0x02;
const INPUT_PORT_3: u16 = 0x03;
const INPUT_PORT_4: u16 = 0x04;
const INPUT_PORT_5: u16 = 0x05;
const INPUT_PORT_6: u16 = 0x06;
const INPUT_PORT_7: u16 = 0x07;
const OUTPUT_PORT_0: u16 = 0x08;
const OUTPUT_PORT_1: u16 = 0x09;
const OUTPUT_PORT_2: u16 = 0x0A;
const OUTPUT_PORT_3: u16 = 0x0B;
const OUTPUT_PORT_4: u16 = 0x0C;
const OUTPUT_PORT_5: u16 = 0x0D;
const OUTPUT_PORT_6: u16 = 0x0E;
const OUTPUT_PORT_7: u16 = 0x0F;
const PORT_SELECT: u16 = 0x18;
const INTERRUPT_MASK: u16 = 0x19;
const PIN_DIRECTION: u16 = 0x1C;
const DEVICE_INFO: u16 = 0x2E;

pub struct CY8C9560A {
    bus: u8,
    address: u16,
}

impl CY8C9560A {
    pub fn new(bus: u8, address: u16) -> Self {
        Self { bus, address }
    }
    pub fn read_device_info(&self) -> Result<(u8, u8), LinuxI2CError> {
        let mut dev = LinuxI2CDevice::new(&format!("/dev/i2c-{}", self.bus), self.address)?;
        let device_info = dev.smbus_read_byte_data(DEVICE_INFO as u8)?;
        let device_family = (device_info & 0xF0) >> 4;
        let device_setting = device_info & 0x01;

        Ok((device_family, device_setting))
    }
    pub fn read_port_status(&self, port: u8) -> Result<u8, LinuxI2CError> {
        let mut dev = LinuxI2CDevice::new(&format!("/dev/i2c-{}", self.bus), self.address)?;
        let port_status = dev.smbus_read_byte_data(
            match port {
                0 => INPUT_PORT_0 as u8,
                1 => INPUT_PORT_1 as u8,
                2 => INPUT_PORT_2 as u8,
                3 => INPUT_PORT_3 as u8,
                4 => INPUT_PORT_4 as u8,
                5 => INPUT_PORT_5 as u8,
                6 => INPUT_PORT_6 as u8,
                7 => INPUT_PORT_7 as u8,
                _ => 0xFF,
            },
        )?;

        Ok(port_status)
    }
    pub fn set_output_port(&self, port: u8, value: u8) -> Result<(), LinuxI2CError> {
        let mut dev = LinuxI2CDevice::new(&format!("/dev/i2c-{}", self.bus), self.address)?;
        dev.smbus_write_byte_data(
            match port {
                0 => OUTPUT_PORT_0 as u8,
                1 => OUTPUT_PORT_1 as u8,
                2 => OUTPUT_PORT_2 as u8,
                3 => OUTPUT_PORT_3 as u8,
                4 => OUTPUT_PORT_4 as u8,
                5 => OUTPUT_PORT_5 as u8,
                6 => OUTPUT_PORT_6 as u8,
                7 => OUTPUT_PORT_7 as u8,
                _ => 0xFF,
            },
            value
        )?;

        Ok(())
    }
}