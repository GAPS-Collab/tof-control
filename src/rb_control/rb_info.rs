#![allow(unused)]
use crate::constant::*;
use crate::memory::*;

use crate::helper::rb_type::{RBInfo, RBInfoError};

impl RBInfo {
    pub fn new() -> Self {

        // // write_control_reg(TRIGGER_ENABLE, 1).expect("cannot write TRIGGER_ENABLE register");
        // // let event_counter = read_control_reg(MT_EVENT_CNT).expect("cannot write MT_EVENT_CNT register");
        // // let cnt_lost_event = (read_control_reg(CNT_LOST_EVENT).expect("cannot read CNT_LOST_EVENT register") >> 16) as u16;
        // // let trig_received = read_control_reg(CNT_EVENT).expect("cannot read CNT_EVENT register");
        // // let trigger_rate = read_control_reg(TRIGGER_RATE).expect("cannot read TRIGGER_RATE register");
        // // let lost_trigger_rate = read_control_reg(LOST_TRIGGER_RATE).expect("cannot read LOST_TRIGGER_RATE register");

        match Self::read_all_info() {
            Ok(rb_info) => {
                rb_info
            }
            Err(_) => {
                Self {
                    board_id: u8::MAX,
                    lol: u8::MAX,
                    lol_stable: u8::MAX,
                    trig_rate: u16::MAX,
                    fw_version: "0.0.0".to_string(),
                    readout_mask: u16::MAX,
                }
            }
        }
        
    }
    pub fn read_all_info() -> Result<RBInfo, RBInfoError> {
        let board_id = Self::read_board_id()?;
        let lol = Self::read_lol()?;
        let lol_stable = Self::read_lol_stable()?;
        let trig_rate = Self::read_trig_rate()?;
        // Additional Info
        let fw_version = Self::read_fw_version()?;
        let readout_mask = Self::read_readout_mask()?;

        Ok(
            RBInfo {
                board_id,
                lol,
                lol_stable,
                trig_rate,
                fw_version,
                readout_mask,
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
    pub fn read_lol() -> Result<u8, RBInfoError> {
        let mut lol = read_control_reg(LOSS_OF_LOCK)? as u8;
        lol = lol & 0x01;
        if lol == 0x01 {
            lol = 0x00
        } else {
            lol = 0x01
        }

        Ok(lol)
    }
    pub fn read_lol_stable() -> Result<u8, RBInfoError> {
        let mut lol_stable = read_control_reg(LOSS_OF_LOCK_STABLE)? as u8;
        lol_stable = (lol_stable >> 1) & 0x01;
        if lol_stable == 0x01 {
            lol_stable = 0x00
        } else {
            lol_stable = 0x01
        }

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
    pub fn read_readout_mask() -> Result<u16, RBInfoError> {
        let readout_mask = read_control_reg(READOUT_MASK)? as u16;

        Ok(readout_mask)
    }

    // pub fn print_rb_info() {
    //     // println!("Event Counter from MTB:   {}", rb_info.event_counter);
    //     // println!("Number of Trigger Lost:   {}", rb_info.cnt_lost_event);
    //     // println!("Number of Trig Received:  {}", rb_info.trig_received);
    //     // println!("Trigger Rate:             {} [Hz]", rb_info.trigger_rate);
    //     // println!("Lost Trigger Rate:        {} [Hz]", rb_info.lost_trigger_rate);
    // }
}
