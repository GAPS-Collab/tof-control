use crate::constant::*;
use crate::device::max5815;

pub struct LTBdac {
    pub dac0: f32,
    pub dac1: f32,
    pub dac2: f32,
}

impl LTBdac {
    pub fn new() -> Self {
        let ltb_dac = max5815::MAX5815::new(I2C_BUS, LTB_MAX5815_ADDRESS);
        let dac0 = Self::adc_to_mv(ltb_dac.read_dacn(0).expect("cannot read MAX5815"));
        let dac1 = Self::adc_to_mv(ltb_dac.read_dacn(1).expect("cannot read MAX5815"));
        let dac2 = Self::adc_to_mv(ltb_dac.read_dacn(2).expect("cannot read MAX5815"));

        Self {
            dac0,
            dac1,
            dac2,
        }
    }
    pub fn print_ltb_dac() {
        let ltb_dac = LTBdac::new();
        println!("Threshold 0:            {:.3} [mV]", ltb_dac.dac0);
        println!("Threshold 1:            {:.3} [mV]", ltb_dac.dac1);
        println!("Threshold 2:            {:.3} [mV]", ltb_dac.dac2);
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