use crate::constant::*;
use crate::device::{pca9548a, tmp1075, ina226};

pub fn initialize() {
    PBvcp::new();
}

pub struct PBTemp {
    pds_temp: f32,
    pas_temp: f32,
    nas_temp: f32,
    shv_temp: f32,
}

impl PBTemp {
    pub fn new() -> Self {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, PB_PCA9548A_ADDRESS);

        i2c_mux.select(PB_TMP1075_CHANNEL).expect("cannot access to PCA9548A");

        let pds_tmp1075 = tmp1075::TMP1075::new(I2C_BUS, PB_PDS_TMP1075_ADDRESS);
        pds_tmp1075.config().expect("cannot configure TMP1075");
        let pds_temp = pds_tmp1075.read().expect("cannot read TMP1075");

        let pas_tmp1075 = tmp1075::TMP1075::new(I2C_BUS, PB_PAS_TMP1075_ADDRESS);
        pas_tmp1075.config().expect("cannot configure TMP1075");
        let pas_temp = pas_tmp1075.read().expect("cannot read TMP1075");

        let nas_tmp1075 = tmp1075::TMP1075::new(I2C_BUS, PB_NAS_TMP1075_ADDRESS);
        nas_tmp1075.config().expect("cannot configure TMP1075");
        let nas_temp = nas_tmp1075.read().expect("cannot read TMP1075");

        let shv_tmp1075 = tmp1075::TMP1075::new(I2C_BUS, PB_SHV_TMP1075_ADDRESS);
        shv_tmp1075.config().expect("cannot configure TMP1075");
        let shv_temp = shv_tmp1075.read().expect("cannot read TMP1075");

        i2c_mux.reset().expect("cannot reset PCA9548A");

        Self {
            pds_temp,
            pas_temp,
            nas_temp,
            shv_temp,
        }
    }
    pub fn print_pb_temp() {
        let pb_temp = PBTemp::new();
        println!("PDS Temperature:  {:.3}[°C]", pb_temp.pds_temp);
        println!("PAS Temperature:  {:.3}[°C]", pb_temp.pas_temp);
        println!("NAS Temperature:  {:.3}[°C]", pb_temp.nas_temp);
        println!("SHV Temperature:  {:.3}[°C]", pb_temp.shv_temp);
    }
}

// vcp = voltage, current and power
pub struct PBvcp {
    p3v6a_preamp_voltage: f32,
    p3v6a_preamp_current: f32,
    p3v6a_preamp_power: f32,
}

impl PBvcp {
    pub fn new() -> Self {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, PB_PCA9548A_ADDRESS);

        i2c_mux.select(PB_P3V6A_PREAMP_INA226_CHANNEL).expect("cannot access to PCA9548A");
        let p3v6a_preamp_ina226 = ina226::INA226::new(I2C_BUS, PB_P3V6A_PREAMP_INA226_ADDRESS, PB_P3V6A_PREAMP_INA226_RSHUNT, PB_P3V6A_PREAMP_INA226_MEC);
        p3v6a_preamp_ina226.configure().expect("cannot configure INA226 (P3V6A PREAMP)");
        let (p3v6a_preamp_voltage, p3v6a_preamp_current, p3v6a_preamp_power) = p3v6a_preamp_ina226.read_data().expect("cannot read INA226 (P3V6A PREAMP)");

        i2c_mux.reset().expect("cannot reset PCA9548A");

        Self {
            p3v6a_preamp_voltage,
            p3v6a_preamp_current,
            p3v6a_preamp_power,
        }
    }
    pub fn print_pb_vcp() {
        let pb_vcp = PBvcp::new();
        println!("Preamp 3.6V Voltage:    {:.3}[V]", pb_vcp.p3v6a_preamp_voltage);
        println!("Preamp 3.6V Current:    {:.3}[A]", pb_vcp.p3v6a_preamp_current);
        println!("Preamp 3.6V Power:      {:.3}[W]", pb_vcp.p3v6a_preamp_power);
    }
}