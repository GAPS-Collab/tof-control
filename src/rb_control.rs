use crate::constant::*;
use crate::memory::*;
use crate::device::{pca9548a, tmp112, lis3mdltr};

pub struct RBTemp {
    drs_temp: f32,
    clk_temp: f32,
    adc_temp: f32,
    lis3mdltr_temp: f32,
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

        i2c_mux_1.select(RB_LIS3MDLTR_CHANNEL).expect("cannot accesss to PCA9548A");
        let lis3mdltr = lis3mdltr::LIS3MDLTR::new(I2C_BUS, RB_LIS3MDLTR_ADDRESS);
        lis3mdltr.configure();
        let lis3mdltr_temp = lis3mdltr.read_temperature().expect("cannot read LIS3MDLTR");

        i2c_mux_1.reset().expect("cannot reset PCA9548A");
        i2c_mux_2.reset().expect("cannot reset PCA9548A");

        let zynq_temp_adc = read_control_reg(RB_TEMP).expect("cannot read TEMP register");
        let zynq_temp = (((zynq_temp_adc & 4095) as f32 * 503.95) / 4096.0) - 273.15;

        Self {
            drs_temp,
            clk_temp,
            adc_temp,
            lis3mdltr_temp,
            zynq_temp,
        }
    }
    pub fn print_rb_temp() {
        let rb_temp = RBTemp::new();
        println!("DRS Temperature:          {:.3}[°C]", rb_temp.drs_temp);
        println!("CLK Temperature:          {:.3}[°C]", rb_temp.clk_temp);
        println!("DRS Temperature:          {:.3}[°C]", rb_temp.adc_temp);
        println!("LIS3MDLTR Temperature:    {:.3}[°C]", rb_temp.lis3mdltr_temp);
        println!("ZYNQ Temperature:         {:.3}[°C]", rb_temp.zynq_temp);
    }
}

pub struct RBMagnetic {
    magnetic_x: f32,
    magnetic_y: f32,
    magnetic_z: f32,
    magnetic_t: f32,
}

impl RBMagnetic {
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
        let rb_magnetic = RBMagnetic::new();
        println!("Magnetic X:  {:.3}[G]", rb_magnetic.magnetic_x);
        println!("Magnetic Y:  {:.3}[G]", rb_magnetic.magnetic_y);
        println!("Magnetic Z:  {:.3}[G]", rb_magnetic.magnetic_z);
        println!("Magnetic T:  {:.3}[G]", rb_magnetic.magnetic_t);
    }
}