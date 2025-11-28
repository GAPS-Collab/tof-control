use sysinfo::{DiskExt, System, SystemExt};
use i2cdev::core::*;
use i2cdev::linux::LinuxI2CDevice;

use crate::constant::*;
use crate::memory::read_control_reg;
use crate::helper::rb_type::{RBInfo, RBError};
use crate::rb_control::rb_mode;
use crate::i2c_bus_lock::with_i2c_bus_lock;

impl RBInfo {
    pub fn new() -> Self {
        Self::read_all_info()
    }
    pub fn read_all_info() -> RBInfo {
        let board_id = Self::read_board_id().unwrap_or(u8::MAX);
        let sub_board = Self::read_sub_board().unwrap_or(u8::MAX);
        let lol = Self::read_lol().unwrap_or(u8::MAX);
        let lol_stable = Self::read_lol_stable().unwrap_or(u8::MAX);
        let trig_rate = Self::read_trig_rate().unwrap_or(u16::MAX);
        // Additional Info
        let fw_version = Self::read_fw_version().unwrap_or("0.0.0".to_string());
        let fw_hash = Self::read_fw_hash().unwrap_or("XXXXX".to_string());
        let uptime = Self::read_uptime();
        let sd_usage = Self::read_sd_usage();
        let input_mode = Self::read_input_mode().unwrap_or("Input Mode Error".to_string());
        let rat_num = Self::read_rat_num().unwrap_or(u8::MAX);
        let rat_pos = Self::read_rat_pos().unwrap_or(u8::MAX);
        let rb_pos = Self::read_rb_pos().unwrap_or(u8::MAX);

        RBInfo {
            board_id,
            sub_board,
            lol,
            lol_stable,
            trig_rate,
            fw_version,
            fw_hash,
            uptime,
            sd_usage,
            input_mode,
            rat_num,
            rat_pos,
            rb_pos,
        }
    }
    pub fn read_board_id() -> Result<u8, RBError> {
        let mut board_id = read_control_reg(BOARD_ID)? as u8;
        if board_id >= u8::MAX {
            board_id = u8::MAX;
        }
        
        Ok(board_id)
    }
    pub fn read_sub_board() -> Result<u8, RBError> {
        with_i2c_bus_lock(|| {
            let sub_board: u8;

            let mut ltb_i2c = LinuxI2CDevice::new(&format!("/dev/i2c-{}", I2C_BUS), LTB_TRENZ_ADDRESS)?;
            let mut pb_i2c = LinuxI2CDevice::new(&format!("/dev/i2c-{}", I2C_BUS), PB_PCA9548A_ADDRESS)?;

            let ltb_on = ltb_i2c.smbus_read_byte().is_ok();
            let pb_on = pb_i2c.smbus_read_byte().is_ok();

            if ltb_on && !pb_on {
                sub_board = 1;
            } else if !ltb_on && pb_on {
                sub_board = 2;
            } else if ltb_on && pb_on {
                sub_board = 3;
            } else {
                sub_board = 0;
            }

            Ok(sub_board)
        })
    }
    pub fn read_lol() -> Result<u8, RBError> {
        let mut lol = read_control_reg(LOSS_OF_LOCK)? as u8;
        lol = lol & 0x01;

        Ok(lol)
    }
    pub fn read_lol_stable() -> Result<u8, RBError> {
        let mut lol_stable = read_control_reg(LOSS_OF_LOCK_STABLE)? as u8;
        lol_stable = (lol_stable >> 1) & 0x01;

        Ok(lol_stable)
    }
    pub fn read_trig_rate() -> Result<u16, RBError> {
        let trig_rate = read_control_reg(MT_TRIGGER_RATE)? as u16;

        Ok(trig_rate)
    }
    pub fn read_fw_version() -> Result<String, RBError> {
        let fw_version_raw = read_control_reg(GLOBAL_VER)?;

        let mut fw_version = format!("{:08X}", fw_version_raw);
        let major_ver = i64::from_str_radix(&fw_version[..2], 16)?;
        let minor_ver = i64::from_str_radix(&fw_version[2..4], 16)?;
        let patch = i64::from_str_radix(&fw_version[4..], 16)?;
        fw_version = format!("{}.{}.{}", major_ver, minor_ver, patch);

        Ok(fw_version)
    }
    pub fn read_fw_hash() -> Result<String, RBError> {
        let fw_hash = format!("{:02X}",read_control_reg(GLOBAL_SHA)?);

        Ok(fw_hash)
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
    pub fn read_input_mode() -> Result<String, RBError> {
        let input_mode = rb_mode::read_input_mode()?;

        Ok(input_mode)
    }
    pub fn read_rat_num() -> Result<u8, RBError> {
        let board_id = Self::read_board_id()?;
        let rat_num: u8;
        match board_id {
            15 | 3 => rat_num = 1,
            14 | 32 => rat_num = 2,
            29 | 31 => rat_num = 3,
            13 | 35 => rat_num = 4,
            21 | 23 => rat_num = 5,
            24 | 27 => rat_num = 6,
            19 | 20 => rat_num = 7,
            25 | 16 => rat_num = 8,
            30 | 8 => rat_num = 9,
            11 | 1 => rat_num = 10,
            22 | 26 => rat_num = 11,
            40 | 39 => rat_num = 12,
            18 | 9 => rat_num = 13,
            42 | 41 => rat_num = 14,
            4 | 2 => rat_num = 15,
            44 | 46 => rat_num = 16,
            17 | 7 => rat_num = 17,
            34 | 33 => rat_num = 18,
            6 | 36 => rat_num = 19,
            5 | 28 => rat_num = 20,
            48 | 47 => rat_num = 21,
            49 | 37 => rat_num = 22,
            _ => rat_num = 0,
        }

        Ok(rat_num)
    }
    pub fn read_rat_pos() -> Result<u8, RBError> {
        let rat_num = Self::read_rat_num()?;
        /*
        rat_pos = 0 => Not Flight RAT
        rat_pos = 1 => CBE RAT
        rat_pos = 2 => UMB RAT
        rat_pos = 3 => COR RAT
        rat_pos = 4 => CBE/COR RAT
        */
        let rat_pos: u8;
        match rat_num {
            1 => rat_pos = 2,
            2 => rat_pos = 2,
            3 => rat_pos = 2,
            4 => rat_pos = 2,
            5 => rat_pos = 2,
            6 => rat_pos = 2,
            7 => rat_pos = 4,
            8 => rat_pos = 1,
            9 => rat_pos = 4,
            10 => rat_pos = 4,
            11 => rat_pos = 4,
            12 => rat_pos = 1,
            13 => rat_pos = 4,
            14 => rat_pos = 4,
            15 => rat_pos = 4,
            16 => rat_pos = 1,
            17 => rat_pos = 4,
            18 => rat_pos = 3,
            19 => rat_pos = 1,
            20 => rat_pos = 3,
            _ => rat_pos = 0,
        }

        Ok(rat_pos)
    }
    /*
    rb_pos = 0 => Not in RAT
    rb_pos = 1 => RB1
    rb_pos = 2 => RB2
    */
    pub fn read_rb_pos() -> Result<u8, RBError> {
        let rb_pos = Self::read_sub_board()?;

        Ok(rb_pos)
    }
}