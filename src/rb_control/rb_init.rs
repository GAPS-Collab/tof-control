#![allow(unused)]
use gethostname::gethostname;
use std::thread;
use std::time::Duration;

use crate::constant::*;
use crate::memory::*;
use crate::rb_control::*;

pub fn initialize() {
    initialize_gpioe();
    initialize_clk_synth();
    initialize_rf_input();
    initialize_dac();

    initialize_env_ics();

    set_board_id();
    disable_daq_fragment();
    enable_spike_clean();
    enable_all_channels();
    start_drs();
}

fn initialize_gpioe() {
    rb_gpioe::initialize();
}

fn initialize_clk_synth() {
    rb_gpioe::enable_si5345b();
    rb_clk::configure_clk_synth();
}

fn initialize_rf_input() {
    // SMA Input
    rb_gpioe::rf_input_select(2);
}

fn initialize_dac() {
    rb_dac::set_dac();
}

fn initialize_env_ics() {
    let mut count = 0;
    while count < 5 {
        rb_temp::RBtemp::new();
        rb_vcp::RBvcp::new();
        rb_ph::RBph::new();
        rb_mag::RBmag::new();
        count += 1;
    }
}

fn set_board_id() {
    let hostname = gethostname()
        .into_string()
        .expect("cannot convert hostname");
    let board_id: u32 = hostname.replace("tof-rb", "").parse().unwrap();
    write_control_reg(BOARD_ID, board_id).expect("cannot write BOARD_ID register");
}

fn start_drs() {
    write_control_reg(START, 0x01).expect("cannot write START register");
}

fn disable_daq_fragment() {
    let mut value =
        read_control_reg(DAQ_FRAGMENT_EN).expect("cannot read DAQ_FRAGMENT_EN register");
    value = value | 0x00;
    write_control_reg(DAQ_FRAGMENT_EN, value).expect("cannot write DAQ_FRAGMENT_EN register");
}

fn enable_spike_clean() {
    let mut value =
        read_control_reg(EN_SPIKE_REMOVAL).expect("cannot read EN_SPIKE_REMOVAL register");
    value = value | 0x400000;
    write_control_reg(EN_SPIKE_REMOVAL, value).expect("cannot write EN_SPIKE_REMOVAL register");
}

fn enable_all_channels() {
    let mut value = read_control_reg(READOUT_MASK).expect("cannot read READOUT_MASK register");
    value = value | 0x1FF;
    write_control_reg(READOUT_MASK, value).expect("cannot write READOUT_MASK register");
}
