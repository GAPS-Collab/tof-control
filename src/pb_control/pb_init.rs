use crate::pb_control::*;

pub fn initialize() {
    let mut count = 0;
    while count < 5 {
        pb_temp::PBtemp::new();
        pb_vcp::PBvcp::new();
        count += 1;
    }
}
