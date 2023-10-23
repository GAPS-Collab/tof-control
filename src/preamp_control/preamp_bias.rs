use crate::constant::*;
use crate::helper::preamp_type::{PreampBias, PreampBiasError};
use crate::device::{max11615, max11617, max5825, pca9548a};

impl PreampBias {
    pub fn new() -> Self {
        let mut preamp_biases: [f32; 16] = Default::default();

        let preamp_channels = [
            PREAMP_SEN_1_CHANNEL, PREAMP_SEN_2_CHANNEL, PREAMP_SEN_3_CHANNEL, PREAMP_SEN_4_CHANNEL,
            PREAMP_SEN_5_CHANNEL, PREAMP_SEN_6_CHANNEL, PREAMP_SEN_7_CHANNEL, PREAMP_SEN_8_CHANNEL,
            PREAMP_SEN_9_CHANNEL, PREAMP_SEN_10_CHANNEL, PREAMP_SEN_11_CHANNEL, PREAMP_SEN_12_CHANNEL,
            PREAMP_SEN_13_CHANNEL, PREAMP_SEN_14_CHANNEL, PREAMP_SEN_15_CHANNEL, PREAMP_SEN_16_CHANNEL,
        ];

        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, PB_PCA9548A_ADDRESS);
        if i2c_mux.select(PB_ADC_1_CHANNEL).is_err() {
            preamp_biases = [f32::MAX; 16];
            return Self { preamp_biases }
        }
        let max11615 = max11615::MAX11615::new(I2C_BUS, PB_MAX11615_ADDRESS);
        if max11615.setup().is_err() {
            preamp_biases = [f32::MAX; 16];
            return Self { preamp_biases }
        }
        let max11617 = max11617::MAX11617::new(I2C_BUS, PB_MAX11617_ADDRESS);
        if max11617.setup().is_err() {
            preamp_biases = [f32::MAX; 16];
            return Self { preamp_biases }
        }

        for i in 0..=7 {
            if (0..=3).contains(&i) {
                match max11615.read(preamp_channels[i]) {
                    Ok(t) => preamp_biases[i] = Self::convert_bias_voltage(t),
                    Err(_) => preamp_biases[i] = f32::MAX,
                }
            } else {
                match max11617.read(preamp_channels[i]) {
                    Ok(t) => preamp_biases[i] = Self::convert_bias_voltage(t),
                    Err(_) => preamp_biases[i] = f32::MAX,
                }
            }
        }

        if i2c_mux.select(PB_ADC_2_CHANNEL).is_err() {
            preamp_biases = [f32::MAX; 16];
            return Self { preamp_biases }
        }
        if max11615.setup().is_err() {
            preamp_biases = [f32::MAX; 16];
            return Self { preamp_biases }
        }
        if max11617.setup().is_err() {
            preamp_biases = [f32::MAX; 16];
            return Self { preamp_biases }
        }

        for i in 8..=15 {
            if (8..=11).contains(&i) {
                match max11615.read(preamp_channels[i]) {
                    Ok(t) => preamp_biases[i] = Self::convert_bias_voltage(t),
                    Err(_) => preamp_biases[i] = f32::MAX,
                }
            } else {
                match max11617.read(preamp_channels[i]) {
                    Ok(t) => preamp_biases[i] = Self::convert_bias_voltage(t),
                    Err(_) => preamp_biases[i] = f32::MAX,
                }
            }
        }

        if i2c_mux.reset().is_err() {
            preamp_biases = [f32::MAX; 16];
            return Self { preamp_biases }
        }

        for i in 0..=15 {
            if preamp_biases[i] < 0.0 || preamp_biases[i] > 67.0 {
                preamp_biases[i] = f32::MAX;
            }
        }

