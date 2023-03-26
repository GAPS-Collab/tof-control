use std::env;

use tof_control::constant::*;
use tof_control::device::{pca9548a, ad5675};
use tof_control::rb_control::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("argument error");
        std::process::exit(1);
    }

    rb_gpioe::enable_ad5675();

    let i2c_mux_2 = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
    i2c_mux_2.select(RB_AD5675_CHANNEL).expect("cannot accesss to PCA9548A");
    let ad5675 = ad5675::AD5675::new(RB_AD5675_ADDRESS);
    // ad5675.write_to_update(0, 20000);
    let channel: u8 = args [1].parse().unwrap();
    let value: u16 = args[2].parse().unwrap();
    ad5675.write_dac(channel, value);
    println!("Channel {}: {}", channel, value);
    // ad5675.read_dac(channel);
    // i2c_mux_2.select(RB_CY8C9560A_CHANNEL).expect("cannot accesss to PCA9548A");
    
}