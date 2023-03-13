use std::thread;
use std::time::Duration;

use crate::rb_control::*;
use crate::constant::*;
use crate::memory::*;

pub fn initialize() {
    let mut count = 0;
    while count < 5 {
        rb_temp::RBtemp::new();
        rb_vcp::RBvcp::new();
        rb_ph::RBph::new();
        rb_mag::RBmag::new();
        count += 1;
    }
    disable_daq_fragment();
    // 1 milisec sleep
    thread::sleep(Duration::from_millis(1));
    start_drs();
}

fn start_drs() {
    write_control_reg(START, 0x01).expect("cannot write START register");
}

fn disable_daq_fragment() {
    write_control_reg(DAQ_FRAGMENT_EN, 0x00).expect("cannot write DAQ_FRAGMENT_EN register");
}