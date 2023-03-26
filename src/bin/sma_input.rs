use std::env;

use tof_control::constant::*;
use tof_control::device::{pca9548a, cy8c9560a};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mode: u8 = args[1].parse().unwrap();
    // println!("{}", mode)
    input_select(mode);
}

fn input_select(mode: u8) {
    let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
    i2c_mux.select(RB_CY8C9560A_CHANNEL).expect("cannot accesss to PCA9548A");

    let cy8c9560a = cy8c9560a::CY8C9560A::new(I2C_BUS, RB_CY8C9560A_ADDRESS);

    let port_status = cy8c9560a.read_port_status(7).expect("cannot read CY8C9560A");

    match mode {
        0 => {
            let value = port_status | 0x30;
            cy8c9560a.set_output_port(7, value).expect("cannot write CY8C9560A");
        },
        1 => {
            let value = port_status & !0x30;
            cy8c9560a.set_output_port(7, value).expect("cannot write CY8C9560A");
        },
        2 => {
            let value = port_status & !0x10;
            cy8c9560a.set_output_port(7, value).expect("cannot write CY8C9560A");
        },
        _ => {
            cy8c9560a.set_output_port(7, port_status).expect("cannot write CY8C9560A");
        }
    }
}