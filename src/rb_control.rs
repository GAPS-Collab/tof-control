use crate::constant::*;
use crate::memory::*;
use crate::device::{pca9548a, tmp112};

pub struct RBTemp {
    drs_temp: f32,
    clk_temp: f32,
    adc_temp: f32,
    zynq_temp: f32,
}

impl RBTemp {
    pub fn new() -> Self {
        let i2c_mux_1 = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_1);
        let i2c_mux_2 = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
        
        i2c_mux_1.select(RB_DRS_TMP112_CHANNEL).expect("cannot accesss to PCA9548A");
        let drs_tmp112 = tmp112::TMP112::new(I2C_BUS, RB_DRS_TMP112_ADDRESS);
        drs_tmp112.config().expect("cannot configure TMP112");
        let drs_temp = drs_tmp112.read().expect("cannot read TMP112");
    
        i2c_mux_2.select(RB_CLK_TMP112_CHANNEL).expect("cannot accesss to PCA9548A");
        let clk_tmp112 = tmp112::TMP112::new(I2C_BUS, RB_CLK_TMP112_ADDRESS);
        clk_tmp112.config().expect("cannot configure TMP112");
        let clk_temp = clk_tmp112.read().expect("cannot read TMP112");
    
        i2c_mux_2.select(RB_ADC_TMP112_CHANNEL).expect("cannot accesss to PCA9548A");
        let adc_tmp112 = tmp112::TMP112::new(I2C_BUS, RB_ADC_TMP112_ADDRESS);
        adc_tmp112.config().expect("cannot configure TMP112");
        let adc_temp = adc_tmp112.read().expect("cannot read TMP112");

        i2c_mux_1.reset().expect("cannot reset PCA9548A");
        i2c_mux_2.reset().expect("cannot reset PCA9548A");

        let zynq_temp_adc = read_control_reg(RB_TEMP).expect("cannot read TEMP register");
        let zynq_temp = (((zynq_temp_adc & 4095) as f32 * 503.95) / 4096.0) - 273.15;

        Self {
            drs_temp,
            clk_temp,
            adc_temp,
            zynq_temp,
        }
    }
    pub fn print_rb_temp() {
        let rb_temp = RBTemp::new();
        println!("DRS Temperature:  {:.3}[°C]", rb_temp.drs_temp);
        println!("CLK Temperature:  {:.3}[°C]", rb_temp.clk_temp);
        println!("DRS Temperature:  {:.3}[°C]", rb_temp.adc_temp);
        println!("ZYNQ Temperature: {:.3}[°C]", rb_temp.zynq_temp);
    }
}