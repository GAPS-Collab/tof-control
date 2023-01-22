use crate::constant::*;
use crate::device::{pca9548a, max11615, max11617};

pub struct PreampTemp {
    preamp_tmp_1: f32,
    preamp_tmp_2: f32,
    preamp_tmp_3: f32,
    preamp_tmp_4: f32,
    preamp_tmp_5: f32,
    preamp_tmp_6: f32,
    preamp_tmp_7: f32,
    preamp_tmp_8: f32,
    preamp_tmp_9: f32,
    preamp_tmp_10: f32,
    preamp_tmp_11: f32,
    preamp_tmp_12: f32,
    preamp_tmp_13: f32,
    preamp_tmp_14: f32,
    preamp_tmp_15: f32,
    preamp_tmp_16: f32,
}

impl PreampTemp {
    pub fn new() -> Self {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, PB_PCA9548A_ADDRESS);

        i2c_mux.select(PB_ADC_1_CHANNEL).expect("cannot access to PCA9548A");
        let max11615 = max11615::MAX11615::new(I2C_BUS, PB_MAX11615_ADDRESS);
        max11615.setup().expect("cannot setup MAX11615");
        let max11617 = max11617::MAX11617::new(I2C_BUS, PB_MAX11617_ADDRESS);
        max11617.setup().expect("cannot setup MAX11617");

        let preamp_tmp_1 = Self::voltage_to_temp(max11615.read(PREAMP_TEMP_1_CHNANNEL).expect("cannot read MAX11615"));
        let preamp_tmp_2 = Self::voltage_to_temp(max11615.read(PREAMP_TEMP_2_CHNANNEL).expect("cannot read MAX11615"));
        let preamp_tmp_3 = Self::voltage_to_temp(max11615.read(PREAMP_TEMP_3_CHNANNEL).expect("cannot read MAX11615"));
        let preamp_tmp_4 = Self::voltage_to_temp(max11615.read(PREAMP_TEMP_4_CHNANNEL).expect("cannot read MAX11615"));
        let preamp_tmp_5 = Self::voltage_to_temp(max11617.read(PREAMP_TEMP_5_CHNANNEL).expect("cannot read MAX11617"));
        let preamp_tmp_6 = Self::voltage_to_temp(max11617.read(PREAMP_TEMP_6_CHNANNEL).expect("cannot read MAX11617"));
        let preamp_tmp_7 = Self::voltage_to_temp(max11617.read(PREAMP_TEMP_7_CHNANNEL).expect("cannot read MAX11617"));
        let preamp_tmp_8 = Self::voltage_to_temp(max11617.read(PREAMP_TEMP_8_CHNANNEL).expect("cannot read MAX11617"));

        i2c_mux.select(PB_ADC_2_CHANNEL).expect("cannot access to PCA9548A");
        let max11615 = max11615::MAX11615::new(I2C_BUS, PB_MAX11615_ADDRESS);
        max11615.setup().expect("cannot setup MAX11615");
        let max11617 = max11617::MAX11617::new(I2C_BUS, PB_MAX11617_ADDRESS);
        max11617.setup().expect("cannot setup MAX11617");

        let preamp_tmp_9 = Self::voltage_to_temp(max11615.read(PREAMP_TEMP_9_CHNANNEL).expect("cannot read MAX11615"));
        let preamp_tmp_10 = Self::voltage_to_temp(max11615.read(PREAMP_TEMP_10_CHNANNEL).expect("cannot read MAX11615"));
        let preamp_tmp_11 = Self::voltage_to_temp(max11615.read(PREAMP_TEMP_11_CHNANNEL).expect("cannot read MAX11615"));
        let preamp_tmp_12 = Self::voltage_to_temp(max11615.read(PREAMP_TEMP_12_CHNANNEL).expect("cannot read MAX11615"));
        let preamp_tmp_13 = Self::voltage_to_temp(max11617.read(PREAMP_TEMP_13_CHNANNEL).expect("cannot read MAX11617"));
        let preamp_tmp_14 = Self::voltage_to_temp(max11617.read(PREAMP_TEMP_14_CHNANNEL).expect("cannot read MAX11617"));
        let preamp_tmp_15 = Self::voltage_to_temp(max11617.read(PREAMP_TEMP_15_CHNANNEL).expect("cannot read MAX11617"));
        let preamp_tmp_16 = Self::voltage_to_temp(max11617.read(PREAMP_TEMP_16_CHNANNEL).expect("cannot read MAX11617"));

        i2c_mux.reset().expect("cannot reset PCA9548A");

        Self {
            preamp_tmp_1,
            preamp_tmp_2,
            preamp_tmp_3,
            preamp_tmp_4,
            preamp_tmp_5,
            preamp_tmp_6,
            preamp_tmp_7,
            preamp_tmp_8,
            preamp_tmp_9,
            preamp_tmp_10,
            preamp_tmp_11,
            preamp_tmp_12,
            preamp_tmp_13,
            preamp_tmp_14,
            preamp_tmp_15,
            preamp_tmp_16,
        }
    }
    pub fn print_preamp_temp() {
        let preamp_temp = PreampTemp::new();
        println!("Preamp Board 1 Temperature:   {:.3}[°C]", preamp_temp.preamp_tmp_1);
        println!("Preamp Board 2 Temperature:   {:.3}[°C]", preamp_temp.preamp_tmp_2);
        println!("Preamp Board 3 Temperature:   {:.3}[°C]", preamp_temp.preamp_tmp_3);
        println!("Preamp Board 4 Temperature:   {:.3}[°C]", preamp_temp.preamp_tmp_4);
        println!("Preamp Board 5 Temperature:   {:.3}[°C]", preamp_temp.preamp_tmp_5);
        println!("Preamp Board 6 Temperature:   {:.3}[°C]", preamp_temp.preamp_tmp_6);
        println!("Preamp Board 7 Temperature:   {:.3}[°C]", preamp_temp.preamp_tmp_7);
        println!("Preamp Board 8 Temperature:   {:.3}[°C]", preamp_temp.preamp_tmp_8);
        println!("Preamp Board 9 Temperature:   {:.3}[°C]", preamp_temp.preamp_tmp_9);
        println!("Preamp Board 10 Temperature:  {:.3}[°C]", preamp_temp.preamp_tmp_10);
        println!("Preamp Board 11 Temperature:  {:.3}[°C]", preamp_temp.preamp_tmp_11);
        println!("Preamp Board 12 Temperature:  {:.3}[°C]", preamp_temp.preamp_tmp_12);
        println!("Preamp Board 13 Temperature:  {:.3}[°C]", preamp_temp.preamp_tmp_13);
        println!("Preamp Board 14 Temperature:  {:.3}[°C]", preamp_temp.preamp_tmp_14);
        println!("Preamp Board 15 Temperature:  {:.3}[°C]", preamp_temp.preamp_tmp_15);
        println!("Preamp Board 16 Temperature:  {:.3}[°C]", preamp_temp.preamp_tmp_16);
    }
    fn voltage_to_temp(voltage: f32) -> f32 {
        let mut temperature = (voltage - 0.5) * 100.0;
        if -40.0 > temperature || temperature > 150.0 {
            temperature = 500.0;
        }

        temperature
    }
}