use crate::constant::*;
use crate::helper::rb_type::{RBDac, RBError};
use crate::device::{ad5675, pca9548a};
use crate::i2c_bus_lock::with_i2c_bus_lock;

impl RBDac {
    pub fn new() -> Self {
        match Self::read_dac() {
            Ok(rb_dac) => {
                rb_dac
            }
            Err(_) => {
                Self {
                    in_neg: f32::MAX,
                    in_pos: f32::MAX,
                    drs_rofs: f32::MAX,
                    v_cm: f32::MAX,
                    drs_bias: f32::MAX,
                }
            }
        }
    }
    pub fn read_dac() -> Result<RBDac, RBError> {
        with_i2c_bus_lock(|| {
            let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
            i2c_mux.select(RB_AD5675_CHANNEL)?;
            let ad5675 = ad5675::AD5675::new(I2C_BUS, RB_AD5675_ADDRESS);

            let in_neg: f32 = ad5675.read_dac_voltage(0)?;
            let in_pos: f32 = ad5675.read_dac_voltage(1)?;
            let drs_rofs: f32 = ad5675.read_dac_voltage(2)?;
            let v_cm: f32 = ad5675.read_dac_voltage(3)?;
            let drs_bias: f32 = ad5675.read_dac_voltage(4)?;

            i2c_mux.reset()?;

            Ok(
                RBDac { in_neg, in_pos, drs_rofs, v_cm, drs_bias }
            )
        })
    }

}

pub fn set_dac() -> Result<(), RBError> {
    with_i2c_bus_lock(|| {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
        i2c_mux.select(RB_AD5675_CHANNEL)?;
        let ad5675 = ad5675::AD5675::new(I2C_BUS, RB_AD5675_ADDRESS);

        /*	DAC settings
            Next few lines will configure the DAC outputs for DRS4/analog front end
            The AD5675 is a 16-bit DAC with range 0 to 2.048 V
            Decimal step size is: 0.00003125 V / integer

            DAC Channels:
                0x0: Vout -
                0x1: Vout +
                0X2: ROFS
                0X3: THS4509 Common Voltage
                0X4: DRS BIAS
        */

        // DRS4 analog input offset/bias: IN+_OFS, IN-_OFS
        // in_neg
        ad5675.write_dac(0, RB_AD5675_DAC0)?;
        // in_pos
        ad5675.write_dac(1, RB_AD5675_DAC1)?;
        // offset
        // DRS ROFS 1V, 1.6V max
        // ad5675.write_dac(2, 35200);
        ad5675.write_dac(2, RB_AD5675_DAC2)?;
        // THS4509 common mode voltage: V_CM
        // For +3.5 V and -1.5 V split supply, half range is 1 V
        ad5675.write_dac(3, RB_AD5675_DAC3)?;
        // DRS BIAS 0.7V
        ad5675.write_dac(4, RB_AD5675_DAC4)?;

        i2c_mux.reset()?;

        Ok(())
    })
}

pub fn read_dac() -> Result<[u16; 5], RBError> {
    with_i2c_bus_lock(|| {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
        i2c_mux.select(RB_AD5675_CHANNEL)?;
        let ad5675 = ad5675::AD5675::new(I2C_BUS, RB_AD5675_ADDRESS);

        let mut dac_values: [u16; 5] = [u16::max_value(); 5];
        for i in 0..=4 {
            let dac = ad5675.read_dac(i)?;
            dac_values[i as usize] = dac;
        }

        i2c_mux.reset()?;

        Ok(dac_values)
    })
}

pub fn read_single_dac(channel: u8) -> Result<u16, RBError> {
    with_i2c_bus_lock(|| {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
        i2c_mux.select(RB_AD5675_CHANNEL)?;
        let ad5675 = ad5675::AD5675::new(I2C_BUS, RB_AD5675_ADDRESS);

        let dac = ad5675.read_dac(channel)?;

        i2c_mux.reset()?;

        Ok(dac)
    })
}

pub fn zero_dac() -> Result<(), RBError> {
    with_i2c_bus_lock(|| {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
        i2c_mux.select(RB_AD5675_CHANNEL)?;
        let ad5675 = ad5675::AD5675::new(I2C_BUS, RB_AD5675_ADDRESS);

        for i in 0..=4 {
            ad5675.write_dac(i, 0)?;
        }

        i2c_mux.reset()?;

        Ok(())
    })
}

