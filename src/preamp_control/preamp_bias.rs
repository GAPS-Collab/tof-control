use crate::constant::*;
use crate::device::{pca9548a, max11615, max11617, max5825};

pub struct PreampBiasRead {
    preamp_bias_read_1: f32,
    preamp_bias_read_2: f32,
    preamp_bias_read_3: f32,
    preamp_bias_read_4: f32,
    preamp_bias_read_5: f32,
    preamp_bias_read_6: f32,
    preamp_bias_read_7: f32,
    preamp_bias_read_8: f32,
    preamp_bias_read_9: f32,
    preamp_bias_read_10: f32,
    preamp_bias_read_11: f32,
    preamp_bias_read_12: f32,
    preamp_bias_read_13: f32,
    preamp_bias_read_14: f32,
    preamp_bias_read_15: f32,
    preamp_bias_read_16: f32,
}

impl PreampBiasRead {
    pub fn new() -> Self {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, PB_PCA9548A_ADDRESS);

        i2c_mux.select(PB_ADC_1_CHANNEL).expect("cannot access to PCA9548A");
        let max11615 = max11615::MAX11615::new(I2C_BUS, PB_MAX11615_ADDRESS);
        max11615.setup().expect("cannot setup MAX11615");
        let max11617 = max11617::MAX11617::new(I2C_BUS, PB_MAX11617_ADDRESS);
        max11617.setup().expect("cannot setup MAX11617");

        let preamp_bias_read_1 = Self::convert_bias_voltage(max11615.read(PREAMP_SEN_1_CHANNEL).expect("cannot read MAX11615"));
        let preamp_bias_read_2 = Self::convert_bias_voltage(max11615.read(PREAMP_SEN_2_CHANNEL).expect("cannot read MAX11615"));
        let preamp_bias_read_3 = Self::convert_bias_voltage(max11615.read(PREAMP_SEN_3_CHANNEL).expect("cannot read MAX11615"));
        let preamp_bias_read_4 = Self::convert_bias_voltage(max11615.read(PREAMP_SEN_4_CHANNEL).expect("cannot read MAX11615"));
        let preamp_bias_read_5 = Self::convert_bias_voltage(max11617.read(PREAMP_SEN_5_CHANNEL).expect("cannot read MAX11617"));
        let preamp_bias_read_6 = Self::convert_bias_voltage(max11617.read(PREAMP_SEN_6_CHANNEL).expect("cannot read MAX11617"));
        let preamp_bias_read_7 = Self::convert_bias_voltage(max11617.read(PREAMP_SEN_7_CHANNEL).expect("cannot read MAX11617"));
        let preamp_bias_read_8 = Self::convert_bias_voltage(max11617.read(PREAMP_SEN_8_CHANNEL).expect("cannot read MAX11617"));

        i2c_mux.select(PB_ADC_2_CHANNEL).expect("cannot access to PCA9548A");
        let max11615 = max11615::MAX11615::new(I2C_BUS, PB_MAX11615_ADDRESS);
        max11615.setup().expect("cannot setup MAX11615");
        let max11617 = max11617::MAX11617::new(I2C_BUS, PB_MAX11617_ADDRESS);
        max11617.setup().expect("cannot setup MAX11617");

        let preamp_bias_read_9 = Self::convert_bias_voltage(max11615.read(PREAMP_SEN_9_CHANNEL).expect("cannot read MAX11615"));
        let preamp_bias_read_10 = Self::convert_bias_voltage(max11615.read(PREAMP_SEN_10_CHANNEL).expect("cannot read MAX11615"));
        let preamp_bias_read_11 = Self::convert_bias_voltage(max11615.read(PREAMP_SEN_11_CHANNEL).expect("cannot read MAX11615"));
        let preamp_bias_read_12 = Self::convert_bias_voltage(max11615.read(PREAMP_SEN_12_CHANNEL).expect("cannot read MAX11615"));
        let preamp_bias_read_13 = Self::convert_bias_voltage(max11617.read(PREAMP_SEN_13_CHANNEL).expect("cannot read MAX11617"));
        let preamp_bias_read_14 = Self::convert_bias_voltage(max11617.read(PREAMP_SEN_14_CHANNEL).expect("cannot read MAX11617"));
        let preamp_bias_read_15 = Self::convert_bias_voltage(max11617.read(PREAMP_SEN_15_CHANNEL).expect("cannot read MAX11617"));
        let preamp_bias_read_16 = Self::convert_bias_voltage(max11617.read(PREAMP_SEN_16_CHANNEL).expect("cannot read MAX11617"));

