#[allow(unused)]
use crate::constant::*;
use crate::memory::*;
use crate::device::{pca9548a, tmp112, lis3mdltr, bme280};

use log::error;

pub struct RBtemp {
    pub drs_temp: f32,
    pub clk_temp: f32,
    pub adc_temp: f32,
    pub lis3mdltr_temp: f32,
    pub bme280_temp: f32,
    pub zynq_temp: f32,
}

impl RBtemp {
    pub fn new() -> Self {
      let mut temps = Self {
           drs_temp : f32::MAX,
           clk_temp : f32::MAX,
           adc_temp : f32::MAX,
           lis3mdltr_temp : f32::MAX,
           bme280_temp    : f32::MAX,
           zynq_temp      : f32::MAX,
        };
        let i2c_mux_1 = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_1);
        let i2c_mux_2 = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
        match i2c_mux_1.select(RB_DRS_TMP112_CHANNEL) {
          Err(err) => {
            error!("Can not select TMP112 channel, {err}");
            return temps;
          },
          Ok(_) => (),
        }
        let drs_tmp112 = tmp112::TMP112::new(I2C_BUS, RB_DRS_TMP112_ADDRESS);
        match drs_tmp112.config() {
          Err(err) => {
            error!("Can not configure TMP112, {err}");
            return temps;
          },
          Ok(_) => (),
        }
        match drs_tmp112.read() {
          Err(err) => {
            error!("Can not read DRS4 Temp, {err}")
          },
          Ok(_t) => {
            temps.drs_temp = _t;
          }
        }
        match i2c_mux_2.select(RB_CLK_TMP112_CHANNEL) {
          Err(err) => {
            error!("Can not access to PCA9548A, {err}");
          },
          Ok(_) => ()
        }
        let clk_tmp112 = tmp112::TMP112::new(I2C_BUS, RB_CLK_TMP112_ADDRESS);
        match clk_tmp112.config() {
          Err(err) => {
            error!("cannot configure TMP112, {err}");
          },
          Ok(_) => ()
        }

        match clk_tmp112.read() {
          Err(err) => {
            error!("Can not read tmp112, {err}");
          },
          Ok(_t) => {
            temps.clk_temp = _t;
          }
        }
        match i2c_mux_2.select(RB_ADC_TMP112_CHANNEL) {
          Err(err) => {
            error!("Cannot access to PCA9548A");
          },
          Ok(_) => ()
        }
        let adc_tmp112 = tmp112::TMP112::new(I2C_BUS, RB_ADC_TMP112_ADDRESS);
        match adc_tmp112.config() {
          Err(err) => {
            error!("cannot configure TMP112, {err}");
          },
          Ok(_) => ()
        }
        match adc_tmp112.read() {
          Err(err) => {
            error!("cannot read TMP112, {err}");
          },
          Ok(_t) => {
            temps.adc_temp = _t;
          }
        }
        match i2c_mux_1.select(RB_LIS3MDLTR_CHANNEL) {
          Err(err) => {
            error!("cannot accesss to PCA9548A, {err}");
          },
          Ok(_) => (),
        }
        let lis3mdltr = lis3mdltr::LIS3MDLTR::new(I2C_BUS, RB_LIS3MDLTR_ADDRESS);
        lis3mdltr.configure();
        match lis3mdltr.read_temperature() {
          Err(err) => {
            error!("cannot read LIS3MDLTR, {err}");
          },
          Ok(_t) => {
            temps.lis3mdltr_temp = _t;
          }
        }
        match i2c_mux_1.select(RB_BME280_CHANNEL) {
          Err(err) => {
            error!("cannot accesss to PCA9548A, {err}");
          },
          Ok(_) => (),
        }
        let bme280 = bme280::BME280::new(I2C_BUS, RB_BME280_ADDRESS);
        match bme280.configure() {
          Err(err) => {
            error!("cannot configure BME280, {err}");
          },
          Ok(_) => () 
        }
        match bme280.read_data() { 
          Err(err) => {
            error!("Cannot read bme280, {err}");
          },
          Ok(_t) => {
            temps.bme280_temp = _t.0;
          }
        }
        match i2c_mux_1.reset() {
          Err(err) => {
            error!("cannot reset PCA9548A, {err}");
          },
          Ok(_) => ()
        }
        match i2c_mux_2.reset() { 
          Err(err) => {
            error!("cannot reset PCA9548A, {err}");
          }, 
          Ok(_) => ()
        }
        match read_control_reg(RB_TEMP) {
          Err(err) => {
            error!("cannot read TEMP register, err {err}");
          },
          Ok(_t) => {
            let zynq_temp = (((_t & 4095) as f32 * 503.975) / 4096.0) - 273.15;
            temps.zynq_temp = zynq_temp;
          }
        }

        // let zynq_temp = (((zynq_temp_adc & 4095) as f32 * 503.95) / 4096.0) - 273.15;
        //let zynq_temp = (((zynq_temp_adc & 4095) as f32 * 503.975) / 4096.0) - 273.15;
        return temps
    }
    pub fn print_rb_temp() {
        let rb_temp = RBtemp::new();
        println!("DRS Temperature:          {:.3} [°C]", rb_temp.drs_temp);
        println!("CLK Temperature:          {:.3} [°C]", rb_temp.clk_temp);
        println!("ADC Temperature:          {:.3} [°C]", rb_temp.adc_temp);
        println!("LIS3MDLTR Temperature:    {:.3} [°C]", rb_temp.lis3mdltr_temp);
        println!("BME280 Temperature:       {:.3} [°C]", rb_temp.bme280_temp);
        println!("ZYNQ Temperature:         {:.3} [°C]", rb_temp.zynq_temp);
    }
}