pub fn set_dac_500() -> Result<(), RBError> {
    with_i2c_bus_lock(|| {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
        i2c_mux.select(RB_AD5675_CHANNEL)?;
        let ad5675 = ad5675::AD5675::new(I2C_BUS, RB_AD5675_ADDRESS);

        ad5675.write_dac(2, 49600)?;

        i2c_mux.reset()?;

        Ok(())
    })
}

pub fn set_single_dac(channel: u8, value: u16) -> Result<(), RBError> {
    with_i2c_bus_lock(|| {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
        i2c_mux.select(RB_AD5675_CHANNEL)?;
        let ad5675 = ad5675::AD5675::new(I2C_BUS, RB_AD5675_ADDRESS);

        ad5675.write_dac(channel, value)?;

        i2c_mux.reset()?;

        Ok(())
    })
}

pub fn set_input_range(lower_bound: f32) -> Result<(), RBError> {
    with_i2c_bus_lock(|| {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
        i2c_mux.select(RB_AD5675_CHANNEL)?;
        let ad5675 = ad5675::AD5675::new(I2C_BUS, RB_AD5675_ADDRESS);

        let rofs: u16 = ((1.05 - (lower_bound / 1000.0)) * 32000.0) as u16;
        println!("Setting ROFS to: {}", rofs);

        ad5675.write_dac(2, rofs)?;
        i2c_mux.reset()?;

        Ok(())
    })
}

pub fn dac_noi_mode() -> Result<(), RBError> {
    with_i2c_bus_lock(|| {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
        i2c_mux.select(RB_AD5675_CHANNEL)?;
        let ad5675 = ad5675::AD5675::new(I2C_BUS, RB_AD5675_ADDRESS);

        ad5675.write_dac(0, RB_AD5675_DAC0)?;
        ad5675.write_dac(1, RB_AD5675_DAC1)?;
        ad5675.write_dac(2, RB_AD5675_DAC2)?;
        ad5675.write_dac(3, RB_AD5675_DAC3)?;
        ad5675.write_dac(4, RB_AD5675_DAC4)?;

        i2c_mux.reset()?;

        Ok(())
    })
}

pub fn dac_vcal_mode() -> Result<(), RBError> {
    with_i2c_bus_lock(|| {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
        i2c_mux.select(RB_AD5675_CHANNEL)?;
        let ad5675 = ad5675::AD5675::new(I2C_BUS, RB_AD5675_ADDRESS);

        ad5675.write_dac(0, RB_AD5675_DAC0)?;
        ad5675.write_dac(1, RB_AD5675_DAC1_VCAL)?;
        ad5675.write_dac(2, RB_AD5675_DAC2)?;
        ad5675.write_dac(3, RB_AD5675_DAC3)?;
        ad5675.write_dac(4, RB_AD5675_DAC4)?;

        i2c_mux.reset()?;

        Ok(())
    })
}

pub fn dac_tcal_mode() -> Result<(), RBError> {
    with_i2c_bus_lock(|| {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
        i2c_mux.select(RB_AD5675_CHANNEL)?;
        let ad5675 = ad5675::AD5675::new(I2C_BUS, RB_AD5675_ADDRESS);

        ad5675.write_dac(0, RB_AD5675_DAC0)?;
        ad5675.write_dac(1, RB_AD5675_DAC1)?;
        ad5675.write_dac(2, RB_AD5675_DAC2)?;
        ad5675.write_dac(3, RB_AD5675_DAC3)?;
        ad5675.write_dac(4, RB_AD5675_DAC4)?;

        i2c_mux.reset()?;

        Ok(())
    })
}

pub fn dac_sma_mode() -> Result<(), RBError> {
    with_i2c_bus_lock(|| {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
        i2c_mux.select(RB_AD5675_CHANNEL)?;
        let ad5675 = ad5675::AD5675::new(I2C_BUS, RB_AD5675_ADDRESS);

        ad5675.write_dac(0, RB_AD5675_DAC0)?;
        ad5675.write_dac(1, RB_AD5675_DAC1)?;
        ad5675.write_dac(2, RB_AD5675_DAC2)?;
        ad5675.write_dac(3, RB_AD5675_DAC3)?;
        ad5675.write_dac(4, RB_AD5675_DAC4)?;

        i2c_mux.reset()?;

        Ok(())
    })
}