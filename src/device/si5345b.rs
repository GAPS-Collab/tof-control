#![allow(unused)]
use crate::constant::*;

use std::thread;
use std::time::Duration;
use csv;
use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

const SET_PAGE: u16 = 0x01;

const LOL_HOLD_STATUS: u16 = 0x00E;

pub struct SI5345B {
    bus: u8,
    address: u16,
}

impl SI5345B {
    pub fn new(bus: u8, address: u16) -> Self {
        Self { bus, address }
    }
    pub fn read_lol_status(&self) -> Result<(bool), LinuxI2CError> {
        let mut dev = LinuxI2CDevice::new(&format!("/dev/i2c-{}", self.bus), self.address)?;
        dev.smbus_write_byte_data(SET_PAGE as u8, ((LOL_HOLD_STATUS >> 8) as u8));

        let mut lol_status = dev.smbus_read_byte_data(LOL_HOLD_STATUS as u8)?;
        lol_status = lol_status & 0x02;

        if lol_status == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }
    pub fn read_holdover_status(&self) -> Result<(bool), LinuxI2CError> {
        let mut dev = LinuxI2CDevice::new(&format!("/dev/i2c-{}", self.bus), self.address)?;
        dev.smbus_write_byte_data(SET_PAGE as u8, ((LOL_HOLD_STATUS >> 8) as u8));

        let mut dspll_mode = dev.smbus_read_byte_data(LOL_HOLD_STATUS as u8)?;
        dspll_mode = dspll_mode & 0x20;

        if dspll_mode == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }
    pub fn configure_si5345b(&self) -> Result<(), LinuxI2CError> {
        let mut dev = LinuxI2CDevice::new(&format!("/dev/i2c-{}", self.bus), self.address)?;

        let si5345b_csv = include_str!("../config/rb_config/si5345b.csv");
        let mut reader = csv::ReaderBuilder::new()
            .comment(Some(b'#'))
            .escape(Some(b'\\'))
            .flexible(true)
            .from_reader(si5345b_csv.as_bytes());

        for (i, record) in reader.records().enumerate() {
            let record = record.expect("failed to convert record");
            let address = i64::from_str_radix(&record[0].trim_start_matches("0x"), 16).expect("cannot convert register from address");
            let data = i64::from_str_radix(&record[1].trim_start_matches("0x"), 16).expect("cannot convert register from data");
            let page = address >> 8;
            let register = address & 0xFF;
            // println!("{} {:?} {:?}", i, address, data);
            
            dev.smbus_write_byte_data(SET_PAGE as u8, page as u8);
            dev.smbus_write_byte_data(register as u8, data as u8);

            if i == 2 {
                thread::sleep(Duration::from_millis(300));
            }
        }

        Ok(())
    }
}