        i2c_mux.reset().expect("cannot reset PCA9548A");

        Self {
            preamp_bias_read_1,
            preamp_bias_read_2,
            preamp_bias_read_3,
            preamp_bias_read_4,
            preamp_bias_read_5,
            preamp_bias_read_6,
            preamp_bias_read_7,
            preamp_bias_read_8,
            preamp_bias_read_9,
            preamp_bias_read_10,
            preamp_bias_read_11,
            preamp_bias_read_12,
            preamp_bias_read_13,
            preamp_bias_read_14,
            preamp_bias_read_15,
            preamp_bias_read_16,
        }
    }
    fn convert_bias_voltage(voltage: f32) -> f32 {
        22.27659574468085 * voltage
    }
    pub fn print_preamp_bias() {
        let preamp_bias_read = PreampBiasRead::new();
        println!("Preamp 1 Bias Voltage:    {:.3} [V]", preamp_bias_read.preamp_bias_read_1);
        println!("Preamp 2 Bias Voltage:    {:.3} [V]", preamp_bias_read.preamp_bias_read_2);
        println!("Preamp 3 Bias Voltage:    {:.3} [V]", preamp_bias_read.preamp_bias_read_3);
        println!("Preamp 4 Bias Voltage:    {:.3} [V]", preamp_bias_read.preamp_bias_read_4);
        println!("Preamp 5 Bias Voltage:    {:.3} [V]", preamp_bias_read.preamp_bias_read_5);
        println!("Preamp 6 Bias Voltage:    {:.3} [V]", preamp_bias_read.preamp_bias_read_6);
        println!("Preamp 7 Bias Voltage:    {:.3} [V]", preamp_bias_read.preamp_bias_read_7);
        println!("Preamp 8 Bias Voltage:    {:.3} [V]", preamp_bias_read.preamp_bias_read_8);
        println!("Preamp 9 Bias Voltage:    {:.3} [V]", preamp_bias_read.preamp_bias_read_9);
        println!("Preamp 10 Bias Voltage:   {:.3} [V]", preamp_bias_read.preamp_bias_read_10);
        println!("Preamp 11 Bias Voltage:   {:.3} [V]", preamp_bias_read.preamp_bias_read_11);
        println!("Preamp 12 Bias Voltage:   {:.3} [V]", preamp_bias_read.preamp_bias_read_12);
        println!("Preamp 13 Bias Voltage:   {:.3} [V]", preamp_bias_read.preamp_bias_read_13);
        println!("Preamp 14 Bias Voltage:   {:.3} [V]", preamp_bias_read.preamp_bias_read_14);
        println!("Preamp 15 Bias Voltage:   {:.3} [V]", preamp_bias_read.preamp_bias_read_15);
        println!("Preamp 16 Bias Voltage:   {:.3} [V]", preamp_bias_read.preamp_bias_read_16);
    }
}

pub struct PreampBiasSet {
}

impl PreampBiasSet {
    pub fn set_bias() {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, PB_PCA9548A_ADDRESS);

