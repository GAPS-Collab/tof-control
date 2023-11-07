use crate::constant::*;
use crate::helper::preamp_type::{PreampTemp, PreampTempError};
use crate::device::{max11615, max11617, pca9548a};

impl PreampTemp {
    pub fn new() -> Self {
        match Self::read_temp() {
            Ok(preamp_temps) => {
                preamp_temps
            }
            Err(_) => {
                Self {
                    preamp_temps: [f32::MAX; 16]
                }
            }
        }
    }

    pub fn read_temp() -> Result<PreampTemp, PreampTempError> {
        let mut preamp_temps: [f32; 16] = Default::default();

        let preamp_channels = [
            PREAMP_TEMP_1_CHNANNEL, PREAMP_TEMP_2_CHNANNEL, PREAMP_TEMP_3_CHNANNEL, PREAMP_TEMP_4_CHNANNEL,
            PREAMP_TEMP_5_CHNANNEL, PREAMP_TEMP_6_CHNANNEL, PREAMP_TEMP_7_CHNANNEL, PREAMP_TEMP_8_CHNANNEL,
            PREAMP_TEMP_9_CHNANNEL, PREAMP_TEMP_10_CHNANNEL, PREAMP_TEMP_11_CHNANNEL, PREAMP_TEMP_12_CHNANNEL,
            PREAMP_TEMP_13_CHNANNEL, PREAMP_TEMP_14_CHNANNEL, PREAMP_TEMP_15_CHNANNEL, PREAMP_TEMP_16_CHNANNEL,
        ];
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, PB_PCA9548A_ADDRESS);
        i2c_mux.select(PB_ADC_1_CHANNEL)?;
        let max11615 = max11615::MAX11615::new(I2C_BUS, PB_MAX11615_ADDRESS);
        max11615.setup()?;
        let max11617 = max11617::MAX11617::new(I2C_BUS, PB_MAX11617_ADDRESS);
        max11617.setup()?;

        for i in 0..=7 {
            if (0..=3).contains(&i) {
                let preamp_temp_raw = max11615.read(preamp_channels[i])?;
                preamp_temps[i] = Self::voltage_to_temp(preamp_temp_raw);
            } else {
                let preamp_temp_raw = max11617.read(preamp_channels[i])?;
                preamp_temps[i] = Self::voltage_to_temp(preamp_temp_raw);
            }
        }

        i2c_mux.select(PB_ADC_2_CHANNEL)?;
        max11615.setup()?;
        max11617.setup()?;

        for i in 8..=15 {
            if (8..=11).contains(&i) {
                let preamp_temp_raw = max11615.read(preamp_channels[i])?;
                preamp_temps[i] = Self::voltage_to_temp(preamp_temp_raw);
            } else {
                let preamp_temp_raw = max11617.read(preamp_channels[i])?;
                preamp_temps[i] = Self::voltage_to_temp(preamp_temp_raw);
            }
        }

        i2c_mux.reset()?;

        Ok(
            PreampTemp {
                preamp_temps
            }
        )


    }
    pub fn read_signle_temp(ch: usize) -> Result<f32, PreampTempError> {
        let preamp_temp;

        let preamp_channels = [
            PREAMP_TEMP_1_CHNANNEL, PREAMP_TEMP_2_CHNANNEL, PREAMP_TEMP_3_CHNANNEL, PREAMP_TEMP_4_CHNANNEL,
            PREAMP_TEMP_5_CHNANNEL, PREAMP_TEMP_6_CHNANNEL, PREAMP_TEMP_7_CHNANNEL, PREAMP_TEMP_8_CHNANNEL,
            PREAMP_TEMP_9_CHNANNEL, PREAMP_TEMP_10_CHNANNEL, PREAMP_TEMP_11_CHNANNEL, PREAMP_TEMP_12_CHNANNEL,
            PREAMP_TEMP_13_CHNANNEL, PREAMP_TEMP_14_CHNANNEL, PREAMP_TEMP_15_CHNANNEL, PREAMP_TEMP_16_CHNANNEL,
        ];

        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, PB_PCA9548A_ADDRESS);
        let max11615 = max11615::MAX11615::new(I2C_BUS, PB_MAX11615_ADDRESS);
        let max11617 = max11617::MAX11617::new(I2C_BUS, PB_MAX11617_ADDRESS);
        
        match ch {
            0..=3 => {
                i2c_mux.select(PB_ADC_1_CHANNEL)?;
                max11615.setup()?;
                let preamp_temp_raw = max11615.read(preamp_channels[ch])?;
                preamp_temp = Self::voltage_to_temp(preamp_temp_raw);
            }
            4..=7 => {
                i2c_mux.select(PB_ADC_1_CHANNEL)?;
                max11617.setup()?;
                let preamp_temp_raw = max11617.read(preamp_channels[ch])?;
                preamp_temp = Self::voltage_to_temp(preamp_temp_raw);
            }
            8..=11 => {
                i2c_mux.select(PB_ADC_2_CHANNEL)?;
                max11615.setup()?;
                let preamp_temp_raw = max11615.read(preamp_channels[ch])?;
                preamp_temp = Self::voltage_to_temp(preamp_temp_raw);
            }
            12..=15 => {
                i2c_mux.select(PB_ADC_2_CHANNEL)?;
                max11617.setup()?;
                let preamp_temp_raw = max11617.read(preamp_channels[ch])?;
                preamp_temp = Self::voltage_to_temp(preamp_temp_raw);
            }
            _ => {
                preamp_temp = f32::MAX;
            }
        }

        i2c_mux.reset()?;

        Ok(preamp_temp)
    }
    
    fn voltage_to_temp(voltage: f32) -> f32 {
        let mut temperature = (voltage - 0.5) * 100.0;
        if -60.0 > temperature || temperature > 150.0 {
            temperature = f32::MAX;
        }

        temperature
    }
}
