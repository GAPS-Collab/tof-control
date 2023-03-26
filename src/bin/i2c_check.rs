use tof_control::constant::*;
use tof_control::device::{pca9548a, tmp112, ina226, lis3mdltr, bme280, max11645, si5345b, cy8c9560a, ad5675};
use tof_control::rb_control::*;

fn main() {
    let i2c_mux_1 = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_1);
    let i2c_mux_2 = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);

    println!("Checking I2C Multiplexer 1 ({:#02X})", RB_PCA9548A_ADDRESS_1);

    // BME280
    i2c_mux_1.select(RB_BME280_CHANNEL).expect("cannot accesss to PCA9548A");
    let bme280 = bme280::BME280::new(I2C_BUS, RB_BME280_ADDRESS).read_data();
    let bme280 = match bme280 {
        Ok(_f32) => println!("Passed => Channel  {}: BME280 ({:#02X})", RB_BME280_CHANNEL, RB_BME280_ADDRESS),
        Err(_LinuxI2CError) => println!("\x1b[91mFailed\x1b[0m => Channel  {}: BME280 ({:#02X})", RB_BME280_CHANNEL, RB_BME280_ADDRESS)
    };

    // LIS3MDLTR
    i2c_mux_1.select(RB_LIS3MDLTR_CHANNEL).expect("cannot accesss to PCA9548A");
    let lis3mdltr = lis3mdltr::LIS3MDLTR::new(I2C_BUS, RB_LIS3MDLTR_ADDRESS).read_id();
    let lis3mdltr = match lis3mdltr {
        Ok(_u8) => println!("Passed => Channel  {}: LIS3MDLTR ({:#02X})", RB_LIS3MDLTR_CHANNEL, RB_LIS3MDLTR_ADDRESS),
        Err(_LinuxI2CError) => println!("\x1b[91mFailed\x1b[0m => Channel  {}: LIS3MDLTR ({:#02X})", RB_LIS3MDLTR_CHANNEL, RB_LIS3MDLTR_ADDRESS)
    };

    // TMP112 (DRS4)
    i2c_mux_1.select(RB_DRS_TMP112_CHANNEL).expect("cannot accesss to PCA9548A");
    let drs_tmp112 = tmp112::TMP112::new(I2C_BUS, RB_DRS_TMP112_ADDRESS).read();
    let drs_tmp112 = match drs_tmp112 {
        Ok(_f32) => println!("Passed => Channel  {}: DRS4 TMP112 ({:#02X})", RB_DRS_TMP112_CHANNEL, RB_DRS_TMP112_ADDRESS),
        Err(_LinuxI2CError) => println!("\x1b[91mFailed\x1b[0m => Channel  {}: DRS4 TMP112 ({:#02X})", RB_DRS_TMP112_CHANNEL, RB_DRS_TMP112_ADDRESS)
    };

    // INA226 (DRS4 DVDD)
    i2c_mux_1.select(RB_DRS_DVDD_INA226_CHANNEL).expect("cannot accesss to PCA9548A");
    let drs_dvdd_ina226 = ina226::INA226::new(I2C_BUS, RB_DRS_DVDD_INA226_ADDRESS, RB_DRS_DVDD_INA226_RSHUNT, RB_DRS_DVDD_INA226_MEC).read_data();
    let drs_dvdd_ina226 = match drs_dvdd_ina226 {
        Ok(_f32) => println!("Passed => Channel  {}: DRS4 DVDD INA226 ({:#02X})", RB_DRS_DVDD_INA226_CHANNEL, RB_DRS_DVDD_INA226_ADDRESS),
        Err(_LinuxI2CError) => println!("\x1b[91mFailed\x1b[0m => Channel  {}: DRS4 DVDD INA226 ({:#02X})", RB_DRS_DVDD_INA226_CHANNEL, RB_DRS_DVDD_INA226_ADDRESS)
    };

    // INA226 (3.3V)
    i2c_mux_1.select(RB_P3V3_INA226_CHANNEL).expect("cannot accesss to PCA9548A");
    let p3v3_ina226 = ina226::INA226::new(I2C_BUS, RB_P3V3_INA226_ADDRESS, RB_P3V3_INA226_RSHUNT, RB_P3V3_INA226_MEC).read_data();
    let p3v3_ina226 = match p3v3_ina226 {
        Ok(_f32) => println!("Passed => Channel  {}: 3.3V INA226 ({:#02X})", RB_P3V3_INA226_CHANNEL, RB_P3V3_INA226_ADDRESS),
        Err(_LinuxI2CError) => println!("\x1b[91mFailed\x1b[0m => Channel  {}: 3.3V INA226 ({:#02X})", RB_P3V3_INA226_CHANNEL, RB_P3V3_INA226_ADDRESS)
    };

    // INA226 (ZYNQ)
    i2c_mux_1.select(RB_ZYNQ_INA226_CHANNEL).expect("cannot accesss to PCA9548A");
    let zynq_ina226 = ina226::INA226::new(I2C_BUS, RB_ZYNQ_INA226_ADDRESS, RB_ZYNQ_INA226_RSHUNT, RB_ZYNQ_INA226_MEC).read_data();
    let zynq_ina226 = match zynq_ina226 {
        Ok(_f32) => println!("Passed => Channel  {}: ZYNQ INA226 ({:#02X})", RB_ZYNQ_INA226_CHANNEL, RB_ZYNQ_INA226_ADDRESS),
        Err(_LinuxI2CError) => println!("\x1b[91mFailed\x1b[0m => Channel  {}: ZYNQ INA226 ({:#02X})", RB_ZYNQ_INA226_CHANNEL, RB_ZYNQ_INA226_ADDRESS)
    };

    // MAX11645
    i2c_mux_1.select(RB_MAX11645_CHANNEL).expect("cannot accesss to PCA9548A");
    let max11645 = max11645::MAX11645::new(I2C_BUS, RB_MAX11645_ADDRESS).read(RB_N1V5_CURRENT_INA200_CHANNEL);
    let max11645 = match max11645 {
        Ok(_f32) => println!("Passed => Channel  {}: MAX11645 ({:#02X})", RB_MAX11645_CHANNEL, RB_MAX11645_ADDRESS),
        Err(_LinuxI2CError) => println!("\x1b[91mFailed\x1b[0m => Channel  {}: MAX11645 ({:#02X})", RB_MAX11645_CHANNEL, RB_MAX11645_ADDRESS)
    };

    // INA226 (3.5V)
    i2c_mux_1.select(RB_P3V5_INA226_CHANNEL).expect("cannot accesss to PCA9548A");
    let p3v5_ina226 = ina226::INA226::new(I2C_BUS, RB_P3V5_INA226_ADDRESS, RB_P3V5_INA226_RSHUNT, RB_P3V5_INA226_MEC).read_data();
    let p3v5_ina226 = match p3v5_ina226 {
        Ok(_f32) => println!("Passed => Channel  {}: 3.5V INA226 ({:#02X})", RB_P3V5_INA226_CHANNEL, RB_P3V5_INA226_ADDRESS),
        Err(_LinuxI2CError) => println!("\x1b[91mFailed\x1b[0m => Channel  {}: 3.5V INA226 ({:#02X})", RB_P3V5_INA226_CHANNEL, RB_P3V5_INA226_ADDRESS)
    };

    i2c_mux_1.reset().expect("cannot reset PCA9548A");

    println!("");

    println!("Checking I2C Multiplexer 2 ({:#02X})", RB_PCA9548A_ADDRESS_2);

    // TMP112 (CLK)
    i2c_mux_2.select(RB_CLK_TMP112_CHANNEL).expect("cannot accesss to PCA9548A");
    let clk_tmp112 = tmp112::TMP112::new(I2C_BUS, RB_CLK_TMP112_ADDRESS).read();
    let clk_tmp112 = match clk_tmp112 {
        Ok(_f32) => println!("Passed => Channel  {}: CLK TMP112 ({:#02X})", RB_CLK_TMP112_CHANNEL, RB_CLK_TMP112_ADDRESS),
        Err(_LinuxI2CError) => println!("\x1b[91mFailed\x1b[0m => Channel  {}: CLK TMP112 ({:#02X})", RB_CLK_TMP112_CHANNEL, RB_CLK_TMP112_ADDRESS)
    };

    // SI5345
    i2c_mux_2.select(RB_SI5345B_CHANNEL).expect("cannot accesss to PCA9548A");
    let si5345b = si5345b::SI5345B::new(I2C_BUS, RB_SI5345B_ADDRESS).read_lol_status();
    let si5345b = match si5345b {
        Ok(_bool) => println!("Passed => Channel  {}: SI5345 ({:#02X})", RB_SI5345B_CHANNEL, RB_SI5345B_ADDRESS),
        Err(_LinuxI2CError) => println!("\x1b[91mFailed\x1b[0m => Channel  {}: SI5345 ({:#02X})", RB_SI5345B_CHANNEL, RB_SI5345B_ADDRESS)
    };

    // // AD5675
    // rb_gpioe::RBgpioe::dac_reset();
    // i2c_mux_2.select(RB_AD5675_CHANNEL).expect("cannot accesss to PCA9548A");
    // let ad5675 = ad5675::AD5675::new(I2C_BUS, RB_AD5675_ADDRESS).read_dac(0);
    // let ad5675 = match ad5675 {
    //     Ok(_u16) => println!("Passed => Channel  {}: AD5675 ({:#02X})", RB_AD5675_CHANNEL, RB_AD5675_ADDRESS),
    //     Err(_LinuxI2CError) => println!("\x1b[91mFailed\x1b[0m => Channel  {}: AD5675 ({:#02X})", RB_AD5675_CHANNEL, RB_AD5675_ADDRESS)
    // };

    // INA226 (ADC DVDD)
    i2c_mux_2.select(RB_ADC_DVDD_INA226_CHANNEL).expect("cannot accesss to PCA9548A");
    let adc_dvdd_ina226 = ina226::INA226::new(I2C_BUS, RB_ADC_DVDD_INA226_ADDRESS, RB_ADC_DVDD_INA226_RSHUNT, RB_ADC_DVDD_INA226_MEC).read_data();
    let adc_dvdd_ina226 = match adc_dvdd_ina226 {
        Ok(_f32) => println!("Passed => Channel  {}: ADC DVDD INA226 ({:#02X})", RB_ADC_DVDD_INA226_CHANNEL, RB_ADC_DVDD_INA226_ADDRESS),
        Err(_LinuxI2CError) => println!("\x1b[91mFailed\x1b[0m => Channel  {}: ADC DVDD INA226 ({:#02X})", RB_ADC_DVDD_INA226_CHANNEL, RB_ADC_DVDD_INA226_ADDRESS)
    };

    // TMP112 (ADC)
    i2c_mux_2.select(RB_ADC_TMP112_CHANNEL).expect("cannot accesss to PCA9548A");
    let adc_tmp112 = tmp112::TMP112::new(I2C_BUS, RB_ADC_TMP112_ADDRESS).read();
    let adc_tmp112 = match adc_tmp112 {
        Ok(_f32) => println!("Passed => Channel  {}: ADC TMP112 ({:#02X})", RB_ADC_TMP112_CHANNEL, RB_ADC_TMP112_ADDRESS),
        Err(_LinuxI2CError) => println!("\x1b[91mFailed\x1b[0m => Channel  {}: ADC TMP112 ({:#02X})", RB_ADC_TMP112_CHANNEL, RB_ADC_TMP112_ADDRESS)
    };

    // INA226 (ADC AVDD)
    i2c_mux_2.select(RB_ADC_AVDD_INA226_CHANNEL).expect("cannot accesss to PCA9548A");
    let adc_avdd_ina226 = ina226::INA226::new(I2C_BUS, RB_ADC_AVDD_INA226_ADDRESS, RB_ADC_AVDD_INA226_RSHUNT, RB_ADC_AVDD_INA226_MEC).read_data();
    let adc_avdd_ina226 = match adc_avdd_ina226 {
        Ok(_f32) => println!("Passed => Channel  {}: ADC AVDD INA226 ({:#02X})", RB_ADC_AVDD_INA226_CHANNEL, RB_ADC_AVDD_INA226_ADDRESS),
        Err(_LinuxI2CError) => println!("\x1b[91mFailed\x1b[0m => Channel  {}: ADC AVDD INA226 ({:#02X})", RB_ADC_AVDD_INA226_CHANNEL, RB_ADC_AVDD_INA226_ADDRESS)
    };

    // INA 226 (DRS4 AVDD)
    i2c_mux_2.select(RB_DRS_AVDD_INA226_CHANNEL).expect("cannot accesss to PCA9548A");
    let drs_avdd_ina226 = ina226::INA226::new(I2C_BUS, RB_DRS_AVDD_INA226_ADDRESS, RB_DRS_AVDD_INA226_RSHUNT, RB_DRS_AVDD_INA226_MEC).read_data();
    let drs_avdd_ina226 = match drs_avdd_ina226 {
        Ok(_f32) => println!("Passed => Channel  {}: DRS4 AVDD INA226 ({:#02X})", RB_DRS_AVDD_INA226_CHANNEL, RB_DRS_AVDD_INA226_ADDRESS),
        Err(_LinuxI2CError) => println!("\x1b[91mFailed\x1b[0m => Channel  {}: DRS4 AVDD INA226 ({:#02X})", RB_DRS_AVDD_INA226_CHANNEL, RB_DRS_AVDD_INA226_ADDRESS)
    };

    // CY8C9560A
    i2c_mux_2.select(RB_CY8C9560A_CHANNEL).expect("cannot accesss to PCA9548A");
    let cy8c9560a = cy8c9560a::CY8C9560A::new(I2C_BUS, RB_CY8C9560A_ADDRESS).read_device_info();
    let cy8c9560a = match cy8c9560a {
        Ok(_u8) => println!("Passed => Channel  {}: CY8C9560A ({:#02X})", RB_CY8C9560A_CHANNEL, RB_CY8C9560A_ADDRESS),
        Err(_LinuxI2CError) => println!("\x1b[91mFailed\x1b[0m => Channel  {}: CY8C9560A ({:#02X})", RB_CY8C9560A_CHANNEL, RB_CY8C9560A_ADDRESS)
    };

    i2c_mux_2.reset().expect("cannot reset PCA9548A");

    println!("");

    println!("I2C Test Done");

}