use crate::constant::*;
use crate::helper::cpc_type::{CPCVcp, CPCError};
use crate::device::ina219;

impl CPCVcp {
    pub fn new() -> Self {
        match Self::read_vcp() {
            Ok(cpc_vcp) => {
                cpc_vcp
            }
            Err(_) => {
                Self {
                    cpc_vcp: [f32::MAX; 3],
                }
            }
        }
    }
    pub fn read_vcp() -> Result<CPCVcp, CPCError> {
        let cpc_ina219 = ina219::INA219::new(
            CPC_I2C_BUS,
            CPC_INA219_ADDRESS,
            CPC_INA219_RSHUNT,
            CPC_INA219_MEC,
        );
        cpc_ina219.configure()?;
        let cpc_vcp = cpc_ina219.read()?;

        Ok(
            CPCVcp{
                cpc_vcp,
            }
        )
    }
}