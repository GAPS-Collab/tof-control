use crate::constant::*;
use crate::helper::tcpc_type::{TCPCTemp, TCPCError};
use crate::device::tmp1075;
use crate::i2c_bus_lock::with_i2c_bus_lock;

impl TCPCTemp {
    pub fn new() -> Self {
        match Self::read_temp() {
            Ok(tcpc_temp) => {
                tcpc_temp
            }
            Err(_) => {
                Self {
                    tcpc_temp: f32::MAX,
                }
            }
        }
    }
    pub fn read_temp() -> Result<TCPCTemp, TCPCError> {
        with_i2c_bus_lock(|| {
            let tcpc_tmp1075 = tmp1075::TMP1075::new(1, TCPC_TMP1075_ADDRESS);
            tcpc_tmp1075.config()?;
            let tcpc_temp = tcpc_tmp1075.read()?;

            Ok(
                TCPCTemp {
                    tcpc_temp,
                }
            )
        })
    }
}