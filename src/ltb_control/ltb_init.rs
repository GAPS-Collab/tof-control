use crate::ltb_control::*;
use crate::helper::ltb_type::LTBTemp;

pub fn initialize() {
    let mut count = 0;
    while count < 5 {
        LTBTemp::new();
        count += 1;
    }
    // ltb_threshold::LTBdac::set_threshold();
}
