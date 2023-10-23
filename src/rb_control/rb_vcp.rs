use crate::constant::*;

use crate::helper::rb_type::RBVcpError;
use crate::device::{ina226, max11645, pca9548a};

pub struct RBvcp {
    pub drs_dvdd_voltage: f32,
    pub drs_dvdd_current: f32,
    pub drs_dvdd_power: f32,
    pub p3v3_voltage: f32,
    pub p3v3_current: f32,
    pub p3v3_power: f32,
    pub zynq_voltage: f32,
    pub zynq_current: f32,
    pub zynq_power: f32,
    pub p3v5_voltage: f32,
    pub p3v5_current: f32,
    pub p3v5_power: f32,
    pub adc_dvdd_voltage: f32,
    pub adc_dvdd_current: f32,
    pub adc_dvdd_power: f32,
    pub adc_avdd_voltage: f32,
    pub adc_avdd_current: f32,
    pub adc_avdd_power: f32,
    pub drs_avdd_voltage: f32,
    pub drs_avdd_current: f32,
    pub drs_avdd_power: f32,
    pub n1v5_voltage: f32,
    pub n1v5_current: f32,
    pub n1v5_power: f32,
}

impl RBvcp {
    pub fn new() -> Self {
        let i2c_mux_1 = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_1);
        let i2c_mux_2 = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);

        i2c_mux_1
            .select(RB_DRS_DVDD_INA226_CHANNEL)
            .expect("cannot accesss to PCA9548A");
        let drs_dvdd_ina226 = ina226::INA226::new(
            I2C_BUS,
            RB_DRS_DVDD_INA226_ADDRESS,
            RB_DRS_DVDD_INA226_RSHUNT,
            RB_DRS_DVDD_INA226_MEC,
        );
        drs_dvdd_ina226
            .configure()
            .expect("cannot configure INA226 (DRS DVDD)");
        let (drs_dvdd_voltage, drs_dvdd_current, drs_dvdd_power) = drs_dvdd_ina226
            .read_data()
            .expect("cannot read INA226 (DRS DVDD)");

        i2c_mux_1
            .select(RB_P3V3_INA226_CHANNEL)
            .expect("cannot accesss to PCA9548A");
        let p3v3_ina226 = ina226::INA226::new(
            I2C_BUS,
            RB_P3V3_INA226_ADDRESS,
            RB_P3V3_INA226_RSHUNT,
            RB_P3V3_INA226_MEC,
        );
        p3v3_ina226
            .configure()
            .expect("cannot configure INA226 (P3V3)");
        let (p3v3_voltage, p3v3_current, p3v3_power) =
            p3v3_ina226.read_data().expect("cannot read INA226 (P3V3)");

        i2c_mux_1
            .select(RB_ZYNQ_INA226_CHANNEL)
            .expect("cannot accesss to PCA9548A");
        let zynq_ina226 = ina226::INA226::new(
            I2C_BUS,
            RB_ZYNQ_INA226_ADDRESS,
            RB_ZYNQ_INA226_RSHUNT,
            RB_ZYNQ_INA226_MEC,
        );
        zynq_ina226
            .configure()
            .expect("cannot configure INA226 (ZYNQ)");
        let (zynq_voltage, zynq_current, zynq_power) =
            zynq_ina226.read_data().expect("cannot read INA226 (ZYNQ)");

        i2c_mux_1
            .select(RB_P3V5_INA226_CHANNEL)
            .expect("cannot accesss to PCA9548A");
        let p3v5_ina226 = ina226::INA226::new(
            I2C_BUS,
            RB_P3V5_INA226_ADDRESS,
            RB_P3V5_INA226_RSHUNT,
            RB_P3V5_INA226_MEC,
        );
        p3v5_ina226
            .configure()
            .expect("cannot configure INA226 (P3V5)");
        let (p3v5_voltage, p3v5_current, p3v5_power) =
            p3v5_ina226.read_data().expect("cannot read INA226 (P3V5)");

        i2c_mux_2
            .select(RB_ADC_DVDD_INA226_CHANNEL)
            .expect("cannot accesss to PCA9548A");
        let adc_dvdd_ina226 = ina226::INA226::new(
            I2C_BUS,
            RB_ADC_DVDD_INA226_ADDRESS,
            RB_ADC_DVDD_INA226_RSHUNT,
            RB_ADC_DVDD_INA226_MEC,
        );
        adc_dvdd_ina226
            .configure()
            .expect("cannot configure INA226 (ADC DVDD)");
        let (adc_dvdd_voltage, adc_dvdd_current, adc_dvdd_power) = adc_dvdd_ina226
            .read_data()
            .expect("cannot read INA226 (ADC DVDD)");

        i2c_mux_2
            .select(RB_ADC_AVDD_INA226_CHANNEL)
            .expect("cannot accesss to PCA9548A");
        let adc_avdd_ina226 = ina226::INA226::new(
            I2C_BUS,
            RB_ADC_AVDD_INA226_ADDRESS,
            RB_ADC_AVDD_INA226_RSHUNT,
            RB_ADC_AVDD_INA226_MEC,
        );
        adc_avdd_ina226
            .configure()
            .expect("cannot configure INA226 (ADC AVDD)");
        let (adc_avdd_voltage, adc_avdd_current, adc_avdd_power) = adc_avdd_ina226
            .read_data()
            .expect("cannot read INA226 (ADC AVDD)");

        i2c_mux_2
            .select(RB_DRS_AVDD_INA226_CHANNEL)
            .expect("cannot accesss to PCA9548A");
        let drs_avdd_ina226 = ina226::INA226::new(
            I2C_BUS,
            RB_DRS_AVDD_INA226_ADDRESS,
            RB_DRS_AVDD_INA226_RSHUNT,
            RB_DRS_AVDD_INA226_MEC,
        );
        drs_avdd_ina226
            .configure()
            .expect("cannot configure INA226 (DRS AVDD)");
        let (drs_avdd_voltage, drs_avdd_current, drs_avdd_power) = drs_avdd_ina226
            .read_data()
            .expect("cannot read INA226 (DRS AVDD)");

        i2c_mux_1
            .select(RB_MAX11645_CHANNEL)
            .expect("cannot accesss to PCA9548A");
        let max11645 = max11645::MAX11645::new(I2C_BUS, RB_MAX11645_ADDRESS);
        max11645.setup().expect("cannot configure MAX11645");
        let n1v5_voltage = max11645
            .read(RB_N1V5_VOLTAGE_INA200_CHANNEL)
            .expect("cannot read INA200 (N1V5 VOLTAGE)")
            * -1.0;
        let n1v5_current = max11645
            .read(RB_N1V5_CURRENT_INA200_CHANNEL)
            .expect("cannot read INA200 (N1V5 CURRENT)")
            / 20.0
            / 0.039;
        let n1v5_power = n1v5_voltage.abs() * n1v5_current;

        i2c_mux_1.reset().expect("cannot reset PCA9548A");
        i2c_mux_2.reset().expect("cannot reset PCA9548A");

        Self {
            drs_dvdd_voltage,
            drs_dvdd_current,
            drs_dvdd_power,
            p3v3_voltage,
            p3v3_current,
            p3v3_power,
            zynq_voltage,
            zynq_current,
            zynq_power,
            p3v5_voltage,
            p3v5_current,
            p3v5_power,
            adc_dvdd_voltage,
            adc_dvdd_current,
            adc_dvdd_power,
            adc_avdd_voltage,
            adc_avdd_current,
            adc_avdd_power,
            drs_avdd_voltage,
            drs_avdd_current,
            drs_avdd_power,
            n1v5_voltage,
            n1v5_current,
            n1v5_power,
        }
    }
    pub fn print_rb_vcp() {
        let rb_vcp = RBvcp::new();
        println!(
            "ZYNQ 3.3V Power:          {:.3} [V] | {:.3} [A] | {:.3} [W]",
            rb_vcp.zynq_voltage, rb_vcp.zynq_current, rb_vcp.zynq_power
        );
        println!(
            "3.3V Power:               {:.3} [V] | {:.3} [A] | {:.3} [W]",
            rb_vcp.p3v3_voltage, rb_vcp.p3v3_current, rb_vcp.p3v3_power
        );
        println!(
            "3.5V Power:               {:.3} [V] | {:.3} [A] | {:.3} [W]",
            rb_vcp.p3v5_voltage, rb_vcp.p3v5_current, rb_vcp.p3v5_power
        );
        println!(
            "-1.5V Power:             {:.3} [V] | {:.3} [A] | {:.3} [W]",
            rb_vcp.n1v5_voltage, rb_vcp.n1v5_current, rb_vcp.n1v5_power
        );
        println!(
            "DRS4 Digital 2.5V Power:  {:.3} [V] | {:.3} [A] | {:.3} [W]",
            rb_vcp.drs_dvdd_voltage, rb_vcp.drs_dvdd_current, rb_vcp.drs_dvdd_power
        );
        println!(
            "DRS4 Analog 2.5V Power:   {:.3} [V] | {:.3} [A] | {:.3} [W]",
            rb_vcp.drs_avdd_voltage, rb_vcp.drs_avdd_current, rb_vcp.drs_avdd_power
        );
        println!(
            "ADC Digital 2.5V Power:   {:.3} [V] | {:.3} [A] | {:.3} [W]",
            rb_vcp.adc_dvdd_voltage, rb_vcp.adc_dvdd_current, rb_vcp.adc_dvdd_power
        );
        println!(
            "ADC Analog 3.0V Power:    {:.3} [V] | {:.3} [A] | {:.3} [W]",
            rb_vcp.adc_avdd_voltage, rb_vcp.adc_avdd_current, rb_vcp.adc_avdd_power
        );
    }
}

