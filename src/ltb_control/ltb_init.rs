use crate::ltb_control::*;

pub fn initialize() {
    let mut count = 0;
    while count < 5 {
        ltb_temp::LTBtemp::new();
        count += 1;
    }
    ltb_dac::LTBdac::set_threshold();
}