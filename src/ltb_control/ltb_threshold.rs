use crate::constant::*;
use crate::helper::ltb_type::{LTBThreshold, LTBThresholdError};
use crate::device::max5815;

impl LTBThreshold {
    pub fn new() -> Self {
        let ltb_dac: max5815::MAX5815 = max5815::MAX5815::new(I2C_BUS, LTB_MAX5815_ADDRESS);

        let mut thresholds: [f32; 3] = Default::default();
        for i in 0..=2 {
            match ltb_dac.read_dacn(i) {
                Ok(v) => thresholds[i as usize] = Self::adc_to_mv(v),
                Err(_) => thresholds[i as usize] = f32::MAX,
             }
        }

        Self {
            thresholds,
        }

    }
    pub fn read_threshold(channel: u8) -> Result<f32, LTBThresholdError> {
        let ltb_dac = max5815::MAX5815::new(I2C_BUS, LTB_MAX5815_ADDRESS);
        let threshold_raw = ltb_dac.read_dacn(channel)?;
        let threshold = Self::adc_to_mv(threshold_raw);

        Ok(threshold)
    }
    pub fn read_thresholds() -> Result<[f32; 3], LTBThresholdError> {
        let ltb_dac: max5815::MAX5815 = max5815::MAX5815::new(I2C_BUS, LTB_MAX5815_ADDRESS);
        let mut thresholds: [f32; 3] = Default::default();
        for i in 0..=2 {
            let threshold_raw = ltb_dac.read_dacn(i)?;
            thresholds[i as usize] = Self::adc_to_mv(threshold_raw);
        }

        Ok(thresholds)
    }
    fn adc_to_mv(adc: u16) -> f32 {
        let voltage = LTB_DAC_REF_VOLTAGE * (adc as f32) / 2f32.powf(12.0);

        voltage * 1000.0
    }
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

pub fn set_threshold(channel: u8, threshold: f32) -> Result<(), LTBThresholdError> {

    if !(0..=2).contains(&channel) {
        return Err(LTBThresholdError::SetThreshold())
    } 

    let ltb_dac = max5815::MAX5815::new(I2C_BUS, LTB_MAX5815_ADDRESS);
    ltb_dac.configure()?;

    ltb_dac.coden_loadn(channel, mv_to_adc(threshold))?;

    Ok(())
}

fn mv_to_adc(mv: f32) -> u16 {
let adc = (mv / 1000.0) / LTB_DAC_REF_VOLTAGE * 2f32.powf(12.0);

adc as u16
}

pub fn reset_threshold() -> Result<(), LTBThresholdError> {
    let ltb_dac = max5815::MAX5815::new(I2C_BUS, LTB_MAX5815_ADDRESS);
    ltb_dac.reset_dac()?;

    Ok(())
}