        Self {
            preamp_biases,
        }

    }
    pub fn read_bias() -> Result<[f32; 16], PreampBiasError> {
        let mut preamp_biases: [f32; 16] = Default::default();

        let preamp_channels = [
            PREAMP_SEN_1_CHANNEL, PREAMP_SEN_2_CHANNEL, PREAMP_SEN_3_CHANNEL, PREAMP_SEN_4_CHANNEL,
            PREAMP_SEN_5_CHANNEL, PREAMP_SEN_6_CHANNEL, PREAMP_SEN_7_CHANNEL, PREAMP_SEN_8_CHANNEL,
            PREAMP_SEN_9_CHANNEL, PREAMP_SEN_10_CHANNEL, PREAMP_SEN_11_CHANNEL, PREAMP_SEN_12_CHANNEL,
            PREAMP_SEN_13_CHANNEL, PREAMP_SEN_14_CHANNEL, PREAMP_SEN_15_CHANNEL, PREAMP_SEN_16_CHANNEL,
        ];
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, PB_PCA9548A_ADDRESS);
        i2c_mux.select(PB_ADC_1_CHANNEL)?;
        let max11615 = max11615::MAX11615::new(I2C_BUS, PB_MAX11615_ADDRESS);
        max11615.setup()?;
        let max11617 = max11617::MAX11617::new(I2C_BUS, PB_MAX11617_ADDRESS);
        max11617.setup()?;

        for i in 0..=7 {
            if (0..=3).contains(&i) {
                let preamp_bias_raw = max11615.read(preamp_channels[i])?;
                preamp_biases[i] = Self::convert_bias_voltage(preamp_bias_raw);
            } else {
                let preamp_bias_raw = max11617.read(preamp_channels[i])?;
                preamp_biases[i] = Self::convert_bias_voltage(preamp_bias_raw);
            }
        }

        i2c_mux.select(PB_ADC_2_CHANNEL)?;
        max11615.setup()?;
        max11617.setup()?;

        for i in 8..=15 {
            if (8..=11).contains(&i) {
                let preamp_bias_raw = max11615.read(preamp_channels[i])?;
                preamp_biases[i] = Self::convert_bias_voltage(preamp_bias_raw);
            } else {
                let preamp_bias_raw = max11617.read(preamp_channels[i])?;
                preamp_biases[i] = Self::convert_bias_voltage(preamp_bias_raw);
            }
        }

        i2c_mux.reset()?;

        Ok(
            preamp_biases,
        )
    }
    fn convert_bias_voltage(voltage: f32) -> f32 {
        22.27659574468085 * voltage
    }
}

pub struct PreampBiasSet {}

impl PreampBiasSet {
    // pub fn reset_bias() {
    //     let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, PB_PCA9548A_ADDRESS);

    //     i2c_mux
    //         .select(PB_DAC_1_CHANNEL)
    //         .expect("cannot access to PCA9548A");
    //     let pb_dac_1 = max5825::MAX5825::new(I2C_BUS, PB_MAX5825_ADDRESS);
    //     pb_dac_1.reset_dac().expect("cannot reset MAX5825(DAC1)");

    //     i2c_mux
    //         .select(PB_DAC_2_CHANNEL)
    //         .expect("cannot access to PCA9548A");
    //     let pb_dac_2 = max5825::MAX5825::new(I2C_BUS, PB_MAX5825_ADDRESS);

    //     pb_dac_2.reset_dac().expect("cannot reset MAX5825(DAC2)");
    // }

    // pub fn set_bias_manual(bias_voltage: f32) {
    //     let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, PB_PCA9548A_ADDRESS);

    //     i2c_mux
    //         .select(PB_DAC_1_CHANNEL)
    //         .expect("cannot access to PCA9548A");
    //     let pb_dac_1 = max5825::MAX5825::new(I2C_BUS, PB_MAX5825_ADDRESS);
    //     pb_dac_1
    //         .coden_loadn(PREAMP_DAC_1_CHANNEL, Self::bias_to_adc(bias_voltage))
    //         .expect("cannot set bias voltage 1 on MAX5825(DAC1)");
    //     pb_dac_1
    //         .coden_loadn(PREAMP_DAC_2_CHANNEL, Self::bias_to_adc(bias_voltage))
    //         .expect("cannot set bias voltage 2 on MAX5825(DAC1)");
    //     pb_dac_1
    //         .coden_loadn(PREAMP_DAC_3_CHANNEL, Self::bias_to_adc(bias_voltage))
    //         .expect("cannot set bias voltage 3 on MAX5825(DAC1)");
    //     pb_dac_1
    //         .coden_loadn(PREAMP_DAC_4_CHANNEL, Self::bias_to_adc(bias_voltage))
    //         .expect("cannot set bias voltage 4 on MAX5825(DAC1)");
    //     pb_dac_1
    //         .coden_loadn(PREAMP_DAC_5_CHANNEL, Self::bias_to_adc(bias_voltage))
    //         .expect("cannot set bias voltage 5 on MAX5825(DAC1)");
    //     pb_dac_1
    //         .coden_loadn(PREAMP_DAC_6_CHANNEL, Self::bias_to_adc(bias_voltage))
    //         .expect("cannot set bias voltage 6 on MAX5825(DAC1)");
    //     pb_dac_1
    //         .coden_loadn(PREAMP_DAC_7_CHANNEL, Self::bias_to_adc(bias_voltage))
    //         .expect("cannot set bias voltage 7 on MAX5825(DAC1)");
    //     pb_dac_1
    //         .coden_loadn(PREAMP_DAC_8_CHANNEL, Self::bias_to_adc(bias_voltage))
    //         .expect("cannot set bias voltage 8 on MAX5825(DAC1)");

