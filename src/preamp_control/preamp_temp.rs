use crate::constant::*;
use crate::helper::preamp_type::{PreampTemp, PreampTempError};
use crate::device::{max11615, max11617, pca9548a};

impl PreampTemp {
    pub fn new() -> Self {
        let mut preamp_temp_ary: [[f32; 16]; 3] = Default::default();
        let mut preamp_temps: [f32; 16] = Default::default();
        
        let preamp_channels = [
            PREAMP_TEMP_1_CHNANNEL, PREAMP_TEMP_2_CHNANNEL, PREAMP_TEMP_3_CHNANNEL, PREAMP_TEMP_4_CHNANNEL,
            PREAMP_TEMP_5_CHNANNEL, PREAMP_TEMP_6_CHNANNEL, PREAMP_TEMP_7_CHNANNEL, PREAMP_TEMP_8_CHNANNEL,
            PREAMP_TEMP_9_CHNANNEL, PREAMP_TEMP_10_CHNANNEL, PREAMP_TEMP_11_CHNANNEL, PREAMP_TEMP_12_CHNANNEL,
            PREAMP_TEMP_13_CHNANNEL, PREAMP_TEMP_14_CHNANNEL, PREAMP_TEMP_15_CHNANNEL, PREAMP_TEMP_16_CHNANNEL,
        ];

        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, PB_PCA9548A_ADDRESS);

        for i in 0..3 {
            if i2c_mux.select(PB_ADC_1_CHANNEL).is_err() {
                preamp_temps = [f32::MAX; 16];
                return Self { preamp_temps }
            }
            let max11615 = max11615::MAX11615::new(I2C_BUS, PB_MAX11615_ADDRESS);
            if max11615.setup().is_err() {
                preamp_temps = [f32::MAX; 16];
                return Self { preamp_temps }
            }
            let max11617 = max11617::MAX11617::new(I2C_BUS, PB_MAX11617_ADDRESS);
            if max11617.setup().is_err() {
                preamp_temps = [f32::MAX; 16];
                return Self { preamp_temps }
            }

            for i in 0..=7 {
                let preamp_temp: f32;
                if (0..=3).contains(&i) {
                    match max11615.read(preamp_channels[i]) {
                        Ok(t) => preamp_temp = Self::voltage_to_temp(t),
                        Err(_) => preamp_temp = f32::MAX,
                    }
                    preamp_temps[i] = preamp_temp;
                } else {
                    match max11617.read(preamp_channels[i]) {
                        Ok(t) => preamp_temp = Self::voltage_to_temp(t),
                        Err(_) => preamp_temp = f32::MAX,
                    }
                    preamp_temps[i] = preamp_temp;
                }
            }

            if i2c_mux.select(PB_ADC_2_CHANNEL).is_err() {
                preamp_temps = [f32::MAX; 16];
                return Self { preamp_temps }
            }
            if max11615.setup().is_err() {
                preamp_temps = [f32::MAX; 16];
                return Self { preamp_temps }
            }
            if max11617.setup().is_err() {
                preamp_temps = [f32::MAX; 16];
                return Self { preamp_temps }
            }

            for i in 8..=15 {
                let preamp_temp: f32;
                if (8..=11).contains(&i) {
                    match max11615.read(preamp_channels[i]) {
                        Ok(t) => preamp_temp = Self::voltage_to_temp(t),
                        Err(_) => preamp_temp = f32::MAX,
                    }
                    preamp_temps[i] = preamp_temp;
                } else {
                    match max11617.read(preamp_channels[i]) {
                        Ok(t) => preamp_temp = Self::voltage_to_temp(t),
                        Err(_) => preamp_temp = f32::MAX,
                    }
                    preamp_temps[i] = preamp_temp;
                }
            }

            preamp_temp_ary[..][i] = preamp_temps;
        }

        if i2c_mux.reset().is_err() {
            preamp_temps = [f32::MAX; 16];
            return Self { preamp_temps }
        }

        for i in 0..=15 {
            let delta_0 = (preamp_temp_ary[0][i] - preamp_temp_ary[1][i]).abs();
            let delta_1 = (preamp_temp_ary[1][i] - preamp_temp_ary[2][i]).abs();
            let delta_2 = (preamp_temp_ary[2][i] - preamp_temp_ary[0][i]).abs();
            let delta_avg = (delta_0 + delta_1 + delta_2) / 3.0;

            if delta_avg > 1.0 {
                preamp_temps[i] = f32::MAX;
            } else {
                preamp_temps[i] = (preamp_temp_ary[0][i] + preamp_temp_ary[1][i] + preamp_temp_ary[2][i]) / 3.0;
            }

            if preamp_temps[i] < -40.0 || preamp_temps[i] > 150.0 {
                preamp_temps[i] = f32::MAX;
            }
        }

        Self {
            preamp_temps,
        }
    }
    pub fn read_temp() -> Result<[f32; 16], PreampTempError> {
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
            preamp_temps
        )


    }
    fn voltage_to_temp(voltage: f32) -> f32 {
        let mut temperature = (voltage - 0.5) * 100.0;
        if -60.0 > temperature || temperature > 150.0 {
            temperature = 500.0;
        }

        temperature
    }
}
