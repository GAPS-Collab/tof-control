pub mod rb_clk;
pub mod rb_config;
pub mod rb_dac;
pub mod rb_gpioe;
pub mod rb_info;
pub mod rb_init;
pub mod rb_input;
pub mod rb_mag;
pub mod rb_mode;
pub mod rb_ph;
pub mod rb_temp;
pub mod rb_vcp;

use crate::helper::rb_type::{RBLevel1, RBLevel1Error};
use crate::device::{pca9548a, tmp112, ina226};
use crate::constant::*;
use crate::memory::*;
/// RB Level 1 Environmental Sensor Data
impl RBLevel1 {
    pub fn new() -> Self {
        match Self::get_data() {
            Ok(rb_level1) => {
                rb_level1
            }
            Err(_) => {
                Self {
                    zynq_temp: f32::MAX,
                    clk_temp: f32::MAX,
                    zynq_vc: [f32::MAX; 2],
                }
            }
        }
        
    }
    pub fn get_data() -> Result<RBLevel1, RBLevel1Error> {
        let zynq_temp_adc = read_control_reg(RB_TEMP)?;
        let zynq_temp = (((zynq_temp_adc & 4095) as f32 * 503.975) / 4096.0) - 273.15;

        let i2c_mux_1 = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_1);
        let i2c_mux_2 = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);

        i2c_mux_1.select(RB_ZYNQ_INA226_CHANNEL)?;
        let zynq_ina226 = ina226::INA226::new(I2C_BUS, RB_ZYNQ_INA226_ADDRESS, RB_ZYNQ_INA226_RSHUNT, RB_ZYNQ_INA226_MEC,);
        zynq_ina226.configure()?;
        let zynq_vcp = zynq_ina226.read()?;
        let zynq_vc = [zynq_vcp[0], zynq_vcp[1]];

        i2c_mux_2.select(RB_CLK_TMP112_CHANNEL)?;
        let clk_tmp112 = tmp112::TMP112::new(I2C_BUS, RB_CLK_TMP112_ADDRESS);
        clk_tmp112.config()?;
        let clk_temp = clk_tmp112.read()?;

        i2c_mux_1.reset()?;
        i2c_mux_2.reset()?;

        Ok(
            RBLevel1 {
                zynq_temp,
                clk_temp,
                zynq_vc,
            }
        )
    }
}