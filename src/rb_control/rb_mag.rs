use crate::constant::*;
use crate::device::{pca9548a, lis3mdltr};

pub struct RBmag {
    magnetic_x: f32,
    magnetic_y: f32,
    magnetic_z: f32,
    magnetic_t: f32,
}

impl RBmag {
    pub fn new() -> Self {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_1);
        i2c_mux.select(RB_LIS3MDLTR_CHANNEL).expect("cannot accesss to PCA9548A");

        let lis3mdltr = lis3mdltr::LIS3MDLTR::new(I2C_BUS, RB_LIS3MDLTR_ADDRESS);
        lis3mdltr.configure();
        let magnetic_field = lis3mdltr.read_magnetic_field().expect("cannot read LIS3MDLTR");

        i2c_mux.reset().expect("cannot reset PCA9548A");

        Self {
            magnetic_x: magnetic_field[0],
            magnetic_y: magnetic_field[1],
            magnetic_z: magnetic_field[2],
            magnetic_t: magnetic_field[3],
        }
    }
    pub fn print_rb_magnetic() {
        let rb_magnetic = RBmag::new();
        println!("Magnetic X:              {:.3} [G]", rb_magnetic.magnetic_x);
        println!("Magnetic Y:               {:.3} [G]", rb_magnetic.magnetic_y);
        println!("Magnetic Z:              {:.3} [G]", rb_magnetic.magnetic_z);
        println!("Magnetic Total:           {:.3} [G]", rb_magnetic.magnetic_t);
    }
}