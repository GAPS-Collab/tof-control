use crate::constant::*;
use crate::device::{pca9548a, tmp1075};

pub struct PBtemp {
    pds_temp: f32,
    pas_temp: f32,
    nas_temp: f32,
    shv_temp: f32,
}

impl PBtemp {
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
        let pb_temp = PBtemp::new();
        println!("PDS Temperature:          {:.3} [°C]", pb_temp.pds_temp);
        println!("PAS Temperature:          {:.3} [°C]", pb_temp.pas_temp);
        println!("NAS Temperature:          {:.3} [°C]", pb_temp.nas_temp);
        println!("SHV Temperature:          {:.3} [°C]", pb_temp.shv_temp);
    }
}