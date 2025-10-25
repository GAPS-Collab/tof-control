#![allow(unused)]
use libc::{ioctl, O_RDWR, STDOUT_FILENO};
use ratatui::buffer;
use std::fs::File;
use std::os::fd::{AsRawFd, IntoRawFd, RawFd};
use std::os::raw::{c_uint, c_ulong};

use crate::constant::*;

use i2c_linux_sys::*;

use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CMessage, LinuxI2CError};

const READBACK_ENABLE: u16 = 0x90;
const POWER_DOWN_UP: u16 = 0x40;

pub struct AD5675 {
    fd: RawFd,
    address: u16,
}
impl AD5675 {
    pub fn new(address: u16) -> Self {
        let fd = if let Ok(file) = File::open("/dev/i2c-0") {
            file.into_raw_fd()
        } else {
            STDOUT_FILENO
        };
        Self { fd, address }
    }
    pub fn write_dac(&self, channel: u8, value: u16) {
        unsafe { ioctl(self.fd, (I2C_SLAVE as c_ulong).try_into().unwrap(), 0x77) };
        i2c_linux_sys::i2c_smbus_write_byte(self.fd, 0x04);
        unsafe {
            ioctl(
                self.fd,
                (I2C_SLAVE as c_ulong).try_into().unwrap(),
                self.address as c_uint,
            )
        };
        let mut buffer = (value & 0xFF00) >> 8;
        buffer = buffer | (value & 0x00FF) << 8;
        i2c_linux_sys::i2c_smbus_write_word_data(self.fd, 0x30 + channel, buffer);
    }
}
