use std::process::Command;

use tof_control::constant::*;
use tof_control::device::{ina219, max7320, tmp1075};

fn main() {
    // check_i2cdev_number();
    println!("CPC Temperature:          {:.3} [°C]", cpc_temperature());
    println!(
        "CPC Power:                {:.3} [V] | {:.3} [A] | {:.3} [W]",
        cpc_vcp().0,
        cpc_vcp().1,
        cpc_vcp().2
    );
}

fn cpc_temperature() -> f32 {
    let cpc_tmp1075 = tmp1075::TMP1075::new(3, CPC_TMP1075_ADDRESS);
    cpc_tmp1075.config().expect("cannot configure TMP1075");
    let cpc_temp = cpc_tmp1075.read().expect("cannot read TMP1075");

    cpc_temp
}

fn cpc_vcp() -> (f32, f32, f32) {
    let cpc_ina219 = ina219::INA219::new(3, CPC_INA219_ADDRESS, CPC_INA219_RSHUNT, CPC_INA219_MEC);
    cpc_ina219.configure().expect("cannot configure INA219");
    let (cpc_voltage, cpc_current, cpc_power) = cpc_ina219.read_data().expect("cannot read INA219");

    (cpc_voltage, cpc_current, cpc_power)
}

// fn check_i2cdev_number() {
//     let output = Command::new("i2cdetect")
//                                 .arg("-l")
//                                 .arg("|")
//                                 .arg("grep")
//                                 .arg("'iManager SMB 1 adapter'").output().unwrap();
//     println!("{:?}", output);
// }
