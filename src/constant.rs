#![allow(unused)]
// I2C Options
pub const I2C_BUS: u8 = 0;

// I2C Options for Readout Board
pub const RB_PCA9548A_ADDRESS_1: u16 = 0x75;
pub const RB_PCA9548A_ADDRESS_2: u16 = 0x77;

// PCA9548A_ADDRESS_1
pub const RB_DRS_TMP112_ADDRESS: u16 = 0x48;
pub const RB_DRS_TMP112_CHANNEL: u8 = 2;
// PCA9548A_ADDRESS_2
pub const RB_CLK_TMP112_ADDRESS: u16 = 0x4B;
pub const RB_CLK_TMP112_CHANNEL: u8 = 0;
pub const RB_ADC_TMP112_ADDRESS: u16 = 0x4A;
pub const RB_ADC_TMP112_CHANNEL: u8 = 4;


pub const RB_UIO0: &'static str = "/dev/uio0";

pub const RB_TEMP: u32 = 0xA0;


// I2C Options for Power Board
pub const PB_PCA9548A_ADDRESS: u16 = 0x70;

pub const PB_TMP1075_CHANNEL: u8 = 4;
pub const PB_PDS_TMP1075_ADDRESS: u16 = 0x48;
pub const PB_PAS_TMP1075_ADDRESS: u16 = 0x49;
pub const PB_NAS_TMP1075_ADDRESS: u16 = 0x4A;
pub const PB_SHV_TMP1075_ADDRESS: u16 = 0x4B;

// I2C Options for Local Trigger Board
pub const LTB_TMP112_ADDRESS: u16 = 0x49;