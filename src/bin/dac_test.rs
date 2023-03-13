use tof_control::constant::*;
use tof_control::device::{pca9548a, ad5675};
use tof_control::rb_control::*;

fn main() {
    let i2c_mux_2 = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
    i2c_mux_2.select(RB_AD5675_CHANNEL).expect("cannot accesss to PCA9548A");
    // i2c_mux_2.select(RB_CY8C9560A_CHANNEL).expect("cannot accesss to PCA9548A");
    
}