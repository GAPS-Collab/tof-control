use crate::constant::*;

use crate::device::{pca9548a, ad5675};

pub struct RBdac {
    dac0: u16,
}

impl RBdac {
    pub fn new() -> Self {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
        i2c_mux.select(RB_AD5675_CHANNEL).expect("cannot accesss to PCA9548A");

        let ad5675 = ad5675::AD5675::new(I2C_BUS, RB_AD5675_ADDRESS);

        let dac0 = ad5675.read_dac(0).expect("cannot read AD5675");

        i2c_mux.reset().expect("cannot reset PCA9548A");

        Self {
            dac0,
        }
    }
    pub fn print_rb_dac() {
        let rb_dac = RBdac::new();
        println!("DAC0:     {}", rb_dac.dac0);
    }
    pub fn dac_test() {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
        i2c_mux.select(RB_AD5675_CHANNEL).expect("cannot accesss to PCA9548A");

        let ad5675 = ad5675::AD5675::new(I2C_BUS, RB_AD5675_ADDRESS);

        ad5675.power_up_all_dac().expect("cannot write AD5675");
        
    }
}