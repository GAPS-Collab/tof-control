use crate::constant::*;
use crate::helper::ltb_type::{LTBThreshold, LTBThresholdError};
use crate::device::max5815;

impl LTBThreshold {
    pub fn new() -> Result<Self, LTBThresholdError> {
        let ltb_dac = max5815::MAX5815::new(I2C_BUS, LTB_MAX5815_ADDRESS);
        let threshold_0_raw = ltb_dac.read_dacn(0)?;
        let threshold_1_raw = ltb_dac.read_dacn(1)?;
        let threshold_2_raw = ltb_dac.read_dacn(2)?;

        Ok(
            Self {
                threshold_0: Self::adc_to_mv(threshold_0_raw),
                threshold_1: Self::adc_to_mv(threshold_1_raw),
                threshold_2: Self::adc_to_mv(threshold_2_raw),
            }
        )
    }
    fn adc_to_mv(adc: u16) -> f32 {
        let voltage = LTB_DAC_REF_VOLTAGE * (adc as f32) / 2f32.powf(12.0);

        voltage * 1000.0
    }
    // fn mv_to_adc(mv: f32) -> u16 {
    //     let adc = (mv / 1000.0) / LTB_DAC_REF_VOLTAGE * 2f32.powf(12.0);

    //     adc as u16
    // }
    // pub fn set_threshold() {
    //     let ltb_dac = max5815::MAX5815::new(I2C_BUS, LTB_MAX5815_ADDRESS);
    //     ltb_dac.configure().expect("cannot configure MAX5815");
    //     ltb_dac
    //         .coden_loadn(0, Self::mv_to_adc(LTB_DAC_THRESHOLD_0))
    //         .expect("cannot set threshold 0 on MAX5815");
    //     ltb_dac
    //         .coden_loadn(1, Self::mv_to_adc(LTB_DAC_THRESHOLD_1))
    //         .expect("cannot set threshold 1 on MAX5815");
    //     ltb_dac
    //         .coden_loadn(2, Self::mv_to_adc(LTB_DAC_THRESHOLD_2))
    //         .expect("cannot set threshold 2 on MAX5815");
    // }
    // pub fn set_threshold_ucla() {
    //     let ltb_dac = max5815::MAX5815::new(I2C_BUS, LTB_MAX5815_ADDRESS);
    //     ltb_dac.configure().expect("cannot configure MAX5815");
    //     ltb_dac
    //         .coden_loadn(0, Self::mv_to_adc(50.0))
    //         .expect("cannot set threshold 0 on MAX5815");
    //     ltb_dac
    //         .coden_loadn(1, Self::mv_to_adc(50.0))
    //         .expect("cannot set threshold 1 on MAX5815");
    //     ltb_dac
    //         .coden_loadn(2, Self::mv_to_adc(150.0))
    //         .expect("cannot set threshold 2 on MAX5815");
    // }
    // pub fn reset_threshold() {
    //     let ltb_dac = max5815::MAX5815::new(I2C_BUS, LTB_MAX5815_ADDRESS);
    //     ltb_dac
    //         .reset_dac()
    //         .expect("cannot reset threshold on MAX5815");
    // }
}

pub fn set_default_threshold() -> Result<(), LTBThresholdError> {
    let ltb_dac = max5815::MAX5815::new(I2C_BUS, LTB_MAX5815_ADDRESS);
    ltb_dac.configure()?;

    let default_thresholds = [LTB_DAC_THRESHOLD_0, LTB_DAC_THRESHOLD_1, LTB_DAC_THRESHOLD_2];

    for (i, default_threshold) in default_thresholds.iter().enumerate() {
        ltb_dac.coden_loadn(i as u8, mv_to_adc(*default_threshold))?;
    };

    Ok(())
}

fn mv_to_adc(mv: f32) -> u16 {
let adc = (mv / 1000.0) / LTB_DAC_REF_VOLTAGE * 2f32.powf(12.0);

adc as u16
}