pub fn config_vcp() -> Result<(), RBVcpError> {
    let i2c_mux_1 = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_1);
    let i2c_mux_2 = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);

    i2c_mux_1.select(RB_DRS_DVDD_INA226_CHANNEL)?;
    let drs_dvdd_ina226 = ina226::INA226::new(
        I2C_BUS,
        RB_DRS_DVDD_INA226_ADDRESS,
        RB_DRS_DVDD_INA226_RSHUNT,
        RB_DRS_DVDD_INA226_MEC,
    );
    drs_dvdd_ina226.configure()?;

    i2c_mux_1.select(RB_P3V3_INA226_CHANNEL)?;
    let p3v3_ina226 = ina226::INA226::new(
        I2C_BUS,
        RB_P3V3_INA226_ADDRESS,
        RB_P3V3_INA226_RSHUNT,
        RB_P3V3_INA226_MEC,
    );
    p3v3_ina226.configure()?;

    i2c_mux_1.select(RB_ZYNQ_INA226_CHANNEL)?;
    let zynq_ina226 = ina226::INA226::new(
        I2C_BUS,
        RB_ZYNQ_INA226_ADDRESS,
        RB_ZYNQ_INA226_RSHUNT,
        RB_ZYNQ_INA226_MEC,
    );
    zynq_ina226.configure()?;

    i2c_mux_1.select(RB_P3V5_INA226_CHANNEL)?;
    let p3v5_ina226 = ina226::INA226::new(
        I2C_BUS,
        RB_P3V5_INA226_ADDRESS,
        RB_P3V5_INA226_RSHUNT,
        RB_P3V5_INA226_MEC,
    );
    p3v5_ina226.configure()?;

    i2c_mux_2.select(RB_ADC_DVDD_INA226_CHANNEL)?;
    let adc_dvdd_ina226 = ina226::INA226::new(
        I2C_BUS,
        RB_ADC_DVDD_INA226_ADDRESS,
        RB_ADC_DVDD_INA226_RSHUNT,
        RB_ADC_DVDD_INA226_MEC,
    );
    adc_dvdd_ina226.configure()?;

    i2c_mux_2.select(RB_ADC_AVDD_INA226_CHANNEL)?;
    let adc_avdd_ina226 = ina226::INA226::new(
        I2C_BUS,
        RB_ADC_AVDD_INA226_ADDRESS,
        RB_ADC_AVDD_INA226_RSHUNT,
        RB_ADC_AVDD_INA226_MEC,
    );
    adc_avdd_ina226.configure()?;

    i2c_mux_2.select(RB_DRS_AVDD_INA226_CHANNEL)?;
    let drs_avdd_ina226 = ina226::INA226::new(
        I2C_BUS,
        RB_DRS_AVDD_INA226_ADDRESS,
        RB_DRS_AVDD_INA226_RSHUNT,
        RB_DRS_AVDD_INA226_MEC,
    );
    drs_avdd_ina226.configure()?;

    i2c_mux_1.select(RB_MAX11645_CHANNEL)?;
    let max11645 = max11645::MAX11645::new(I2C_BUS, RB_MAX11645_ADDRESS);
    max11645.setup()?;

    i2c_mux_1.reset()?;
    i2c_mux_2.reset()?;

    Ok(())
}