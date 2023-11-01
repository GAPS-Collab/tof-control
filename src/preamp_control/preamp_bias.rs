use crate::constant::*;
use crate::helper::preamp_type::{PreampReadBias, PreampTemp, PreampBiasError};
use crate::device::{max11615, max11617, max5825, pca9548a};

impl PreampReadBias {
    pub fn new() -> Self {
        match Self::read_bias() {
            Ok(read_biases) => {
                read_biases
            }
            Err(_) => {
                Self {
                    read_biases: [f32::MAX; 16]
                }
            }
        }
    }
    pub fn read_bias() -> Result<PreampReadBias, PreampBiasError> {
        let mut read_biases: [f32; 16] = Default::default();

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
                read_biases[i] = Self::convert_bias_voltage(preamp_bias_raw);
            } else {
                let preamp_bias_raw = max11617.read(preamp_channels[i])?;
                read_biases[i] = Self::convert_bias_voltage(preamp_bias_raw);
            }
        }

        i2c_mux.select(PB_ADC_2_CHANNEL)?;
        max11615.setup()?;
        max11617.setup()?;

        for i in 8..=15 {
            if (8..=11).contains(&i) {
                let preamp_bias_raw = max11615.read(preamp_channels[i])?;
                read_biases[i] = Self::convert_bias_voltage(preamp_bias_raw);
            } else {
                let preamp_bias_raw = max11617.read(preamp_channels[i])?;
                read_biases[i] = Self::convert_bias_voltage(preamp_bias_raw);
            }
        }

        i2c_mux.reset()?;

        Ok(
            PreampReadBias {
                read_biases,
            }
        )
    }
    fn convert_bias_voltage(voltage: f32) -> f32 {
        // let bias_voltage = voltage * 22.27659574468085;
        let bias_voltage = voltage * (10f32.powi(6i32) + 47f32*10f32.powi(3i32))/(47f32*10f32.powi(3i32));

        bias_voltage
    }
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

pub fn set_bias() -> Result<(), PreampBiasError> {

    // let mut bias_voltage = Default::default();

    let preamp_bias_channels = [
        PREAMP_DAC_1_CHANNEL, PREAMP_DAC_2_CHANNEL, PREAMP_DAC_3_CHANNEL, PREAMP_DAC_4_CHANNEL,
        PREAMP_DAC_5_CHANNEL, PREAMP_DAC_6_CHANNEL, PREAMP_DAC_7_CHANNEL, PREAMP_DAC_8_CHANNEL,
        PREAMP_DAC_9_CHANNEL, PREAMP_DAC_10_CHANNEL, PREAMP_DAC_11_CHANNEL, PREAMP_DAC_12_CHANNEL,
        PREAMP_DAC_13_CHANNEL, PREAMP_DAC_14_CHANNEL, PREAMP_DAC_15_CHANNEL, PREAMP_DAC_16_CHANNEL,
    ];

    let mut bias_voltages: [u16; 16] = Default::default();

    for i in 0..=15 {
        bias_voltages[i] = bias_to_adc(sipm_temp_comp(i)?);
    }

    let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, PB_PCA9548A_ADDRESS);
    
    i2c_mux.select(PB_DAC_1_CHANNEL)?;
    let pb_dac_1 = max5825::MAX5825::new(I2C_BUS, PB_MAX5825_ADDRESS);
    for i in 0..=7 {
        pb_dac_1.coden_loadn(preamp_bias_channels[i], bias_voltages[i])?;
    }

    i2c_mux.select(PB_DAC_2_CHANNEL)?;
    let pb_dac_2 = max5825::MAX5825::new(I2C_BUS, PB_MAX5825_ADDRESS);
    for i in 8..=15 {
        pb_dac_2.coden_loadn(preamp_bias_channels[i], bias_voltages[i])?;
    }

    i2c_mux.reset()?;

    Ok(())
}

pub fn sipm_temp_comp(ch: usize) -> Result<f32, PreampBiasError> {
    let mut bias_voltage: f32 = Default::default();
    
    let preamp_temp = PreampTemp::read_signle_temp(ch)?;
    if preamp_temp == f32::MAX {
        bias_voltage = 0.0
    } else {
        if (0..=15).contains(&ch) {
            bias_voltage = PREAMP_DEFAULT_BIAS + (preamp_temp - 20.0) * 0.054;
        } else {
            bias_voltage = 0.0;
        }
    }
    // if (0..=15).contains(&ch) {
    //     bias_voltage = PREAMP_DEFAULT_BIAS + (preamp_temp - 20.0) * 0.054;
    // } else {
    //     bias_voltage = 0.0;
    // }

    Ok(bias_voltage)
}

pub fn read_set_bias() -> Result<[f32; 16], PreampBiasError> {

    let preamp_bias_channels = [
        PREAMP_DAC_1_CHANNEL, PREAMP_DAC_2_CHANNEL, PREAMP_DAC_3_CHANNEL, PREAMP_DAC_4_CHANNEL,
        PREAMP_DAC_5_CHANNEL, PREAMP_DAC_6_CHANNEL, PREAMP_DAC_7_CHANNEL, PREAMP_DAC_8_CHANNEL,
        PREAMP_DAC_9_CHANNEL, PREAMP_DAC_10_CHANNEL, PREAMP_DAC_11_CHANNEL, PREAMP_DAC_12_CHANNEL,
        PREAMP_DAC_13_CHANNEL, PREAMP_DAC_14_CHANNEL, PREAMP_DAC_15_CHANNEL, PREAMP_DAC_16_CHANNEL,
    ];

    let mut set_bias_voltages: [f32; 16] = Default::default();

    let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, PB_PCA9548A_ADDRESS);

    i2c_mux.select(PB_DAC_1_CHANNEL)?;
    let pb_dac_1 = max5825::MAX5825::new(I2C_BUS, PB_MAX5825_ADDRESS);
    for i in 0..=7 {
        set_bias_voltages[i] = adc_to_bias(pb_dac_1.read_dacn(preamp_bias_channels[i])?);
    }

    i2c_mux.select(PB_DAC_2_CHANNEL)?;
    let pb_dac_2 = max5825::MAX5825::new(I2C_BUS, PB_MAX5825_ADDRESS);
    for i in 8..=15 {
        set_bias_voltages[i] = adc_to_bias(pb_dac_2.read_dacn(preamp_bias_channels[i])?);
    }

    i2c_mux.reset()?;

    Ok(set_bias_voltages)
}

// pub fn pb_temp_comp(bias: f32) -> Result<f32, PreampBiasError> {
//     let pb_temp = read_pds_temp()?;

//     let bias_set = bias - 0.2 + 0.005 * pb_temp + 4.0 * 10.0.powf(-5) * pb_temp * pb_temp;
// }

fn bias_to_adc(bias_voltage: f32) -> u16 {
    let adc = (bias_voltage / 22.3) / PB_DAC_REF_VOLTAGE * 2f32.powf(12.0);

    adc as u16
}

fn adc_to_bias(adc: u16) -> f32 {
    let bias_voltage = adc as f32 * PB_DAC_REF_VOLTAGE * 22.3 * 2f32.powf(-12.0);

    bias_voltage
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