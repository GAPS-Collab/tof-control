use crate::constant::*;
use crate::device::{max7320, pca9548a};

pub fn power_on_ltb() {
    let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, PB_PCA9548A_ADDRESS);

    i2c_mux
        .select(PB_MAX7320_CHANNEL)
        .expect("cannot access to PCA9548A");
    let ltb_pwr = max7320::MAX7320::new(I2C_BUS, PB_MAX7320_ADDRESS);
    ltb_pwr
        .output_on_0_3()
        .expect("cannot power on LTB (MAX7320)")
}
pub fn power_off_ltb() {
    let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, PB_PCA9548A_ADDRESS);

    i2c_mux
        .select(PB_MAX7320_CHANNEL)
        .expect("cannot access to PCA9548A");
    let ltb_pwr = max7320::MAX7320::new(I2C_BUS, PB_MAX7320_ADDRESS);
    ltb_pwr
        .output_off_all()
        .expect("cannot power off LTB (MAX7320)")
}
