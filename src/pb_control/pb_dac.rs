use crate::constant::*;
use crate::device::{pca9548a, max5825};

pub struct PBdac {
    device_id_1: u8,
    wd_enabled_1: u8,
    ref_mode_1: u8,
    clr_enabled_1: u8,
    rev_id_1: u8,
    device_id_2: u8,
    wd_enabled_2: u8,
    ref_mode_2: u8,
    clr_enabled_2: u8,
    rev_id_2: u8,
    code_ch1: u16,
    code_ch2: u16,
    code_ch3: u16,
    code_ch4: u16,
    code_ch5: u16,
    code_ch6: u16,
    code_ch7: u16,
    code_ch8: u16,
    code_ch9: u16,
    code_ch10: u16,
    code_ch11: u16,
    code_ch12: u16,
    code_ch13: u16,
    code_ch14: u16,
    code_ch15: u16,
    code_ch16: u16,
    dac_ch1: u16,
    dac_ch2: u16,
    dac_ch3: u16,
    dac_ch4: u16,
    dac_ch5: u16,
    dac_ch6: u16,
    dac_ch7: u16,
    dac_ch8: u16,
    dac_ch9: u16,
    dac_ch10: u16,
    dac_ch11: u16,
    dac_ch12: u16,
    dac_ch13: u16,
    dac_ch14: u16,
    dac_ch15: u16,
    dac_ch16: u16,
}

impl PBdac {
    pub fn new() -> Self {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, PB_PCA9548A_ADDRESS);

        i2c_mux.select(PB_DAC_1_CHANNEL).expect("cannot access to PCA9548A");
        let pb_dac_1 = max5825::MAX5825::new(I2C_BUS, PB_MAX5825_ADDRESS);
        let (wd_enabled_1, ref_mode_1, clr_enabled_1, rev_id_1, device_id_1) = pb_dac_1.read_device_info().expect("cannot read MAX5825(1)");

        let code_ch1 = pb_dac_1.read_coden(PREAMP_DAC_1_CHANNEL).expect("cannot read MAX5825");
        let dac_ch1 = pb_dac_1.read_dacn(PREAMP_DAC_1_CHANNEL).expect("cannot read MAX5825");
        let code_ch2 = pb_dac_1.read_coden(PREAMP_DAC_2_CHANNEL).expect("cannot read MAX5825");
        let dac_ch2 = pb_dac_1.read_dacn(PREAMP_DAC_2_CHANNEL).expect("cannot read MAX5825");
        let code_ch3 = pb_dac_1.read_coden(PREAMP_DAC_3_CHANNEL).expect("cannot read MAX5825");
        let dac_ch3 = pb_dac_1.read_dacn(PREAMP_DAC_3_CHANNEL).expect("cannot read MAX5825");
        let code_ch4 = pb_dac_1.read_coden(PREAMP_DAC_4_CHANNEL).expect("cannot read MAX5825");
        let dac_ch4 = pb_dac_1.read_dacn(PREAMP_DAC_4_CHANNEL).expect("cannot read MAX5825");
        let code_ch5 = pb_dac_1.read_coden(PREAMP_DAC_1_CHANNEL).expect("cannot read MAX5825");
        let dac_ch5 = pb_dac_1.read_dacn(PREAMP_DAC_1_CHANNEL).expect("cannot read MAX5825");
        let code_ch6 = pb_dac_1.read_coden(PREAMP_DAC_6_CHANNEL).expect("cannot read MAX5825");
        let dac_ch6 = pb_dac_1.read_dacn(PREAMP_DAC_6_CHANNEL).expect("cannot read MAX5825");
        let code_ch7 = pb_dac_1.read_coden(PREAMP_DAC_7_CHANNEL).expect("cannot read MAX5825");
        let dac_ch7 = pb_dac_1.read_dacn(PREAMP_DAC_7_CHANNEL).expect("cannot read MAX5825");
        let code_ch8 = pb_dac_1.read_coden(PREAMP_DAC_8_CHANNEL).expect("cannot read MAX5825");
        let dac_ch8 = pb_dac_1.read_dacn(PREAMP_DAC_8_CHANNEL).expect("cannot read MAX5825");

