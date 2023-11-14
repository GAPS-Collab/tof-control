use sysinfo::{System, SystemExt};

use crate::helper::cpu_type::CPUInfo;

impl CPUInfo {
    pub fn new() -> Self {
        let cpu_info = Self::read_info();

        cpu_info
    }
    pub fn read_info() -> CPUInfo {
        let sys = System::new_all();
        let uptime = sys.uptime() as u32;

        CPUInfo {
            uptime,
        }
    }
}