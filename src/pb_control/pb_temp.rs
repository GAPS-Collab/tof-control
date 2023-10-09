use crate::constant::*;
use crate::helper::pb_type::{PBTemp, PBTempError};
use crate::device::{pca9548a, tmp1075};

impl PBTemp {
    pub fn new() -> Result<Self, PBTempError> {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, PB_PCA9548A_ADDRESS);

        i2c_mux.select(PB_TMP1075_CHANNEL)?;

        let pds_tmp1075 = tmp1075::TMP1075::new(I2C_BUS, PB_PDS_TMP1075_ADDRESS);
        pds_tmp1075.config()?;
        // let pds_temp = pds_tmp1075.read()?;
        let pds_temp: f32;
        match pds_tmp1075.read() {
            Ok(t) => {
                pds_temp = t;
            },
            Err(_) => {
                pds_temp = f32::MAX;
            }
        }

        let pas_tmp1075 = tmp1075::TMP1075::new(I2C_BUS, PB_PAS_TMP1075_ADDRESS);
        pas_tmp1075.config()?;
        // let pas_temp = pas_tmp1075.read()?;
        let pas_temp: f32;
        match pas_tmp1075.read() {
            Ok(t) => {
                pas_temp = t;
            },
            Err(_) => {
                pas_temp = f32::MAX;
            }
        }

        let nas_tmp1075 = tmp1075::TMP1075::new(I2C_BUS, PB_NAS_TMP1075_ADDRESS);
        nas_tmp1075.config()?;
        // let nas_temp = nas_tmp1075.read()?;
        let nas_temp: f32;
        match nas_tmp1075.read() {
            Ok(t) => {
                nas_temp = t;
            },
            Err(_) => {
                nas_temp = f32::MAX;
            }
        }

        let shv_tmp1075 = tmp1075::TMP1075::new(I2C_BUS, PB_SHV_TMP1075_ADDRESS);
        shv_tmp1075.config()?;
        // let shv_temp = shv_tmp1075.read()?;
        let shv_temp: f32;
        match shv_tmp1075.read() {
            Ok(t) => {
                shv_temp = t;
            },
            Err(_) => {
                shv_temp = f32::MAX;
            }
        }

        i2c_mux.reset()?;

        Ok(
            Self {
                pds_temp,
                pas_temp,
                nas_temp,
                shv_temp,
            }
        )
    }
}
