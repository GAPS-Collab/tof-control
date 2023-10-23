use crate::constant::*;

use crate::helper::rb_type::RBPhError;
use crate::device::{bme280, pca9548a};

pub struct RBph {
    pub pressure: f32,
    pub humidity: f32,
}

impl RBph {
    pub fn new() -> Self {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_1);

        i2c_mux
            .select(RB_BME280_CHANNEL)
            .expect("cannot accesss to PCA9548A");
        let bme280 = bme280::BME280::new(I2C_BUS, RB_BME280_ADDRESS);
        bme280.configure().expect("cannot configure BME280");
        let (_, pressure, humidity) = bme280.read_data().expect("cannot read BME280");

        i2c_mux.reset().expect("cannot reset PCA9548A");

        Self { pressure, humidity }
    }
    pub fn print_rb_ph() {
        let rb_ph = RBph::new();
        println!("DRS Pressure:             {:.3} [hPa]", rb_ph.pressure);
        println!("DRS Humidity:             {:.3} [%]", rb_ph.humidity);
    }
}

pub fn config_ph() -> Result<(), RBPhError> {
    let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_1);
    i2c_mux.select(RB_BME280_CHANNEL)?;
    let bme280 = bme280::BME280::new(I2C_BUS, RB_BME280_ADDRESS);
    bme280.configure()?;

    i2c_mux.reset()?;

    Ok(())
}