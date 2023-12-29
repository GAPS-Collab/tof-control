#![allow(unused)]
use sysinfo::{DiskExt, System, SystemExt};
use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

use crate::constant::*;
use crate::memory::*;

use crate::helper::rb_type::{RBInfo, RBInfoDebug, RBInfoError};
use crate::rb_control::rb_mode;

impl RBInfo {
    pub fn new() -> Self {

        match Self::read_all_info() {
            Ok(rb_info) => {
                rb_info
            }
            Err(_) => {
                Self {
                    board_id: u8::MAX,
                    sub_board: u8::MAX,
                    lol: u8::MAX,
                    lol_stable: u8::MAX,
                    trig_rate: u16::MAX,
                }
            }
        }
        
    }
    pub fn read_all_info() -> Result<RBInfo, RBInfoError> {
        let board_id = Self::read_board_id()?;
        let sub_board = Self::read_sub_board()?;
        let lol = Self::read_lol()?;
        let lol_stable = Self::read_lol_stable()?;
        let trig_rate = Self::read_trig_rate()?;

        Ok(
            RBInfo {
                board_id,
                sub_board,
                lol,
                lol_stable,
                trig_rate,
            }
        )
    }
    pub fn read_board_id() -> Result<u8, RBInfoError> {
        let mut board_id = read_control_reg(BOARD_ID)? as u8;
        if board_id > 50 {
            board_id = u8::MAX;
        }
        
        Ok(board_id)
    }
    pub fn read_sub_board() -> Result<u8, RBInfoError> {
        let mut sub_board: u8 = Default::default();

        let mut ltb_i2c = LinuxI2CDevice::new(&format!("/dev/i2c-{}", I2C_BUS), LTB_TRENZ_ADDRESS)?;
        let mut pb_i2c = LinuxI2CDevice::new(&format!("/dev/i2c-{}", I2C_BUS), PB_PCA9548A_ADDRESS)?;

        if ltb_i2c.smbus_read_byte().is_ok() {
            sub_board = 1;
        } else if pb_i2c.smbus_read_byte().is_ok() {
            sub_board = 2;
        } else {
            sub_board = 0;
        }

        Ok(sub_board)
    }
    pub fn read_lol() -> Result<u8, RBInfoError> {
        let mut lol = read_control_reg(LOSS_OF_LOCK)? as u8;
        lol = lol & 0x01;

        Ok(lol)
    }
    pub fn read_lol_stable() -> Result<u8, RBInfoError> {
        let mut lol_stable = read_control_reg(LOSS_OF_LOCK_STABLE)? as u8;
        lol_stable = (lol_stable >> 1) & 0x01;

        Ok(lol_stable)
    }
    pub fn read_trig_rate() -> Result<u16, RBInfoError> {
        let trig_rate = read_control_reg(MT_TRIGGER_RATE)? as u16;

        Ok(trig_rate)
    }
}

impl RBInfoDebug {
    pub fn new() -> Self {

        match Self::read_all_info() {
            Ok(rb_info) => {
                rb_info
            }
            Err(_) => {
                Self {
                    board_id: u8::MAX,
                    sub_board: u8::MAX,
                    lol: u8::MAX,
                    lol_stable: u8::MAX,
                    trig_rate: u16::MAX,
                    fw_version: "0.0.0".to_string(),
                    uptime: u32::MAX,
                    sd_usage: u8::MAX,
                    input_mode: "Input Mode Error".to_string(),
                }
            }
        }
        
    }
    pub fn read_all_info() -> Result<RBInfoDebug, RBInfoError> {
        let board_id = Self::read_board_id()?;
        let sub_board = Self::read_sub_board()?;
        let lol = Self::read_lol()?;
        let lol_stable = Self::read_lol_stable()?;
        let trig_rate = Self::read_trig_rate()?;
        // Additional Info
        let fw_version = Self::read_fw_version()?;
        let uptime = Self::read_uptime();
        let sd_usage = Self::read_sd_usage();
        let input_mode = Self::read_input_mode()?;

        Ok(
            RBInfoDebug {
                board_id,
                sub_board,
                lol,
                lol_stable,
                trig_rate,
                fw_version,
                uptime,
                sd_usage,
                input_mode,
            }
        )
    }
    pub fn read_board_id() -> Result<u8, RBInfoError> {
        let mut board_id = read_control_reg(BOARD_ID)? as u8;
        if board_id > 50 {
            board_id = u8::MAX;
        }
        
        Ok(board_id)
    }
    pub fn read_sub_board() -> Result<u8, RBInfoError> {
        let mut sub_board: u8 = Default::default();

        let mut ltb_i2c = LinuxI2CDevice::new(&format!("/dev/i2c-{}", I2C_BUS), LTB_TRENZ_ADDRESS)?;
        let mut pb_i2c = LinuxI2CDevice::new(&format!("/dev/i2c-{}", I2C_BUS), PB_PCA9548A_ADDRESS)?;

        if ltb_i2c.smbus_read_byte().is_ok() {
            sub_board = 1;
        } else if pb_i2c.smbus_read_byte().is_ok() {
            sub_board = 2;
        } else {
            sub_board = 0;
        }

        Ok(sub_board)
    }
    pub fn read_lol() -> Result<u8, RBInfoError> {
        let mut lol = read_control_reg(LOSS_OF_LOCK)? as u8;
        lol = lol & 0x01;

        Ok(lol)
    }
    pub fn read_lol_stable() -> Result<u8, RBInfoError> {
        let mut lol_stable = read_control_reg(LOSS_OF_LOCK_STABLE)? as u8;
        lol_stable = (lol_stable >> 1) & 0x01;

        Ok(lol_stable)
    }
    pub fn read_trig_rate() -> Result<u16, RBInfoError> {
        let trig_rate = read_control_reg(MT_TRIGGER_RATE)? as u16;

        Ok(trig_rate)
    }
    pub fn read_fw_version() -> Result<String, RBInfoError> {
        let mut fw_version_raw = read_control_reg(GLOBAL_VER)?;

        let mut fw_version = format!("{:08X}", fw_version_raw);
        let major_ver = i64::from_str_radix(&fw_version[..2], 16)?;
        let minor_ver = i64::from_str_radix(&fw_version[2..4], 16)?;
        let patch = i64::from_str_radix(&fw_version[4..], 16)?;
        fw_version = format!("{}.{}.{}", major_ver, minor_ver, patch);

        Ok(fw_version)
    }
    pub fn read_uptime() -> u32 {
        let sys = System::new_all();
        let uptime = sys.uptime();

        return uptime as u32
    }
    pub fn read_sd_usage() -> u8 {
        let sys = System::new_all();

        let mut available_space = Default::default();
        let mut total_space = Default::default();
        for disk in sys.disks() {
            available_space = disk.available_space();
            total_space = disk.total_space();
        }

        let sd_usage: f32 = (1.0 - (available_space as f32 / total_space as f32)) * 100.0;

        return sd_usage as u8
    }
    pub fn read_input_mode() -> Result<String, RBInfoError> {
        let input_mode = rb_mode::read_input_mode()?;

        Ok((input_mode))
    }
}