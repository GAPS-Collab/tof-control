use crate::constant::*;
use crate::device::{pca9548a, ina226, ina219, max11617};

pub struct PBvcp {
    pub p3v6_preamp_voltage: f32,
    pub p3v6_preamp_current: f32,
    pub p3v6_preamp_power: f32,
    pub n1v6_preamp_voltage: f32,
    pub n1v6_preamp_current: f32,
    pub n1v6_preamp_power: f32,
    pub p3v4f_ltb_voltage: f32,
    pub p3v4f_ltb_current: f32,
    pub p3v4f_ltb_power: f32,
    pub p3v4d_ltb_voltage: f32,
    pub p3v4d_ltb_current: f32,
    pub p3v4d_ltb_power: f32,
    pub p3v6_ltb_voltage: f32,
    pub p3v6_ltb_current: f32,
    pub p3v6_ltb_power: f32,
    pub n1v6_ltb_voltage: f32,
    pub n1v6_ltb_current: f32,
    pub n1v6_ltb_power: f32,
}

impl PBvcp {
    pub fn new() -> Self {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, PB_PCA9548A_ADDRESS);

        i2c_mux.select(PB_P3V6_PREAMP_INA226_CHANNEL).expect("cannot access to PCA9548A");
        let p3v6_preamp_ina226 = ina226::INA226::new(I2C_BUS, PB_P3V6_PREAMP_INA226_ADDRESS, PB_P3V6_PREAMP_INA226_RSHUNT, PB_P3V6_PREAMP_INA226_MEC);
        p3v6_preamp_ina226.configure().expect("cannot configure INA226 (P3V6 PREAMP)");
        let (p3v6_preamp_voltage, p3v6_preamp_current, p3v6_preamp_power) = p3v6_preamp_ina226.read_data().expect("cannot read INA226 (P3V6 PREAMP)");

        i2c_mux.select(PB_ADC_1_CHANNEL).expect("cannot access to PCA9548A");
        let max11617 = max11617::MAX11617::new(I2C_BUS, PB_MAX11617_ADDRESS);
        max11617.setup().expect("cannot setup MAX11617");
        let n1v6_preamp_voltage = max11617.read(PB_N1V6_PREAMP_VOLTAGE_INA201_CHANNEL).expect("cannot read INA201 (N1V6 PREAMP)") * -1.0;
        let n1v6_preamp_current = max11617.read(PB_N1V6_PREAMP_CURRENT_INA201_CHANNEL).expect("cannot read INA201 (N1V6 PREAMP)") / 50.0 / 0.1;
        let n1v6_preamp_power = n1v6_preamp_voltage.abs() * n1v6_preamp_current;

        i2c_mux.select(PB_LTB_INA219_CHANNEL).expect("cannot access to PCA9548A");

        let p3v4f_ltb_ina219 = ina219::INA219::new(I2C_BUS, PB_P3V4F_LTB_INA219_ADDRESS, PB_P3V4F_LTB_INA219_RSHUNT, PB_P3V4F_LTB_INA219_MEC);
        p3v4f_ltb_ina219.configure().expect("cannot configure INA219 (P3V4F LTB)");
        let (p3v4f_ltb_voltage, p3v4f_ltb_current, p3v4f_ltb_power) = p3v4f_ltb_ina219.read_data().expect("cannot read INA219 (P3V4F LTB)");

        let p3v4d_ltb_ina219 = ina219::INA219::new(I2C_BUS, PB_P3V4D_LTB_INA219_ADDRESS, PB_P3V4D_LTB_INA219_RSHUNT, PB_P3V4D_LTB_INA219_MEC);
        p3v4d_ltb_ina219.configure().expect("cannot configure INA219 (P3V4D LTB)");
        let (p3v4d_ltb_voltage, p3v4d_ltb_current, p3v4d_ltb_power) = p3v4d_ltb_ina219.read_data().expect("cannot read INA219 (P3V4D LTB)");

        let p3v6_ltb_ina219 = ina219::INA219::new(I2C_BUS, PB_P3V6_LTB_INA219_ADDRESS, PB_P3V6_LTB_INA219_RSHUNT, PB_P3V6_LTB_INA219_MEC);
        p3v6_ltb_ina219.configure().expect("cannot configure INA219 (P3V6 LTB)");
        let (p3v6_ltb_voltage, p3v6_ltb_current, p3v6_ltb_power) = p3v6_ltb_ina219.read_data().expect("cannot read INA219 (P3V6 LTB)");

        i2c_mux.select(PB_ADC_2_CHANNEL).expect("cannot access to PCA9548A");
        let max11617 = max11617::MAX11617::new(I2C_BUS, PB_MAX11617_ADDRESS);
        max11617.setup().expect("cannot setup MAX11617");
        let n1v6_ltb_voltage = max11617.read(PB_N1V6_LTB_VOLTAGE_INA202_CHANNEL).expect("cannot read INA202 (N1V6 LTB)") * -1.0;
        let n1v6_ltb_current = max11617.read(PB_N1V6_LTB_CURRENT_INA202_CHANNEL).expect("cannot read INA202 (N1V6 LTB)") / 100.0 / 0.1;
        let n1v6_ltb_power = n1v6_ltb_voltage.abs() * n1v6_ltb_current;

        i2c_mux.reset().expect("cannot reset PCA9548A");

        Self {
            p3v6_preamp_voltage,
            p3v6_preamp_current,
            p3v6_preamp_power,
            n1v6_preamp_voltage,
            n1v6_preamp_current,
            n1v6_preamp_power,
            p3v4f_ltb_voltage,
            p3v4f_ltb_current,
            p3v4f_ltb_power,
            p3v4d_ltb_voltage,
            p3v4d_ltb_current,
            p3v4d_ltb_power,
            p3v6_ltb_voltage,
            p3v6_ltb_current,
            p3v6_ltb_power,
            n1v6_ltb_voltage,
            n1v6_ltb_current,
            n1v6_ltb_power,
        }
    }
    pub fn print_pb_vcp() {
        let pb_vcp = PBvcp::new();
        println!("LTB 3.4V FPGA Power:      {:.3} [V] | {:.3} [A] | {:.3} [W]", pb_vcp.p3v4f_ltb_voltage, pb_vcp.p3v4f_ltb_current, pb_vcp.p3v4f_ltb_power);
        println!("LTB 3.4V DAC Power:       {:.3} [V] | {:.3} [A] | {:.3} [W]", pb_vcp.p3v4d_ltb_voltage, pb_vcp.p3v4d_ltb_current, pb_vcp.p3v4d_ltb_power);
        println!("LTB 3.6V Power:           {:.3} [V] | {:.3} [A] | {:.3} [W]", pb_vcp.p3v6_ltb_voltage, pb_vcp.p3v6_ltb_current, pb_vcp.p3v6_ltb_power);
        println!("LTB -1.6V Power:         {:.3} [V] | {:.3} [A] | {:.3} [W]", pb_vcp.n1v6_ltb_voltage, pb_vcp.n1v6_ltb_current, pb_vcp.n1v6_ltb_power);
        println!("Preamp 3.6V Power:        {:.3} [V] | {:.3} [A] | {:.3} [W]", pb_vcp.p3v6_preamp_voltage, pb_vcp.p3v6_preamp_current, pb_vcp.p3v6_preamp_power);
        println!("Preamp -1.6V Power:      {:.3} [V] | {:.3} [A] | {:.3} [W]", pb_vcp.n1v6_preamp_voltage, pb_vcp.n1v6_preamp_current, pb_vcp.n1v6_preamp_power);
    }
}