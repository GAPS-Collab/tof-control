use crate::constant::*;
use crate::device::{pca9548a, tmp1075, ina226, ina219, max11617, max5825, max7320};

pub fn initialize() {
    let mut count = 0;
    while count < 5 {
        PBtemperature::new();
        PBvcp::new();
        count += 1;
    }
}

pub struct PBtemperature {
    pds_temp: f32,
    pas_temp: f32,
    nas_temp: f32,
    shv_temp: f32,
}

impl PBtemperature {
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
        let pb_temp = PBtemperature::new();
        println!("PDS Temperature:  {:.3}[°C]", pb_temp.pds_temp);
        println!("PAS Temperature:  {:.3}[°C]", pb_temp.pas_temp);
        println!("NAS Temperature:  {:.3}[°C]", pb_temp.nas_temp);
        println!("SHV Temperature:  {:.3}[°C]", pb_temp.shv_temp);
    }
}

// vcp = voltage, current and power
pub struct PBvcp {
    p3v6_preamp_voltage: f32,
    p3v6_preamp_current: f32,
    p3v6_preamp_power: f32,
    n1v6_preamp_voltage: f32,
    n1v6_preamp_current: f32,
    n1v6_preamp_power: f32,
    p3v4f_ltb_voltage: f32,
    p3v4f_ltb_current: f32,
    p3v4f_ltb_power: f32,
    p3v4d_ltb_voltage: f32,
    p3v4d_ltb_current: f32,
    p3v4d_ltb_power: f32,
    p3v6_ltb_voltage: f32,
    p3v6_ltb_current: f32,
    p3v6_ltb_power: f32,
    n1v6_ltb_voltage: f32,
    n1v6_ltb_current: f32,
    n1v6_ltb_power: f32,
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
        println!("Preamp 3.6V Voltage:      {:.3}[V]", pb_vcp.p3v6_preamp_voltage);
        println!("Preamp 3.6V Current:      {:.3}[A]", pb_vcp.p3v6_preamp_current);
        println!("Preamp 3.6V Power:        {:.3}[W]", pb_vcp.p3v6_preamp_power);
        println!("Preamp -1.6V Voltage:    {:.3}[V]", pb_vcp.n1v6_preamp_voltage);
        println!("Preamp -1.6V Current:     {:.3}[A]", pb_vcp.n1v6_preamp_current);
        println!("Preamp -1.6V Power:       {:.3}[W]", pb_vcp.n1v6_preamp_power);
        println!("LTB 3.4V FPGA Voltage:    {:.3}[V]", pb_vcp.p3v4f_ltb_voltage);
        println!("LTB 3.4V FPGA Current:    {:.3}[A]", pb_vcp.p3v4f_ltb_current);
        println!("LTB 3.4V FPGA Power:      {:.3}[W]", pb_vcp.p3v4f_ltb_power);
        println!("LTB 3.4V DAC Voltage:     {:.3}[V]", pb_vcp.p3v4d_ltb_voltage);
        println!("LTB 3.4V DAC Current:     {:.3}[A]", pb_vcp.p3v4d_ltb_current);
        println!("LTB 3.4V DAC Power:       {:.3}[W]", pb_vcp.p3v4d_ltb_power);
        println!("LTB 3.6V Voltage:         {:.3}[V]", pb_vcp.p3v6_ltb_voltage);
        println!("LTB 3.6V Current:         {:.3}[A]", pb_vcp.p3v6_ltb_current);
        println!("LTB 3.6V Power:           {:.3}[W]", pb_vcp.p3v6_ltb_power);
        println!("LTB -1.6V Voltage:       {:.3}[V]", pb_vcp.n1v6_ltb_voltage);
        println!("LTB -1.6V Current:        {:.3}[A]", pb_vcp.n1v6_ltb_current);
        println!("LTB -1.6V Power:          {:.3}[W]", pb_vcp.n1v6_ltb_power);
    }
}

