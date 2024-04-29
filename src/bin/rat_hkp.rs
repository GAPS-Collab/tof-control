#![allow(unused)]
use tof_control::constant::*;
use tof_control::memory::{read_control_reg, write_control_reg};
use tof_control::helper::rb_type::RBTempError;
use tof_control::rb_control::rb_temp::read_drs_temp_raw;

fn read_drs_temp() -> Result<f32, RBTempError> {
    let mut drs_temp_raw = read_control_reg(DRS_TEMP)? as u16;

    let mut sign: f32 = 1.0;
    if drs_temp_raw >= 0x800 {
        sign = -1.0;
        drs_temp_raw = 0xFFF - drs_temp_raw;
    }

    let drs_temp = sign * drs_temp_raw as f32 * 0.0625;

    Ok(drs_temp)
}

fn write_drs_temp() -> Result<(), RBTempError> {
    let drs_temp_raw = read_drs_temp_raw()?;
    write_control_reg(DRS_TEMP, drs_temp_raw as u32)?;

    Ok(())
}

fn main() {
    write_drs_temp().unwrap();

    let drs_temp = read_drs_temp().unwrap();
    println!("{}", drs_temp);
}