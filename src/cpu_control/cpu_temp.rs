use sysinfo::{System, SystemExt, ComponentExt};

use crate::helper::cpu_type::CPUTemp;

impl CPUTemp {
    pub fn new() -> Self {
        let cpu_temp = Self::read_temp();

        cpu_temp
    }
    pub fn read_temp() -> CPUTemp {
        let sys = System::new_all();
        let components = sys.components();

        let mut cpu0_temp: f32 = Default::default();
        let mut cpu1_temp: f32 = Default::default();
        for component in components {
            let label = component.label();
            match label {
                "coretemp Core 0" => {
                    cpu0_temp = component.temperature();
                }
                "coretemp Core 1" => {
                    cpu1_temp = component.temperature();
                }
                _ => {
                    // println!("{}: {}", label, component.temperature())
                }
            }
        }

        CPUTemp {
            cpu0_temp,
            cpu1_temp,
        }
    }
}