pub struct PBdac {
    device_id_1: u8,
    wd_enabled_1: u8,
    ref_mode_1: u8,
    clr_enabled_1: u8,
    rev_id_1: u8,
    device_id_2: u8,
    wd_enabled_2: u8,
    ref_mode_2: u8,
    clr_enabled_2: u8,
    rev_id_2: u8,
    code_ch1: u16,
    code_ch2: u16,
    code_ch3: u16,
    code_ch4: u16,
    code_ch5: u16,
    code_ch6: u16,
    code_ch7: u16,
    code_ch8: u16,
    code_ch9: u16,
    code_ch10: u16,
    code_ch11: u16,
    code_ch12: u16,
    code_ch13: u16,
    code_ch14: u16,
    code_ch15: u16,
    code_ch16: u16,
    dac_ch1: u16,
    dac_ch2: u16,
    dac_ch3: u16,
    dac_ch4: u16,
    dac_ch5: u16,
    dac_ch6: u16,
    dac_ch7: u16,
    dac_ch8: u16,
    dac_ch9: u16,
    dac_ch10: u16,
    dac_ch11: u16,
    dac_ch12: u16,
    dac_ch13: u16,
    dac_ch14: u16,
    dac_ch15: u16,
    dac_ch16: u16,
}

impl PBdac {
    pub fn new() -> Self {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, PB_PCA9548A_ADDRESS);

        i2c_mux.select(PB_DAC_1_CHANNEL).expect("cannot access to PCA9548A");
        let pb_dac_1 = max5825::MAX5825::new(I2C_BUS, PB_MAX5825_ADDRESS);
        let (wd_enabled_1, ref_mode_1, clr_enabled_1, rev_id_1, device_id_1) = pb_dac_1.read_device_info().expect("cannot read MAX5825(1)");

        let code_ch1 = pb_dac_1.read_coden(PREAMP_DAC_1_CHANNEL).expect("cannot read MAX5825");
        let dac_ch1 = pb_dac_1.read_dacn(PREAMP_DAC_1_CHANNEL).expect("cannot read MAX5825");
        let code_ch2 = pb_dac_1.read_coden(PREAMP_DAC_2_CHANNEL).expect("cannot read MAX5825");
        let dac_ch2 = pb_dac_1.read_dacn(PREAMP_DAC_2_CHANNEL).expect("cannot read MAX5825");
        let code_ch3 = pb_dac_1.read_coden(PREAMP_DAC_3_CHANNEL).expect("cannot read MAX5825");
        let dac_ch3 = pb_dac_1.read_dacn(PREAMP_DAC_3_CHANNEL).expect("cannot read MAX5825");
        let code_ch4 = pb_dac_1.read_coden(PREAMP_DAC_4_CHANNEL).expect("cannot read MAX5825");
        let dac_ch4 = pb_dac_1.read_dacn(PREAMP_DAC_4_CHANNEL).expect("cannot read MAX5825");
        let code_ch5 = pb_dac_1.read_coden(PREAMP_DAC_1_CHANNEL).expect("cannot read MAX5825");
        let dac_ch5 = pb_dac_1.read_dacn(PREAMP_DAC_1_CHANNEL).expect("cannot read MAX5825");
        let code_ch6 = pb_dac_1.read_coden(PREAMP_DAC_6_CHANNEL).expect("cannot read MAX5825");
        let dac_ch6 = pb_dac_1.read_dacn(PREAMP_DAC_6_CHANNEL).expect("cannot read MAX5825");
        let code_ch7 = pb_dac_1.read_coden(PREAMP_DAC_7_CHANNEL).expect("cannot read MAX5825");
        let dac_ch7 = pb_dac_1.read_dacn(PREAMP_DAC_7_CHANNEL).expect("cannot read MAX5825");
        let code_ch8 = pb_dac_1.read_coden(PREAMP_DAC_8_CHANNEL).expect("cannot read MAX5825");
        let dac_ch8 = pb_dac_1.read_dacn(PREAMP_DAC_8_CHANNEL).expect("cannot read MAX5825");

        i2c_mux.select(PB_DAC_2_CHANNEL).expect("cannot access to PCA9548A");
        let pb_dac_2 = max5825::MAX5825::new(I2C_BUS, PB_MAX5825_ADDRESS);
        let (wd_enabled_2, ref_mode_2, clr_enabled_2, rev_id_2, device_id_2) = pb_dac_2.read_device_info().expect("cannot read MAX5825(2)");

