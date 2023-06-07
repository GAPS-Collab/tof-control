use crate::ltb_control::*;

pub fn ucla_mode() {
    ltb_dac::LTBdac::set_threshold_ucla();
}