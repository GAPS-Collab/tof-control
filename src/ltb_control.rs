use crate::constant::*;
use crate::device::{tmp112, max5815};

pub fn initialize() {
    let mut count = 0;
    while count < 5 {
        LTBtemperature::new();
        count += 1;
    }
    LTBdac::set_threshold();
}

pub struct LTBtemperature {
    ltb_temp: f32,
}

impl LTBtemperature {
    pub fn new() -> Self {
        let ltb_tmp112 = tmp112::TMP112::new(I2C_BUS, LTB_TMP112_ADDRESS);
        ltb_tmp112.config().expect("cannot configure TMP112");
        let ltb_temp = ltb_tmp112.read().expect("cannot read TMP112");
        Self {
            ltb_temp,
        }
    }
    pub fn print_ltb_temp() {
        let ltb_temp = LTBtemperature::new();
        println!("LTB Temperature:          {:.3}[°C]", ltb_temp.ltb_temp);
    }
}

pub struct LTBdac {
    device_id: u8,
    rev_id: u8,
    ref_mode: u8,
    code0: u16,
    code1: u16,
    code2: u16,
    code3: u16,
    dac0: u16,
    dac1: u16,
    dac2: u16,
    dac3: u16,
}

impl LTBdac {
    pub fn new() -> Self {
        let ltb_dac = max5815::MAX5815::new(I2C_BUS, LTB_MAX5815_ADDRESS);
        let (device_id, rev_id, ref_mode) = ltb_dac.read_device_info().expect("cannot read MAX5815");
        // let codea = ltb_dac.read_codea().expect("cannot read MAX5815");
        let code0 = ltb_dac.read_coden(0).expect("cannot read MAX5815");
        let code1 = ltb_dac.read_coden(1).expect("cannot read MAX5815");
        let code2 = ltb_dac.read_coden(2).expect("cannot read MAX5815");
        let code3 = ltb_dac.read_coden(3).expect("cannot read MAX5815");
        // let daca = ltb_dac.read_daca().expect("cannot read MAX5815");
        let dac0 = ltb_dac.read_dacn(0).expect("cannot read MAX5815");
        let dac1 = ltb_dac.read_dacn(1).expect("cannot read MAX5815");
        let dac2 = ltb_dac.read_dacn(2).expect("cannot read MAX5815");
        let dac3 = ltb_dac.read_dacn(3).expect("cannot read MAX5815");

        Self {
            device_id,
            rev_id,
            ref_mode,
            code0,
            code1,
            code2,
            code3,
            dac0,
            dac1,
            dac2,
            dac3,
        }
    }
    pub fn print_ltb_dac() {
        let ltb_dac = LTBdac::new();
        println!("Ch0 Threshold:            {:.3}[mV]", Self::adc_to_mv(ltb_dac.dac0));
        println!("Ch1 Threshold:            {:.3}[mV]", Self::adc_to_mv(ltb_dac.dac1));
        println!("Ch2 Threshold:            {:.3}[mV]", Self::adc_to_mv(ltb_dac.dac2));
    }
    fn adc_to_mv(adc: u16) -> f32 {
        let voltage = LTB_DAC_REF_VOLTAGE * (adc as f32) / 2f32.powf(12.0);

        voltage * 1000.0
    }
    fn mv_to_adc(mv: f32) -> u16 {
        let adc = (mv / 1000.0) / LTB_DAC_REF_VOLTAGE * 2f32.powf(12.0);

        adc as u16
    }
    pub fn set_threshold() {
        let ltb_dac = max5815::MAX5815::new(I2C_BUS, LTB_MAX5815_ADDRESS);
        ltb_dac.configure().expect("cannot configure MAX5815");
        ltb_dac.coden_loadn(0, Self::mv_to_adc(LTB_DAC_THRESHOLD_0)).expect("cannot set threshold 0 on MAX5815");
        ltb_dac.coden_loadn(1, Self::mv_to_adc(LTB_DAC_THRESHOLD_1)).expect("cannot set threshold 1 on MAX5815");
        ltb_dac.coden_loadn(2, Self::mv_to_adc(LTB_DAC_THRESHOLD_2)).expect("cannot set threshold 2 on MAX5815");
    }
    pub fn reset_threshold() {
        let ltb_dac = max5815::MAX5815::new(I2C_BUS, LTB_MAX5815_ADDRESS);
        ltb_dac.reset_dac().expect("cannot reset threshold on MAX5815");
    }
}