        let code_ch9 = pb_dac_2.read_coden(PREAMP_DAC_9_CHANNEL).expect("cannot read MAX5825");
        let dac_ch9 = pb_dac_2.read_dacn(PREAMP_DAC_9_CHANNEL).expect("cannot read MAX5825");
        let code_ch10 = pb_dac_2.read_coden(PREAMP_DAC_10_CHANNEL).expect("cannot read MAX5825");
        let dac_ch10 = pb_dac_2.read_dacn(PREAMP_DAC_10_CHANNEL).expect("cannot read MAX5825");
        let code_ch11 = pb_dac_2.read_coden(PREAMP_DAC_11_CHANNEL).expect("cannot read MAX5825");
        let dac_ch11 = pb_dac_2.read_dacn(PREAMP_DAC_11_CHANNEL).expect("cannot read MAX5825");
        let code_ch12 = pb_dac_2.read_coden(PREAMP_DAC_12_CHANNEL).expect("cannot read MAX5825");
        let dac_ch12 = pb_dac_2.read_dacn(PREAMP_DAC_12_CHANNEL).expect("cannot read MAX5825");
        let code_ch13 = pb_dac_2.read_coden(PREAMP_DAC_13_CHANNEL).expect("cannot read MAX5825");
        let dac_ch13 = pb_dac_2.read_dacn(PREAMP_DAC_13_CHANNEL).expect("cannot read MAX5825");
        let code_ch14 = pb_dac_2.read_coden(PREAMP_DAC_14_CHANNEL).expect("cannot read MAX5825");
        let dac_ch14 = pb_dac_2.read_dacn(PREAMP_DAC_14_CHANNEL).expect("cannot read MAX5825");
        let code_ch15 = pb_dac_2.read_coden(PREAMP_DAC_15_CHANNEL).expect("cannot read MAX5825");
        let dac_ch15 = pb_dac_2.read_dacn(PREAMP_DAC_15_CHANNEL).expect("cannot read MAX5825");
        let code_ch16 = pb_dac_2.read_coden(PREAMP_DAC_16_CHANNEL).expect("cannot read MAX5825");
        let dac_ch16 = pb_dac_2.read_dacn(PREAMP_DAC_16_CHANNEL).expect("cannot read MAX5825");

        i2c_mux.reset().expect("cannot reset PCA9548A");

