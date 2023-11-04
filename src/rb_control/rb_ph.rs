use log::error;
use crate::constant::*;

use crate::device::{pca9548a, bme280};

pub struct RBph {
    pub pressure: f32,
    pub humidity: f32,
}

impl RBph {
    pub fn new() -> Self {
        let mut pressure = f32::MAX;
        let mut humidity = f32::MAX;
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_1);

        match i2c_mux.select(RB_BME280_CHANNEL) {
          Err(err) => {
            error!("cannot accesss to PCA9548A");
          },
          Ok(_) => ()
        }
        let bme280 = bme280::BME280::new(I2C_BUS, RB_BME280_ADDRESS);
        match bme280.configure() {
          Err(err) => {
            error!("cannot configure BME280");
          },
          Ok(_) => ()
        }
        match bme280.read_data() {
          Err(err) => {
            error!("cannot read BME280, {err}");
          },
          Ok(result) => {
            pressure = result.1;
            humidity = result.2;
          }
        }
        match i2c_mux.reset() {
          Err(err) => {
            error!("cannot reset PCA9548A, {err}");
          },
          Ok(_) => () 
        }

        Self {
            pressure,
            humidity,
        }
    }
    pub fn print_rb_ph() {
        let rb_ph = RBph::new();
        println!("DRS Pressure:             {:.3} [hPa]", rb_ph.pressure);
        println!("DRS Humidity:             {:.3} [%]", rb_ph.humidity);
    }
}