        i2c_mux.select(PB_DAC_2_CHANNEL).expect("cannot access to PCA9548A");
        let pb_dac_2 = max5825::MAX5825::new(I2C_BUS, PB_MAX5825_ADDRESS);
        let (wd_enabled_2, ref_mode_2, clr_enabled_2, rev_id_2, device_id_2) = pb_dac_2.read_device_info().expect("cannot read MAX5825(2)");

        let code_ch9 = pb_dac_2.read_coden(PREAMP_DAC_9_CHANNEL).expect("cannot read MAX5825");
        let dac_ch9 = pb_dac_2.read_dacn(PREAMP_DAC_9_CHANNEL).expect("cannot read MAX5825");
        let code_ch10 = pb_dac_2.read_coden(PREAMP_DAC_10_CHANNEL).expect("cannot read MAX5825");
        let dac_ch10 = pb_dac_2.read_dacn(PREAMP_DAC_10_CHANNEL).expect("cannot read MAX5825");
        let code_ch11 = pb_dac_2.read_coden(PREAMP_DAC_11_CHANNEL).expect("cannot read MAX5825");
        let dac_ch11 = pb_dac_2.read_dacn(PREAMP_DAC_11_CHANNEL).expect("cannot read MAX5825");
        let code_ch12 = pb_dac_2.read_coden(PREAMP_DAC_12_CHANNEL).expect("cannot read MAX5825");
        let dac_ch12 = pb_dac_2.read_dacn(PREAMP_DAC_12_CHANNEL).expect("cannot read MAX5825");
        let code_ch13 = pb_dac_2.read_coden(PREAMP_DAC_13_CHANNEL).expect("cannot read MAX5825");
        let dac_ch13 = pb_dac_2.read_dacn(PREAMP_DAC_13_CHANNEL).expect("cannot read MAX5825");
        let code_ch14 = pb_dac_2.read_coden(PREAMP_DAC_14_CHANNEL).expect("cannot read MAX5825");
        let dac_ch14 = pb_dac_2.read_dacn(PREAMP_DAC_14_CHANNEL).expect("cannot read MAX5825");
        let code_ch15 = pb_dac_2.read_coden(PREAMP_DAC_15_CHANNEL).expect("cannot read MAX5825");
        let dac_ch15 = pb_dac_2.read_dacn(PREAMP_DAC_15_CHANNEL).expect("cannot read MAX5825");
        let code_ch16 = pb_dac_2.read_coden(PREAMP_DAC_16_CHANNEL).expect("cannot read MAX5825");
        let dac_ch16 = pb_dac_2.read_dacn(PREAMP_DAC_16_CHANNEL).expect("cannot read MAX5825");

        i2c_mux.reset().expect("cannot reset PCA9548A");