        Self {
            device_id_1,
            wd_enabled_1,
            ref_mode_1,
            clr_enabled_1,
            rev_id_1,
            device_id_2,
            wd_enabled_2,
            ref_mode_2,
            clr_enabled_2,
            rev_id_2,
            code_ch1,
            code_ch2,
            code_ch3,
            code_ch4,
            code_ch5,
            code_ch6,
            code_ch7,
            code_ch8,
            code_ch9,
            code_ch10,
            code_ch11,
            code_ch12,
            code_ch13,
            code_ch14,
            code_ch15,
            code_ch16,
            dac_ch1,
            dac_ch2,
            dac_ch3,
            dac_ch4,
            dac_ch5,
            dac_ch6,
            dac_ch7,
            dac_ch8,
            dac_ch9,
            dac_ch10,
            dac_ch11,
            dac_ch12,
            dac_ch13,
            dac_ch14,
            dac_ch15,
            dac_ch16,
        }
    }
    pub fn print_pb_dac() {
        let pb_dac = PBdac::new();
        println!("--- Device Information ---");
        println!("Device ID 1:        {}", pb_dac.device_id_1);
        println!("Device ID 2:        {}", pb_dac.device_id_2);
        println!("Revision ID 1:      {}", pb_dac.rev_id_1);
        println!("Revision ID 2:      {}", pb_dac.rev_id_2);
        println!("Reference Mode 1:   {}", pb_dac.ref_mode_1);
        println!("Reference Mode 2:   {}", pb_dac.ref_mode_2);
        println!("Watchdog Status 1:  {}", pb_dac.wd_enabled_1);
        println!("Watchdog Status 2:  {}", pb_dac.wd_enabled_2);
        println!("Clear Status 1:     {}", pb_dac.clr_enabled_1);
        println!("Clear Status 2:     {}", pb_dac.clr_enabled_2);
        println!("--- DAC(CODE) ---");
        println!("Ch1 DAC:            {:.3}[V]({})", Self::adc_to_bias(pb_dac.dac_ch1), pb_dac.code_ch1);
        println!("Ch2 DAC:            {:.3}[V]({})", Self::adc_to_bias(pb_dac.dac_ch2), pb_dac.code_ch2);
        println!("Ch3 DAC:            {:.3}[V]({})", Self::adc_to_bias(pb_dac.dac_ch3), pb_dac.code_ch3);
        println!("Ch4 DAC:            {:.3}[V]({})", Self::adc_to_bias(pb_dac.dac_ch4), pb_dac.code_ch4);
        println!("Ch5 DAC:            {:.3}[V]({})", Self::adc_to_bias(pb_dac.dac_ch5), pb_dac.code_ch5);
        println!("Ch6 DAC:            {:.3}[V]({})", Self::adc_to_bias(pb_dac.dac_ch6), pb_dac.code_ch6);
        println!("Ch7 DAC:            {:.3}[V]({})", Self::adc_to_bias(pb_dac.dac_ch7), pb_dac.code_ch7);
        println!("Ch8 DAC:            {:.3}[V]({})", Self::adc_to_bias(pb_dac.dac_ch8), pb_dac.code_ch8);
        println!("Ch9 DAC:            {:.3}[V]({})", Self::adc_to_bias(pb_dac.dac_ch9), pb_dac.code_ch9);
        println!("Ch10 DAC:           {:.3}[V]({})", Self::adc_to_bias(pb_dac.dac_ch10), pb_dac.code_ch10);
        println!("Ch11 DAC:           {:.3}[V]({})", Self::adc_to_bias(pb_dac.dac_ch11), pb_dac.code_ch11);
        println!("Ch12 DAC:           {:.3}[V]({})", Self::adc_to_bias(pb_dac.dac_ch12), pb_dac.code_ch12);
        println!("Ch13 DAC:           {:.3}[V]({})", Self::adc_to_bias(pb_dac.dac_ch13), pb_dac.code_ch13);
        println!("Ch14 DAC:           {:.3}[V]({})", Self::adc_to_bias(pb_dac.dac_ch14), pb_dac.code_ch14);
        println!("Ch15 DAC:           {:.3}[V]({})", Self::adc_to_bias(pb_dac.dac_ch15), pb_dac.code_ch15);
        println!("Ch16 DAC:           {:.3}[V]({})", Self::adc_to_bias(pb_dac.dac_ch16), pb_dac.code_ch16);
    }
    fn adc_to_voltage(adc: u16) -> f32 {
        let voltage = PB_DAC_REF_VOLTAGE * (adc as f32) / 2f32.powf(12.0);

        voltage
    }
    fn adc_to_bias(adc: u16) -> f32 {
        let bias_voltage = Self::adc_to_voltage(adc) * 22.3;

        bias_voltage
    }
}


// Issue when powering off LTB (February 1, 2023)
pub struct LTBpwr {
}

impl LTBpwr {
    pub fn power_on_ltb() {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, PB_PCA9548A_ADDRESS);

        i2c_mux.select(PB_MAX7320_CHANNEL).expect("cannot access to PCA9548A");
        let ltb_pwr = max7320::MAX7320::new(I2C_BUS, PB_MAX7320_ADDRESS);
        ltb_pwr.output_on_0_3().expect("cannot power on LTB (MAX7320)")
    }
    pub fn power_off_ltb() {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, PB_PCA9548A_ADDRESS);

        i2c_mux.select(PB_MAX7320_CHANNEL).expect("cannot access to PCA9548A");
        let ltb_pwr = max7320::MAX7320::new(I2C_BUS, PB_MAX7320_ADDRESS);
        ltb_pwr.output_off_all().expect("cannot power off LTB (MAX7320)")
    }
}