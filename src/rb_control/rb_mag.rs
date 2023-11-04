use crate::constant::*;
use crate::device::{pca9548a, lis3mdltr};
use log::error;

pub struct RBmag {
    pub magnetic_x: f32,
    pub magnetic_y: f32,
    pub magnetic_z: f32,
    pub magnetic_t: f32,
}

impl RBmag {
    pub fn new() -> Self {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_1);
        match i2c_mux.select(RB_LIS3MDLTR_CHANNEL) {
          Err(err) => {
            error!("cannot accesss to PCA9548A, {err}");
          },
          Ok(_) => () 
        }

        let lis3mdltr = lis3mdltr::LIS3MDLTR::new(I2C_BUS, RB_LIS3MDLTR_ADDRESS);
        lis3mdltr.configure();
        let mut magnetic_field = [f32::MAX, f32::MAX, f32::MAX, f32::MAX];
        match lis3mdltr.read_magnetic_field() {
          Err(err) => {
            error!("Can not read LIS3MDLTR, {err}");
          },
          Ok(_mag_field) => {
            magnetic_field = _mag_field;
          }
        }

        match i2c_mux.reset() {
          Err(err) => {
            error!("cannot reset PCA9548A");
          },
          Ok(_) => ()
        }
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
