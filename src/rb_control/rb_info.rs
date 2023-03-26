#![allow(unused)]
use crate::constant::*;
use crate::memory::*;

pub struct RBinfo {
    pub board_id: u8,
    pub global_ver: String,
    pub global_sha: u32,
    pub loss_of_lock: String,
    pub loss_of_lock_stable: String,
    // cnt_lost_event: u16,
    // event_counter: u32,
    // trig_received: u32,
    // trigger_rate: u32,
    // lost_trigger_rate: u32,
}

impl RBinfo {
    pub fn new() -> Self {
        let board_id = read_control_reg(BOARD_ID).expect("cannot read BOARD_ID register") as u8;
        let global_ver = Self::decode_fw_version(read_control_reg(GLOBAL_VER).expect("cannot read GLOBAL_VER register"));
        let global_sha = read_control_reg(GLOBAL_SHA).expect("cannot read GLOBAL_SHA register");

        let loss_of_lock = Self::decode_loss_of_lock((read_control_reg(LOSS_OF_LOCK).expect("cannot read LOSS_OF_LOCK register") as u8));
        let loss_of_lock_stable = Self::decode_loss_of_lock_stable(((read_control_reg(LOSS_OF_LOCK_STABLE).expect("cannot read LOSS_OF_LOCK_STABLE register") as u8) >> 1) & 0x01);

        // write_control_reg(TRIGGER_ENABLE, 1).expect("cannot write TRIGGER_ENABLE register");
        // let event_counter = read_control_reg(MT_EVENT_CNT).expect("cannot write MT_EVENT_CNT register");
        // let cnt_lost_event = (read_control_reg(CNT_LOST_EVENT).expect("cannot read CNT_LOST_EVENT register") >> 16) as u16;
        // let trig_received = read_control_reg(CNT_EVENT).expect("cannot read CNT_EVENT register");
        // let trigger_rate = read_control_reg(TRIGGER_RATE).expect("cannot read TRIGGER_RATE register");
        // let lost_trigger_rate = read_control_reg(LOST_TRIGGER_RATE).expect("cannot read LOST_TRIGGER_RATE register");

        Self {
            board_id,
            global_ver,
            global_sha,
            loss_of_lock,
            loss_of_lock_stable,
            // event_counter,
            // cnt_lost_event,
            // trig_received,
            // trigger_rate,
            // lost_trigger_rate,
        }
    }
    pub fn print_rb_info() {
        let rb_info = RBinfo::new();
        println!("Board ID:                 {}", rb_info.board_id);
        println!("FPGA Firmware Version:    {}", rb_info.global_ver);
        println!("FPGA Firmware Hash:       {:02X}", rb_info.global_sha);
        println!("Loss of Lock Status:      {}", rb_info.loss_of_lock);
        // println!("Loss of Lock Status:      {}", rb_info.loss_of_lock);
        println!("Loss of Lock Stable:      {}", rb_info.loss_of_lock_stable);
        // println!("Event Counter from MTB:   {}", rb_info.event_counter);
        // println!("Number of Trigger Lost:   {}", rb_info.cnt_lost_event);
        // println!("Number of Trig Received:  {}", rb_info.trig_received);
        // println!("Trigger Rate:             {} [Hz]", rb_info.trigger_rate);
        // println!("Lost Trigger Rate:        {} [Hz]", rb_info.lost_trigger_rate);
    }
    fn decode_fw_version(global_ver: u32) -> String {
        let global_ver_str = format!("{:08X}", global_ver);

        let major_ver = &global_ver_str[..2].parse::<u8>().unwrap();
        let minor_ver = &global_ver_str[2..4].parse::<u8>().unwrap();
        let patch = &global_ver_str[4..].parse::<u8>().unwrap();
        let version = format!("{}.{}.{}", major_ver, minor_ver, patch);

        version
    }
    fn decode_loss_of_lock(loss_of_lock: u8) -> String {
        let mut loss_of_lock_status = String::new();
        if loss_of_lock == 0 {
            loss_of_lock_status = "LOCKED".to_string()
        } else {
            loss_of_lock_status = "UNLOCKED".to_string()
        }

        loss_of_lock_status
    }
    fn decode_loss_of_lock_stable(loss_of_lock_stable: u8) -> String {
        let mut loss_of_lock_stable_status = String::new();
        if loss_of_lock_stable == 0 {
            loss_of_lock_stable_status = "LOCKED PAST SECOND".to_string()
        } else {
            loss_of_lock_stable_status = "UNLOCKED PAST SECOND".to_string()
        }

        loss_of_lock_stable_status
    }
}