        Self {
            device_id_1,
            wd_enabled_1,
            ref_mode_1,
            clr_enabled_1,
            rev_id_1,
            device_id_2,
            wd_enabled_2,
            ref_mode_2,
            clr_enabled_2,
            rev_id_2,
            code_ch1,
            code_ch2,
            code_ch3,
            code_ch4,
            code_ch5,
            code_ch6,
            code_ch7,
            code_ch8,
            code_ch9,
            code_ch10,
            code_ch11,
            code_ch12,
            code_ch13,
            code_ch14,
            code_ch15,
            code_ch16,
            dac_ch1,
            dac_ch2,
            dac_ch3,
            dac_ch4,
            dac_ch5,
            dac_ch6,
            dac_ch7,
            dac_ch8,
            dac_ch9,
            dac_ch10,
            dac_ch11,
            dac_ch12,
            dac_ch13,
            dac_ch14,
            dac_ch15,
            dac_ch16,
        }
    }
    pub fn print_pb_dac() {
        let pb_dac = PBdac::new();
        println!("--- Device Information ---");
        println!("Device ID 1:        {}", pb_dac.device_id_1);
        println!("Device ID 2:        {}", pb_dac.device_id_2);
        println!("Revision ID 1:      {}", pb_dac.rev_id_1);
        println!("Revision ID 2:      {}", pb_dac.rev_id_2);
        println!("Reference Mode 1:   {}", pb_dac.ref_mode_1);
        println!("Reference Mode 2:   {}", pb_dac.ref_mode_2);
        println!("Watchdog Status 1:  {}", pb_dac.wd_enabled_1);
        println!("Watchdog Status 2:  {}", pb_dac.wd_enabled_2);
        println!("Clear Status 1:     {}", pb_dac.clr_enabled_1);
        println!("Clear Status 2:     {}", pb_dac.clr_enabled_2);
        println!("--- DAC(CODE) ---");
        println!("Ch1 DAC:            {:.3} [V]({})", Self::adc_to_bias(pb_dac.dac_ch1), pb_dac.code_ch1);
        println!("Ch2 DAC:            {:.3} [V]({})", Self::adc_to_bias(pb_dac.dac_ch2), pb_dac.code_ch2);
        println!("Ch3 DAC:            {:.3} [V]({})", Self::adc_to_bias(pb_dac.dac_ch3), pb_dac.code_ch3);
        println!("Ch4 DAC:            {:.3} [V]({})", Self::adc_to_bias(pb_dac.dac_ch4), pb_dac.code_ch4);
        println!("Ch5 DAC:            {:.3} [V]({})", Self::adc_to_bias(pb_dac.dac_ch5), pb_dac.code_ch5);
        println!("Ch6 DAC:            {:.3} [V]({})", Self::adc_to_bias(pb_dac.dac_ch6), pb_dac.code_ch6);
        println!("Ch7 DAC:            {:.3} [V]({})", Self::adc_to_bias(pb_dac.dac_ch7), pb_dac.code_ch7);
        println!("Ch8 DAC:            {:.3} [V]({})", Self::adc_to_bias(pb_dac.dac_ch8), pb_dac.code_ch8);
        println!("Ch9 DAC:            {:.3} [V]({})", Self::adc_to_bias(pb_dac.dac_ch9), pb_dac.code_ch9);
        println!("Ch10 DAC:           {:.3} [V]({})", Self::adc_to_bias(pb_dac.dac_ch10), pb_dac.code_ch10);
        println!("Ch11 DAC:           {:.3} [V]({})", Self::adc_to_bias(pb_dac.dac_ch11), pb_dac.code_ch11);
        println!("Ch12 DAC:           {:.3} [V]({})", Self::adc_to_bias(pb_dac.dac_ch12), pb_dac.code_ch12);
        println!("Ch13 DAC:           {:.3} [V]({})", Self::adc_to_bias(pb_dac.dac_ch13), pb_dac.code_ch13);
        println!("Ch14 DAC:           {:.3} [V]({})", Self::adc_to_bias(pb_dac.dac_ch14), pb_dac.code_ch14);
        println!("Ch15 DAC:           {:.3} [V]({})", Self::adc_to_bias(pb_dac.dac_ch15), pb_dac.code_ch15);
        println!("Ch16 DAC:           {:.3} [V]({})", Self::adc_to_bias(pb_dac.dac_ch16), pb_dac.code_ch16);
    }
    fn adc_to_voltage(adc: u16) -> f32 {
        let voltage = PB_DAC_REF_VOLTAGE * (adc as f32) / 2f32.powf(12.0);

        voltage
    }
    fn adc_to_bias(adc: u16) -> f32 {
        let bias_voltage = Self::adc_to_voltage(adc) * 22.3;

        bias_voltage
    }
}