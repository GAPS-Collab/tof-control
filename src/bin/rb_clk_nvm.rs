use tof_control::constant::*;
use tof_control::device::{pca9548a, si5345b};

fn main() {
    let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
    if let Ok(_) = i2c_mux.select(RB_SI5345B_CHANNEL) {
        let si5345b = si5345b::SI5345B::new(I2C_BUS, RB_SI5345B_ADDRESS);
        if let Ok(_) = si5345b.configure_nvm_si5345b() {
            println!("Successfully written clock configuration to NVM!");
        }
    }
}