    //     i2c_mux
    //         .select(PB_DAC_2_CHANNEL)
    //         .expect("cannot access to PCA9548A");
    //     let pb_dac_2 = max5825::MAX5825::new(I2C_BUS, PB_MAX5825_ADDRESS);
    //     pb_dac_2
    //         .coden_loadn(PREAMP_DAC_9_CHANNEL, Self::bias_to_adc(bias_voltage))
    //         .expect("cannot set bias voltage 9 on MAX5825(DAC2)");
    //     pb_dac_2
    //         .coden_loadn(PREAMP_DAC_10_CHANNEL, Self::bias_to_adc(bias_voltage))
    //         .expect("cannot set bias voltage 10 on MAX5825(DAC2)");
    //     pb_dac_2
    //         .coden_loadn(PREAMP_DAC_11_CHANNEL, Self::bias_to_adc(bias_voltage))
    //         .expect("cannot set bias voltage 11 on MAX5825(DAC2)");
    //     pb_dac_2
    //         .coden_loadn(PREAMP_DAC_12_CHANNEL, Self::bias_to_adc(bias_voltage))
    //         .expect("cannot set bias voltage 12 on MAX5825(DAC2)");
    //     pb_dac_2
    //         .coden_loadn(PREAMP_DAC_13_CHANNEL, Self::bias_to_adc(bias_voltage))
    //         .expect("cannot set bias voltage 13 on MAX5825(DAC2)");
    //     pb_dac_2
    //         .coden_loadn(PREAMP_DAC_14_CHANNEL, Self::bias_to_adc(bias_voltage))
    //         .expect("cannot set bias voltage 14 on MAX5825(DAC2)");
    //     pb_dac_2
    //         .coden_loadn(PREAMP_DAC_15_CHANNEL, Self::bias_to_adc(bias_voltage))
    //         .expect("cannot set bias voltage 15 on MAX5825(DAC2)");
    //     pb_dac_2
    //         .coden_loadn(PREAMP_DAC_16_CHANNEL, Self::bias_to_adc(bias_voltage))
    //         .expect("cannot set bias voltage 16 on MAX5825(DAC2)");

    //     i2c_mux.reset().expect("cannot reset PCA9548A");
    // }
}

pub fn set_default_bias() -> Result<(), PreampBiasError> {

    let bias_voltage = bias_to_adc(PREAMP_DEFAULT_BIAS);

    let preamp_bias_channels = [
        PREAMP_DAC_1_CHANNEL, PREAMP_DAC_2_CHANNEL, PREAMP_DAC_3_CHANNEL, PREAMP_DAC_4_CHANNEL,
        PREAMP_DAC_5_CHANNEL, PREAMP_DAC_6_CHANNEL, PREAMP_DAC_7_CHANNEL, PREAMP_DAC_8_CHANNEL,
        PREAMP_DAC_9_CHANNEL, PREAMP_DAC_10_CHANNEL, PREAMP_DAC_11_CHANNEL, PREAMP_DAC_12_CHANNEL,
        PREAMP_DAC_13_CHANNEL, PREAMP_DAC_14_CHANNEL, PREAMP_DAC_15_CHANNEL, PREAMP_DAC_16_CHANNEL,
    ];

    let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, PB_PCA9548A_ADDRESS);
    
    i2c_mux.select(PB_DAC_1_CHANNEL)?;
    let pb_dac_1 = max5825::MAX5825::new(I2C_BUS, PB_MAX5825_ADDRESS);
    for i in 0..=7 {
        pb_dac_1.coden_loadn(preamp_bias_channels[i], bias_voltage)?;
    }

    i2c_mux.select(PB_DAC_2_CHANNEL)?;
    let pb_dac_2 = max5825::MAX5825::new(I2C_BUS, PB_MAX5825_ADDRESS);
    for i in 8..=15 {
        pb_dac_2.coden_loadn(preamp_bias_channels[i], bias_voltage)?;
    }

    i2c_mux.reset()?;

    Ok(())

}

fn bias_to_adc(bias_voltage: f32) -> u16 {
    let adc = (bias_voltage / 22.3) / PB_DAC_REF_VOLTAGE * 2f32.powf(12.0);

    adc as u16
}

pub fn reset_bias() -> Result<(), PreampBiasError> {
    let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, PB_PCA9548A_ADDRESS);

    i2c_mux.select(PB_DAC_1_CHANNEL)?;
    let pb_dac = max5825::MAX5825::new(I2C_BUS, PB_MAX5825_ADDRESS);
    pb_dac.reset_dac()?;
    i2c_mux.select(PB_DAC_2_CHANNEL)?;
    pb_dac.reset_dac()?;

    i2c_mux.reset()?;

    Ok(())

}