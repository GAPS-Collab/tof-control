use crate::constant::*;
use crate::device::tmp112;

pub struct LTBtemperature {
    ltb_temp: f32,
}

impl LTBtemperature {
    pub fn new() -> Self {
        let ltb_tmp112 = tmp112::TMP112::new(I2C_BUS, LTB_TMP112_ADDRESS);
        ltb_tmp112.config().expect("cannot configure TMP112");
        let ltb_temp = ltb_tmp112.read().expect("cannot read TMP112");
        Self {
            ltb_temp,
        }
    }
    pub fn print_ltb_temp() {
        let ltb_temp = LTBtemperature::new();
        println!("LTB Temperature:  {:.3}[°C]", ltb_temp.ltb_temp);
    }
}