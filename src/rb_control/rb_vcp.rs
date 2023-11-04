 use log::error;

use crate::constant::*;

use crate::device::{pca9548a, ina226, max11645};

pub struct RBvcp {
    pub drs_dvdd_voltage: f32,
    pub drs_dvdd_current: f32,
    pub drs_dvdd_power: f32,
    pub p3v3_voltage: f32,
    pub p3v3_current: f32,
    pub p3v3_power: f32,
    pub zynq_voltage: f32,
    pub zynq_current: f32,
    pub zynq_power: f32,
    pub p3v5_voltage: f32,
    pub p3v5_current: f32,
    pub p3v5_power: f32,
    pub adc_dvdd_voltage: f32,
    pub adc_dvdd_current: f32,
    pub adc_dvdd_power: f32,
    pub adc_avdd_voltage: f32,
    pub adc_avdd_current: f32,
    pub adc_avdd_power: f32,
    pub drs_avdd_voltage: f32,
    pub drs_avdd_current: f32,
    pub drs_avdd_power: f32,
    pub n1v5_voltage: f32,
    pub n1v5_current: f32,
    pub n1v5_power: f32,
}

impl RBvcp {
    pub fn new() -> Self {
        let i2c_mux_1 = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_1);
        let i2c_mux_2 = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);

        match i2c_mux_1.select(RB_DRS_DVDD_INA226_CHANNEL) {
          Err(err) => {
            error!("cannot accesss to PCA9548A, {err}");
          },
          Ok(_) => ()
        }

        let drs_dvdd_ina226 = ina226::INA226::new(I2C_BUS, RB_DRS_DVDD_INA226_ADDRESS, RB_DRS_DVDD_INA226_RSHUNT, RB_DRS_DVDD_INA226_MEC);
        match drs_dvdd_ina226.configure() {
          Err(err) => {
            error!("cannot configure INA226 (DRS DVDD)");
          }, 
          Ok(_) => ()
        }
        let mut drs_dvdd_voltage = f32::MAX;
        let mut drs_dvdd_current = f32::MAX;
        let mut drs_dvdd_power   = f32::MAX;
        match drs_dvdd_ina226.read_data() {
          Err(err) => {
            error!("cannot read INA226 (DRS DVDD), {err}");
          },
          Ok(_result) => {
            drs_dvdd_voltage = _result.0;
            drs_dvdd_current = _result.1;
            drs_dvdd_power   = _result.2;
          }
        }
        match i2c_mux_1.select(RB_P3V3_INA226_CHANNEL) {
          Err(err) => {
            error!("cannot accesss to PCA9548A");
          },
          Ok(_) => ()
        }
        let p3v3_ina226 = ina226::INA226::new(I2C_BUS, RB_P3V3_INA226_ADDRESS, RB_P3V3_INA226_RSHUNT, RB_P3V3_INA226_MEC);
        match p3v3_ina226.configure() {
          Err(err) => {
            error!("cannot configure INA226 (P3V3)");
          },
          Ok(_) => () 
        }
        let mut p3v3_voltage = f32::MAX;
        let mut p3v3_current = f32::MAX;
        let mut p3v3_power   = f32::MAX;
        match p3v3_ina226.read_data() {
          Err(err) => {
            error!("cannot read INA226 (P3V3), {err}");
          },
          Ok(_result) => {
            p3v3_voltage = _result.0;
            p3v3_current = _result.1;
            p3v3_power   = _result.2;
          }
        }
        match i2c_mux_1.select(RB_ZYNQ_INA226_CHANNEL) {
          Err(err) => {
            error!("cannot accesss to PCA9548A, {err}");
          },
          Ok(_) => ()
        }
        let zynq_ina226 = ina226::INA226::new(I2C_BUS, RB_ZYNQ_INA226_ADDRESS, RB_ZYNQ_INA226_RSHUNT, RB_ZYNQ_INA226_MEC);
        match zynq_ina226.configure() { 
          Err(err) => {
            error!("cannot configure INA226 (ZYNQ)");
          }
          Ok(_) => ()
        }
        let mut zynq_voltage = f32::MAX;
        let mut zynq_current = f32::MAX;
        let mut zynq_power   = f32::MAX;
        match zynq_ina226.read_data() {
          Err(err) => {
            error!("cannot read INA226 (ZYNQ), {err}");
          },
          Ok(_result) => {
            zynq_voltage = _result.0;
            zynq_current = _result.1;
            zynq_power   = _result.2;
          }
        }
        match i2c_mux_1.select(RB_P3V5_INA226_CHANNEL) {
          Err(err) => {
            error!("cannot accesss to PCA9548A, {err}");
          },
          Ok(_) => () 
        }
        let p3v5_ina226 = ina226::INA226::new(I2C_BUS, RB_P3V5_INA226_ADDRESS, RB_P3V5_INA226_RSHUNT, RB_P3V5_INA226_MEC);
        match p3v5_ina226.configure() {
          Err(err) => {
            error!("cannot configure INA226 (P3V5), {err}");
          },
          Ok(_) => ()
        }
        let mut p3v5_voltage = f32::MAX;
        let mut p3v5_current = f32::MAX;
        let mut p3v5_power   = f32::MAX;
        match p3v5_ina226.read_data() {
          Err(err) => {
            error!("cannot read INA226 (P3V5), {err}");
          },
          Ok(_result) => {
            p3v5_voltage = _result.0;
            p3v5_current = _result.1;
            p3v5_power   = _result.2;
          }
        }
        match i2c_mux_2.select(RB_ADC_DVDD_INA226_CHANNEL) {
          Err(err) => {
            error!("cannot accesss to PCA9548A");
          },
          Ok(_) => (),
        }
        let adc_dvdd_ina226 = ina226::INA226::new(I2C_BUS, RB_ADC_DVDD_INA226_ADDRESS, RB_ADC_DVDD_INA226_RSHUNT, RB_ADC_DVDD_INA226_MEC);
        match adc_dvdd_ina226.configure() {
          Err(err) => {
            error!("cannot configure INA226 (ADC DVDD)");
          },
          Ok(_) => () 
        }
        let mut adc_dvdd_voltage = f32::MAX;
        let mut adc_dvdd_current = f32::MAX;
        let mut adc_dvdd_power   = f32::MAX;
        match adc_dvdd_ina226.read_data() {
          Err(err) => {
            error!("cannot read INA226 (ADC DVDD)");
          },
          Ok(_result) => {
            adc_dvdd_voltage = _result.0;
            adc_dvdd_current = _result.1;
            adc_dvdd_power   = _result.2;
          }
        }

        match i2c_mux_2.select(RB_ADC_AVDD_INA226_CHANNEL) {
          Err(err) => {
            error!("cannot accesss to PCA9548A");
          },
          Ok(_) => ()
        }
        let adc_avdd_ina226 = ina226::INA226::new(I2C_BUS, RB_ADC_AVDD_INA226_ADDRESS, RB_ADC_AVDD_INA226_RSHUNT, RB_ADC_AVDD_INA226_MEC);
        match adc_avdd_ina226.configure() {
          Err(err) => {
            error!("cannot configure INA226 (ADC AVDD)");
          },
          Ok(_) => ()
        }
        let mut adc_avdd_voltage = f32::MAX;
        let mut adc_avdd_current = f32::MAX;
        let mut adc_avdd_power   = f32::MAX;
        match adc_avdd_ina226.read_data() {
          Err(err) => {
            error!("cannot read INA226 (ADC AVDD), {err}");
          },
          Ok(_result) => {
            adc_avdd_voltage = _result.0;
            adc_avdd_current = _result.1;
            adc_avdd_power   = _result.2;
          }
        }
        match i2c_mux_2.select(RB_DRS_AVDD_INA226_CHANNEL) {
          Err(err) => {
            error!("cannot accesss to PCA9548A, {err}");
          },
          Ok(_) => ()
        }
        let drs_avdd_ina226 = ina226::INA226::new(I2C_BUS, RB_DRS_AVDD_INA226_ADDRESS, RB_DRS_AVDD_INA226_RSHUNT, RB_DRS_AVDD_INA226_MEC);
        match drs_avdd_ina226.configure() {
          Err(err) => {
            error!("cannot configure INA226 (DRS AVDD), {err}");
          },
          Ok(_) => ()
        }
        let mut drs_avdd_voltage = f32::MAX;
        let mut drs_avdd_current = f32::MAX;
        let mut drs_avdd_power   = f32::MAX;
        match drs_avdd_ina226.read_data() {
          Err(err) => {
            error!("cannot read INA226 (DRS AVDD), {err}");
          },
          Ok(_result) => {
            drs_avdd_voltage = _result.0;
            drs_avdd_current = _result.1;
            drs_avdd_power   = _result.2;
          }
        }
        match i2c_mux_1.select(RB_MAX11645_CHANNEL) {
          Err(err) => {
            error!("cannot accesss to PCA9548A, {err}");
          },
          Ok(_) => ()
        }
        let max11645 = max11645::MAX11645::new(I2C_BUS, RB_MAX11645_ADDRESS);
        match max11645.setup() {
          Err(err) => {
            error!("cannot configure MAX11645, {err}");
          },
          Ok(_) => () 
        }
        let mut n1v5_voltage = f32::MAX;
        let mut n1v5_current = f32::MAX;
        let mut n1v5_power   = f32::MAX;

        match max11645.read(RB_N1V5_VOLTAGE_INA200_CHANNEL) {
          Err(err) => {
            error!("cannot read INA200 (N1V5 VOLTAGE), {err}");
          },
          Ok(_result) => {
            n1v5_voltage = _result* -1.0;
          }
        }
        match max11645.read(RB_N1V5_CURRENT_INA200_CHANNEL) {
          Err(err) => {
            error!("cannot read INA200 (N1V5 CURRENT), {err}");
          },
          Ok(_result) => {
            n1v5_current = _result/ 20.0 / 0.039;
          }
        }
        let n1v5_power = n1v5_voltage.abs() * n1v5_current;

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

        Self {
            drs_dvdd_voltage,
            drs_dvdd_current,
            drs_dvdd_power,
            p3v3_voltage,
            p3v3_current,
            p3v3_power,
            zynq_voltage,
            zynq_current,
            zynq_power,
            p3v5_voltage,
            p3v5_current,
            p3v5_power,
            adc_dvdd_voltage,
            adc_dvdd_current,
            adc_dvdd_power,
            adc_avdd_voltage,
            adc_avdd_current,
            adc_avdd_power,
            drs_avdd_voltage,
            drs_avdd_current,
            drs_avdd_power,
            n1v5_voltage,
            n1v5_current,
            n1v5_power,
        }
    }
    pub fn print_rb_vcp() {
        let rb_vcp = RBvcp::new();
        println!("ZYNQ 3.3V Power:          {:.3} [V] | {:.3} [A] | {:.3} [W]", rb_vcp.zynq_voltage, rb_vcp.zynq_current, rb_vcp.zynq_power);
        println!("3.3V Power:               {:.3} [V] | {:.3} [A] | {:.3} [W]", rb_vcp.p3v3_voltage, rb_vcp.p3v3_current, rb_vcp.p3v3_power);
        println!("3.5V Power:               {:.3} [V] | {:.3} [A] | {:.3} [W]", rb_vcp.p3v5_voltage, rb_vcp.p3v5_current, rb_vcp.p3v5_power);
        println!("-1.5V Power:             {:.3} [V] | {:.3} [A] | {:.3} [W]", rb_vcp.n1v5_voltage, rb_vcp.n1v5_current, rb_vcp.n1v5_power);
        println!("DRS4 Digital 2.5V Power:  {:.3} [V] | {:.3} [A] | {:.3} [W]", rb_vcp.drs_dvdd_voltage, rb_vcp.drs_dvdd_current, rb_vcp.drs_dvdd_power);
        println!("DRS4 Analog 2.5V Power:   {:.3} [V] | {:.3} [A] | {:.3} [W]", rb_vcp.drs_avdd_voltage, rb_vcp.drs_avdd_current, rb_vcp.drs_avdd_power);
        println!("ADC Digital 2.5V Power:   {:.3} [V] | {:.3} [A] | {:.3} [W]", rb_vcp.adc_dvdd_voltage, rb_vcp.adc_dvdd_current, rb_vcp.adc_dvdd_power);
        println!("ADC Analog 3.0V Power:    {:.3} [V] | {:.3} [A] | {:.3} [W]", rb_vcp.adc_avdd_voltage, rb_vcp.adc_avdd_current, rb_vcp.adc_avdd_power);
    }
}
