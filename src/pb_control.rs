pub mod pb_init;
pub mod pb_ltb_pwr;
pub mod pb_temp;
pub mod pb_vcp;

use crate::helper::pb_type::{PBLevel1, PBLevel1Error};
use crate::device::{pca9548a, tmp1075};
use crate::constant::*;
/// PB Level 1 Environmental Sensor Data
impl PBLevel1 {
    pub fn new() -> Self {
        match Self::get_data() {
            Ok(pb_level1) => {
                pb_level1
            }
            Err(_) => {
                Self {
                    pds_temp: f32::MAX,
                }
            }
        }
        
    }
    pub fn get_data() -> Result<PBLevel1, PBLevel1Error> {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, PB_PCA9548A_ADDRESS);

        let pds_tmp1075 = tmp1075::TMP1075::new(I2C_BUS, PB_PDS_TMP1075_ADDRESS);
        pds_tmp1075.config()?;
        let pds_temp = pds_tmp1075.read()?;

        i2c_mux.reset()?;

        Ok(
            PBLevel1 {
                pds_temp,
            }
        )
    }
}