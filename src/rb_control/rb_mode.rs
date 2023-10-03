#![allow(unused)]
use crate::rb_control::{rb_dac, rb_input};

pub fn select_noi_mode() {
    rb_dac::dac_noi_mode();
    rb_input::disable_rf_input();
}

pub fn select_vcal_mode() {
    rb_dac::dac_vcal_mode();
    rb_input::disable_rf_input();
}

pub fn select_tcal_mode() {
    rb_dac::dac_tcal_mode();
    rb_input::enable_tca_input();
}

pub fn select_sma_mode() {
    rb_dac::dac_sma_mode();
    rb_input::enable_sma_input();
}
