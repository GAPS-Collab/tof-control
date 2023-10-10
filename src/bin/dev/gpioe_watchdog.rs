use std::thread;
use std::time::Duration;

use tof_control::constant::*;
use tof_control::device::{cy8c9560a, pca9548a};

fn main() {
    loop {
        // println!("on");
        switch_gpioe_watchdog(true);
        thread::sleep(Duration::from_secs(20));
        // println!("off");
        switch_gpioe_watchdog(false);
        thread::sleep(Duration::from_secs(20));
    }
}

fn switch_gpioe_watchdog(operation: bool) {
    let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
    i2c_mux
        .select(RB_CY8C9560A_CHANNEL)
        .expect("cannot accesss to PCA9548A");

    let cy8c9560a = cy8c9560a::CY8C9560A::new(I2C_BUS, RB_CY8C9560A_ADDRESS);

    let port_status = cy8c9560a
        .read_port_status(3)
        .expect("cannot read CY8C9560A");

    match operation {
        true => {
            let value = port_status | 0x20;
            cy8c9560a
                .set_output_port(3, value)
                .expect("cannot write CY8C9560A");
        }
        false => {
            let value = port_status & 0xDF;
            cy8c9560a
                .set_output_port(3, value)
                .expect("cannot write CY8C9560A");
        }
    };

    i2c_mux.reset().expect("cannot reset PCA9548A");
}
