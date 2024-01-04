use tof_control::constant::*;
use tof_control::device::{ina219, max7320, tmp1075};

fn main() {
    println!("TCPC Temperature:          {:.3} [°C]", tcpc_temperature());
    println!(
        "TCPC Power:                {:.3} [V] | {:.3} [A] | {:.3} [W]",
        tcpc_vcp().0,
        tcpc_vcp().1,
        tcpc_vcp().2
    );
}

fn tcpc_temperature() -> f32 {
    let tcpc_tmp1075 = tmp1075::TMP1075::new(1, TCPC_TMP1075_ADDRESS);
    tcpc_tmp1075.config().expect("cannot configure TMP1075");
    let tcpc_temp = tcpc_tmp1075.read().expect("cannot read TMP1075");

    tcpc_temp
}

fn tcpc_vcp() -> (f32, f32, f32) {
    let tcpc_ina219 =
        ina219::INA219::new(1, TCPC_INA219_ADDRESS, TCPC_INA219_RSHUNT, TCPC_INA219_MEC);
    tcpc_ina219.configure().expect("cannot configure INA219");
    let (tcpc_voltage, tcpc_current, tcpc_power) =
        tcpc_ina219.read_data().expect("cannot read INA219");

    (tcpc_voltage, tcpc_current, tcpc_power)
}
