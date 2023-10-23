// use systemstat::{System, Platform};

// fn main() {
//     let sys = System::new();

//     match sys.cpu_temp() {
//         Ok(cpu_temp) => println!("CPU Temperature: {}", cpu_temp),
//         Err(e) => eprintln!("{:?}", e),
//     }
// }

use sysinfo::{System, SystemExt, ComponentExt};

fn main() {
    // let mut sys = System::new_all();
    // let components = sys.components();
    // let comp = &components[0].label();
    // println!("{:?}", comp);

    let temps = CPUTemp::new();
    println!("CPU Temp");
    println!("\tCPU0 Temp: {}", temps.cpu0_temp);
    println!("\tCPU1 Temp: {}", temps.cpu1_temp);

}

#[derive(Debug)]
struct CPUTemp {
    cpu0_temp: f32,
    cpu1_temp: f32,
}

impl CPUTemp {
    fn new() -> CPUTemp {
        let mut sys = System::new_all();

        let compontns = sys.components();

        let mut cpu0_temp: f32 = Default::default();
        let mut cpu1_temp: f32 = Default::default();

        for component in compontns {
            let label = component.label();
            match label {
                "coretemp Core 0" => {
                    cpu0_temp = component.temperature();
                }
                "coretemp Core 1" => {
                    cpu1_temp = component.temperature();
                }
                _ => {
                    // println!("Mismatched Lable: {}", label);

                }
            }
        }

        Self {
            cpu0_temp,
            cpu1_temp
        }
    }
}