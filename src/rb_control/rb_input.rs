#![allow(unused)]
use crate::rb_control::rb_gpioe;

pub fn disable_rf_input() {
    rb_gpioe::disable_nb3v9312c();
    rb_gpioe::rf_input_select(0);
}

pub fn enable_sma_input() {
    rb_gpioe::disable_nb3v9312c();
    rb_gpioe::rf_input_select(2);
}

pub fn enable_tca_input() {
    rb_gpioe::enable_nb3v9312c();
    rb_gpioe::rf_input_select(1);
}