        i2c_mux.select(PB_DAC_1_CHANNEL).expect("cannot access to PCA9548A");
        let pb_dac_1 = max5825::MAX5825::new(I2C_BUS, PB_MAX5825_ADDRESS);
        pb_dac_1.coden_loadn(PREAMP_DAC_1_CHANNEL, Self::bias_to_adc(PREAMP_DAC_1_VOLTAGE)).expect("cannot set threshold 0 on MAX5825(DAC1)");
        pb_dac_1.coden_loadn(PREAMP_DAC_2_CHANNEL, Self::bias_to_adc(PREAMP_DAC_2_VOLTAGE)).expect("cannot set threshold 0 on MAX5825(DAC1)");
        pb_dac_1.coden_loadn(PREAMP_DAC_3_CHANNEL, Self::bias_to_adc(PREAMP_DAC_3_VOLTAGE)).expect("cannot set threshold 0 on MAX5825(DAC1)");
        pb_dac_1.coden_loadn(PREAMP_DAC_4_CHANNEL, Self::bias_to_adc(PREAMP_DAC_4_VOLTAGE)).expect("cannot set threshold 0 on MAX5825(DAC1)");
        pb_dac_1.coden_loadn(PREAMP_DAC_5_CHANNEL, Self::bias_to_adc(PREAMP_DAC_5_VOLTAGE)).expect("cannot set threshold 0 on MAX5825(DAC1)");
        pb_dac_1.coden_loadn(PREAMP_DAC_6_CHANNEL, Self::bias_to_adc(PREAMP_DAC_6_VOLTAGE)).expect("cannot set threshold 0 on MAX5825(DAC1)");
        pb_dac_1.coden_loadn(PREAMP_DAC_7_CHANNEL, Self::bias_to_adc(PREAMP_DAC_7_VOLTAGE)).expect("cannot set threshold 0 on MAX5825(DAC1)");
        pb_dac_1.coden_loadn(PREAMP_DAC_8_CHANNEL, Self::bias_to_adc(PREAMP_DAC_8_VOLTAGE)).expect("cannot set threshold 0 on MAX5825(DAC1)");
    
        i2c_mux.select(PB_DAC_2_CHANNEL).expect("cannot access to PCA9548A");
        let pb_dac_2 = max5825::MAX5825::new(I2C_BUS, PB_MAX5825_ADDRESS);
        pb_dac_2.coden_loadn(PREAMP_DAC_9_CHANNEL, Self::bias_to_adc(PREAMP_DAC_9_VOLTAGE)).expect("cannot set threshold 0 on MAX5825(DAC2)");
        pb_dac_2.coden_loadn(PREAMP_DAC_10_CHANNEL, Self::bias_to_adc(PREAMP_DAC_10_VOLTAGE)).expect("cannot set threshold 0 on MAX5825(DAC2)");
        pb_dac_2.coden_loadn(PREAMP_DAC_11_CHANNEL, Self::bias_to_adc(PREAMP_DAC_11_VOLTAGE)).expect("cannot set threshold 0 on MAX5825(DAC2)");
        pb_dac_2.coden_loadn(PREAMP_DAC_12_CHANNEL, Self::bias_to_adc(PREAMP_DAC_12_VOLTAGE)).expect("cannot set threshold 0 on MAX5825(DAC2)");
        pb_dac_2.coden_loadn(PREAMP_DAC_13_CHANNEL, Self::bias_to_adc(PREAMP_DAC_13_VOLTAGE)).expect("cannot set threshold 0 on MAX5825(DAC2)");
        pb_dac_2.coden_loadn(PREAMP_DAC_14_CHANNEL, Self::bias_to_adc(PREAMP_DAC_14_VOLTAGE)).expect("cannot set threshold 0 on MAX5825(DAC2)");
        pb_dac_2.coden_loadn(PREAMP_DAC_15_CHANNEL, Self::bias_to_adc(PREAMP_DAC_15_VOLTAGE)).expect("cannot set threshold 0 on MAX5825(DAC2)");
        pb_dac_2.coden_loadn(PREAMP_DAC_16_CHANNEL, Self::bias_to_adc(PREAMP_DAC_16_VOLTAGE)).expect("cannot set threshold 0 on MAX5825(DAC2)");

        i2c_mux.reset().expect("cannot reset PCA9548A");
    }
    fn bias_to_adc(bias_voltage: f32) -> u16 {
        let adc = (bias_voltage / 22.3) / PB_DAC_REF_VOLTAGE * 2f32.powf(12.0);

        adc as u16
    }
    pub fn reset_bias() {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, PB_PCA9548A_ADDRESS);

        i2c_mux.select(PB_DAC_1_CHANNEL).expect("cannot access to PCA9548A");
        let pb_dac_1 = max5825::MAX5825::new(I2C_BUS, PB_MAX5825_ADDRESS);
        pb_dac_1.reset_dac().expect("cannot reset MAX5825(DAC1)");

        i2c_mux.select(PB_DAC_2_CHANNEL).expect("cannot access to PCA9548A");
        let pb_dac_2 = max5825::MAX5825::new(I2C_BUS, PB_MAX5825_ADDRESS);

        pb_dac_2.reset_dac().expect("cannot reset MAX5825(DAC2)");
    }
}