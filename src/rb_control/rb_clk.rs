#![allow(unused)]
use crate::constant::*;

use crate::device::{pca9548a, si5345b};

pub struct RBclk {
    lock_status: bool,
    mode_of_operation: bool,
}

impl RBclk {
    pub fn new() -> Self {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
        i2c_mux
            .select(RB_SI5345B_CHANNEL)
            .expect("cannot accesss to PCA9548A");

        let si5345b = si5345b::SI5345B::new(I2C_BUS, RB_SI5345B_ADDRESS);

        let lol_status = si5345b
            .read_lol_status()
            .expect("cannot read LOL status from SI5345B");
        let lock_status = if lol_status { false } else { true };

        let holdover_status = si5345b
            .read_holdover_status()
            .expect("cannot read HOLD status from SI5345B");
        let mode_of_operation = if holdover_status { false } else { true };

        i2c_mux.reset().expect("cannot reset PCA9548A");

        Self {
            lock_status,
            mode_of_operation,
        }
    }
    pub fn print_rb_clk() {
        let rb_clk = RBclk::new();
        println!(
            "DSPLL Status:             {}",
            if rb_clk.lock_status {
                String::from("Locked")
            } else {
                String::from("Unlocked")
            }
        );
        println!(
            "Mode of Operation:        {}",
            if rb_clk.mode_of_operation {
                String::from("Locked Mode")
            } else {
                String::from("Holdover Mode (or Freerun Mode)")
            }
        );
    }
    pub fn print_config() {
        let si5345b = si5345b::SI5345B::new(I2C_BUS, RB_SI5345B_ADDRESS);
        si5345b
            .configure_si5345b()
            .expect("cannot configure SI5345B");
    }
}

pub fn configure_clk_synth() {
    let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
    i2c_mux
        .select(RB_SI5345B_CHANNEL)
        .expect("cannot accesss to PCA9548A");

    let si5345b = si5345b::SI5345B::new(I2C_BUS, RB_SI5345B_ADDRESS);

    si5345b
        .configure_si5345b()
        .expect("cannot configure SI5345B");

    i2c_mux.reset().expect("cannot reset PCA9548A");
}

pub fn hard_reset_clk_synth() {
    let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
    i2c_mux
        .select(RB_SI5345B_CHANNEL)
        .expect("cannot accesss to PCA9548A");

    let si5345b = si5345b::SI5345B::new(I2C_BUS, RB_SI5345B_ADDRESS);

    si5345b
        .hard_reset_si5345b()
        .expect("cannot hard reset SI5345B");

    i2c_mux.reset().expect("cannot reset PCA9548A");
}

pub fn soft_reset_clk_synth() {
    let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
    i2c_mux
        .select(RB_SI5345B_CHANNEL)
        .expect("cannot accesss to PCA9548A");

    let si5345b = si5345b::SI5345B::new(I2C_BUS, RB_SI5345B_ADDRESS);

    si5345b
        .soft_reset_si5345b()
        .expect("cannot soft reset SI5345B");

    i2c_mux.reset().expect("cannot reset PCA9548A");
}
