use crate::constant::*;
use crate::memory::*;
use crate::device::{pca9548a, tmp112, ina226, lis3mdltr, bme280, max11645, si5345b, cy8c9560a, ad5675};


pub fn initialize() {
    let mut count = 0;
    while count < 5 {
        RBtemperature::new();
        RBvcp::new();
        RBpressure::new();
        RBhumidity::new();
        RBmagnetic::new();
        count += 1;
    }
}

pub struct RBtemperature {
    drs_temp: f32,
    clk_temp: f32,
    adc_temp: f32,
    lis3mdltr_temp: f32,
    bme280_temp: f32,
    zynq_temp: f32,
}

impl RBtemperature {
    pub fn new() -> Self {
        let i2c_mux_1 = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_1);
        let i2c_mux_2 = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
        
        i2c_mux_1.select(RB_DRS_TMP112_CHANNEL).expect("cannot accesss to PCA9548A");
        let drs_tmp112 = tmp112::TMP112::new(I2C_BUS, RB_DRS_TMP112_ADDRESS);
        drs_tmp112.config().expect("cannot configure TMP112");
        let drs_temp = drs_tmp112.read().expect("cannot read TMP112");
    
        i2c_mux_2.select(RB_CLK_TMP112_CHANNEL).expect("cannot accesss to PCA9548A");
        let clk_tmp112 = tmp112::TMP112::new(I2C_BUS, RB_CLK_TMP112_ADDRESS);
        clk_tmp112.config().expect("cannot configure TMP112");
        let clk_temp = clk_tmp112.read().expect("cannot read TMP112");
    
        i2c_mux_2.select(RB_ADC_TMP112_CHANNEL).expect("cannot accesss to PCA9548A");
        let adc_tmp112 = tmp112::TMP112::new(I2C_BUS, RB_ADC_TMP112_ADDRESS);
        adc_tmp112.config().expect("cannot configure TMP112");
        let adc_temp = adc_tmp112.read().expect("cannot read TMP112");

        i2c_mux_1.select(RB_LIS3MDLTR_CHANNEL).expect("cannot accesss to PCA9548A");
        let lis3mdltr = lis3mdltr::LIS3MDLTR::new(I2C_BUS, RB_LIS3MDLTR_ADDRESS);
        lis3mdltr.configure();
        let lis3mdltr_temp = lis3mdltr.read_temperature().expect("cannot read LIS3MDLTR");

        i2c_mux_1.select(RB_BME280_CHANNEL).expect("cannot accesss to PCA9548A");
        let bme280 = bme280::BME280::new(I2C_BUS, RB_BME280_ADDRESS);
        bme280.configure().expect("cannot configure BME280");
        let bme280_temp = bme280.read_data().expect("cannot read BME280").0;

        i2c_mux_1.reset().expect("cannot reset PCA9548A");
        i2c_mux_2.reset().expect("cannot reset PCA9548A");

        let zynq_temp_adc = read_control_reg(RB_TEMP).expect("cannot read TEMP register");
        let zynq_temp = (((zynq_temp_adc & 4095) as f32 * 503.95) / 4096.0) - 273.15;

        Self {
            drs_temp,
            clk_temp,
            adc_temp,
            lis3mdltr_temp,
            bme280_temp,
            zynq_temp,
        }
    }
    pub fn print_rb_temp() {
        let rb_temp = RBtemperature::new();
        println!("DRS Temperature:          {:.3}[°C]", rb_temp.drs_temp);
        println!("CLK Temperature:          {:.3}[°C]", rb_temp.clk_temp);
        println!("ADC Temperature:          {:.3}[°C]", rb_temp.adc_temp);
        println!("LIS3MDLTR Temperature:    {:.3}[°C]", rb_temp.lis3mdltr_temp);
        println!("BME280 Temperature:       {:.3}[°C]", rb_temp.bme280_temp);
        println!("ZYNQ Temperature:         {:.3}[°C]", rb_temp.zynq_temp);
    }
}

// vcp = voltage, current and power
pub struct RBvcp {
    drs_dvdd_voltage: f32,
    drs_dvdd_current: f32,
    drs_dvdd_power: f32,
    p3v3_voltage: f32,
    p3v3_current: f32,
    p3v3_power: f32,
    zynq_voltage: f32,
    zynq_current: f32,
    zynq_power: f32,
    p3v5_voltage: f32,
    p3v5_current: f32,
    p3v5_power: f32,
    adc_dvdd_voltage: f32,
    adc_dvdd_current: f32,
    adc_dvdd_power: f32,
    adc_avdd_voltage: f32,
    adc_avdd_current: f32,
    adc_avdd_power: f32,
    drs_avdd_voltage: f32,
    drs_avdd_current: f32,
    drs_avdd_power: f32,
    n1v5_voltage: f32,
    n1v5_current: f32,
    n1v5_power: f32,
}

impl RBvcp {
    pub fn new() -> Self {
        let i2c_mux_1 = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_1);
        let i2c_mux_2 = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);

        i2c_mux_1.select(RB_DRS_DVDD_INA226_CHANNEL).expect("cannot accesss to PCA9548A");
        let drs_dvdd_ina226 = ina226::INA226::new(I2C_BUS, RB_DRS_DVDD_INA226_ADDRESS, RB_DRS_DVDD_INA226_RSHUNT, RB_DRS_DVDD_INA226_MEC);
        drs_dvdd_ina226.configure().expect("cannot configure INA226 (DRS DVDD)");
        let (drs_dvdd_voltage, drs_dvdd_current, drs_dvdd_power) = drs_dvdd_ina226.read_data().expect("cannot read INA226 (DRS DVDD)");

        i2c_mux_1.select(RB_P3V3_INA226_CHANNEL).expect("cannot accesss to PCA9548A");
        let p3v3_ina226 = ina226::INA226::new(I2C_BUS, RB_P3V3_INA226_ADDRESS, RB_P3V3_INA226_RSHUNT, RB_P3V3_INA226_MEC);
        p3v3_ina226.configure().expect("cannot configure INA226 (P3V3)");
        let (p3v3_voltage, p3v3_current, p3v3_power) = p3v3_ina226.read_data().expect("cannot read INA226 (P3V3)");

        i2c_mux_1.select(RB_ZYNQ_INA226_CHANNEL).expect("cannot accesss to PCA9548A");
        let zynq_ina226 = ina226::INA226::new(I2C_BUS, RB_ZYNQ_INA226_ADDRESS, RB_ZYNQ_INA226_RSHUNT, RB_ZYNQ_INA226_MEC);
        zynq_ina226.configure().expect("cannot configure INA226 (ZYNQ)");
        let (zynq_voltage, zynq_current, zynq_power) = zynq_ina226.read_data().expect("cannot read INA226 (ZYNQ)");

        i2c_mux_1.select(RB_P3V5_INA226_CHANNEL).expect("cannot accesss to PCA9548A");
        let p3v5_ina226 = ina226::INA226::new(I2C_BUS, RB_P3V5_INA226_ADDRESS, RB_P3V5_INA226_RSHUNT, RB_P3V5_INA226_MEC);
        p3v5_ina226.configure().expect("cannot configure INA226 (P3V5)");
        let (p3v5_voltage, p3v5_current, p3v5_power) = p3v5_ina226.read_data().expect("cannot read INA226 (P3V5)");

        i2c_mux_2.select(RB_ADC_DVDD_INA226_CHANNEL).expect("cannot accesss to PCA9548A");
        let adc_dvdd_ina226 = ina226::INA226::new(I2C_BUS, RB_ADC_DVDD_INA226_ADDRESS, RB_ADC_DVDD_INA226_RSHUNT, RB_ADC_DVDD_INA226_MEC);
        adc_dvdd_ina226.configure().expect("cannot configure INA226 (ADC DVDD)");
        let (adc_dvdd_voltage, adc_dvdd_current, adc_dvdd_power) = adc_dvdd_ina226.read_data().expect("cannot read INA226 (ADC DVDD)");

        i2c_mux_2.select(RB_ADC_AVDD_INA226_CHANNEL).expect("cannot accesss to PCA9548A");
        let adc_avdd_ina226 = ina226::INA226::new(I2C_BUS, RB_ADC_AVDD_INA226_ADDRESS, RB_ADC_AVDD_INA226_RSHUNT, RB_ADC_AVDD_INA226_MEC);
        adc_avdd_ina226.configure().expect("cannot configure INA226 (ADC AVDD)");
        let (adc_avdd_voltage, adc_avdd_current, adc_avdd_power) = adc_avdd_ina226.read_data().expect("cannot read INA226 (ADC AVDD)");

        i2c_mux_2.select(RB_DRS_AVDD_INA226_CHANNEL).expect("cannot accesss to PCA9548A");
        let drs_avdd_ina226 = ina226::INA226::new(I2C_BUS, RB_DRS_AVDD_INA226_ADDRESS, RB_DRS_AVDD_INA226_RSHUNT, RB_DRS_AVDD_INA226_MEC);
        drs_avdd_ina226.configure().expect("cannot configure INA226 (DRS AVDD)");
        let (drs_avdd_voltage, drs_avdd_current, drs_avdd_power) = drs_avdd_ina226.read_data().expect("cannot read INA226 (DRS AVDD)");

        i2c_mux_1.select(RB_MAX11645_CHANNEL).expect("cannot accesss to PCA9548A");
        let max11645 = max11645::MAX11645::new(I2C_BUS, RB_MAX11645_ADDRESS);
        max11645.setup().expect("cannot configure MAX11645");
        let n1v5_voltage = max11645.read(RB_N1V5_VOLTAGE_INA200_CHANNEL).expect("cannot read INA200 (N1V5 VOLTAGE)") * -1.0;
        let n1v5_current = max11645.read(RB_N1V5_CURRENT_INA200_CHANNEL).expect("cannot read INA200 (N1V5 CURRENT)") / 20.0 / 0.039;
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
        println!("DRS4 Digital Rail Voltage:    {:.3}[V]", rb_vcp.drs_dvdd_voltage);
        println!("DRS4 Digital Rail Current:    {:.3}[A]", rb_vcp.drs_dvdd_current);
        println!("DRS4 Digital Rail Power:      {:.3}[W]", rb_vcp.drs_dvdd_power);
        println!("3.3V Rail Voltage:            {:.3}[V]", rb_vcp.p3v3_voltage);
        println!("3.3V Rail Current:            {:.3}[A]", rb_vcp.p3v3_current);
        println!("3.3V Rail Power:              {:.3}[W]", rb_vcp.p3v3_power);
        println!("ZYNQ Rail Voltage:            {:.3}[V]", rb_vcp.zynq_voltage);
        println!("ZYNQ Rail Current:            {:.3}[A]", rb_vcp.zynq_current);
        println!("ZYNQ Rail Power:              {:.3}[W]", rb_vcp.zynq_power);
        println!("3.5V Rail Voltage:            {:.3}[V]", rb_vcp.p3v5_voltage);
        println!("3.5V Rail Current:            {:.3}[A]", rb_vcp.p3v5_current);
        println!("3.5V Rail Power:              {:.3}[W]", rb_vcp.p3v5_power);
        println!("ADC Digital Rail Voltage:     {:.3}[V]", rb_vcp.adc_dvdd_voltage);
        println!("ADC Digital Rail Current:     {:.3}[A]", rb_vcp.adc_dvdd_current);
        println!("ADC Digital Rail Power:       {:.3}[W]", rb_vcp.adc_dvdd_power);
        println!("ADC Analog Rail Voltage:      {:.3}[V]", rb_vcp.adc_avdd_voltage);
        println!("ADC Analog Rail Current:      {:.3}[A]", rb_vcp.adc_avdd_current);
        println!("ADC Analog Rail Power:        {:.3}[W]", rb_vcp.adc_avdd_power);
        println!("DRS4 Analog Rail Voltage:     {:.3}[V]", rb_vcp.drs_avdd_voltage);
        println!("DRS4 Analog Rail Current:     {:.3}[A]", rb_vcp.drs_avdd_current);
        println!("DRS4 Analog Rail Power:       {:.3}[W]", rb_vcp.drs_avdd_power);
        println!("-1.5V Rail Voltage:          {:.3}[V]", rb_vcp.n1v5_voltage);
        println!("-1.5V Rail Current:           {:.3}[A]", rb_vcp.n1v5_current);
        println!("-1.5V Rail Power:             {:.3}[W]", rb_vcp.n1v5_power);
    }
}

pub struct RBpressure {
    pressure: f32,
}

impl RBpressure {
    pub fn new() -> Self {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_1);

        i2c_mux.select(RB_BME280_CHANNEL).expect("cannot accesss to PCA9548A");
        let bme280 = bme280::BME280::new(I2C_BUS, RB_BME280_ADDRESS);
        bme280.configure().expect("cannot configure BME280");
        let pressure = bme280.read_data().expect("cannot read BME280").1;

        i2c_mux.reset().expect("cannot reset PCA9548A");

        Self {
            pressure,
        }
    }
    pub fn print_rb_press() {
        let rb_press = RBpressure::new();
        println!("DRS Pressure:          {:.3}[hPa]", rb_press.pressure);
    }
}

pub struct RBhumidity {
    humidity: f32,
}

impl RBhumidity {
    pub fn new() -> Self {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_1);

        i2c_mux.select(RB_BME280_CHANNEL).expect("cannot accesss to PCA9548A");
        let bme280 = bme280::BME280::new(I2C_BUS, RB_BME280_ADDRESS);
        bme280.configure().expect("cannot configure BME280");
        let humidity = bme280.read_data().expect("cannot read BME280").2;

        i2c_mux.reset().expect("cannot reset PCA9548A");

        Self {
            humidity,
        }
    }
    pub fn print_rb_hum() {
        let rb_hum = RBhumidity::new();
        println!("DRS Humidity:          {:.3}[%]", rb_hum.humidity);
    }
}

pub struct RBmagnetic {
    magnetic_x: f32,
    magnetic_y: f32,
    magnetic_z: f32,
    magnetic_t: f32,
}

impl RBmagnetic {
    pub fn new() -> Self {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_1);
        i2c_mux.select(RB_LIS3MDLTR_CHANNEL).expect("cannot accesss to PCA9548A");

        let lis3mdltr = lis3mdltr::LIS3MDLTR::new(I2C_BUS, RB_LIS3MDLTR_ADDRESS);
        lis3mdltr.configure();
        let magnetic_field = lis3mdltr.read_magnetic_field().expect("cannot read LIS3MDLTR");

        i2c_mux.reset().expect("cannot reset PCA9548A");

        Self {
            magnetic_x: magnetic_field[0],
            magnetic_y: magnetic_field[1],
            magnetic_z: magnetic_field[2],
            magnetic_t: magnetic_field[3],
        }
    }
    pub fn print_rb_magnetic() {
        let rb_magnetic = RBmagnetic::new();
        println!("Magnetic X:  {:.3}[G]", rb_magnetic.magnetic_x);
        println!("Magnetic Y:  {:.3}[G]", rb_magnetic.magnetic_y);
        println!("Magnetic Z:  {:.3}[G]", rb_magnetic.magnetic_z);
        println!("Magnetic T:  {:.3}[G]", rb_magnetic.magnetic_t);
    }
}

pub struct RBclk {
    lock_status: bool,
    mode_of_operation: bool,
}

impl RBclk {
    pub fn new() -> Self {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
        i2c_mux.select(RB_SI5345B_CHANNEL).expect("cannot accesss to PCA9548A");

        let si5345b = si5345b::SI5345B::new(I2C_BUS, RB_SI5345B_ADDRESS);

        let lol_status = si5345b.read_lol_status().expect("cannot read LOL status from SI5345B");
        let lock_status = if lol_status {false} else {true};

        let holdover_status = si5345b.read_holdover_status().expect("cannot read HOLD status from SI5345B");
        let mode_of_operation = if holdover_status {false} else {true};

        i2c_mux.reset().expect("cannot reset PCA9548A");

        Self {
            lock_status,
            mode_of_operation,
        }
    }
    pub fn print_rb_clk() {
        let rb_clk = RBclk::new();
        println!("DSPLL Status:         {}", if rb_clk.lock_status {String::from("Locked")} else {String::from("Unlocked")});
        println!("Mode of Operation:    {}", if rb_clk.mode_of_operation {String::from("Locked Mode")} else {String::from("Holdover Mode (or Freerun Mode)")});
    }
    pub fn print_config() {
        let si5345b = si5345b::SI5345B::new(I2C_BUS, RB_SI5345B_ADDRESS);
        si5345b.configure_si5345b().expect("cannot configure SI5345B");
    }
}

pub struct RBgpioe {
    device_family: u8,
    device_setting: u8,
    port_status: Vec<u8>,
}

impl RBgpioe {
    pub fn new() -> Self {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
        i2c_mux.select(RB_CY8C9560A_CHANNEL).expect("cannot accesss to PCA9548A");

        let cy8c9560a = cy8c9560a::CY8C9560A::new(I2C_BUS, RB_CY8C9560A_ADDRESS);

        let (device_family, device_setting) = cy8c9560a.read_device_info().expect("cannot read CY8C9560A");

        let mut port_status = Vec::new();
        for i in 0..8 {
            port_status.push(cy8c9560a.read_port_status(i).expect("cannot read CY8C9560A"));
        }

        i2c_mux.reset().expect("cannot reset PCA9548A");

        Self {
            device_family,
            device_setting,
            port_status,
        }
    }
    pub fn print_rb_gpioe() {
        let rb_gpioe = RBgpioe::new();
        println!("Device Family:    {}", rb_gpioe.device_family);
        println!("Device Setting:   {}", rb_gpioe.device_setting);
        println!("Port 0 Status:    {:#02X}", rb_gpioe.port_status[0]);
        println!("Port 1 Status:    {:#02X}", rb_gpioe.port_status[1]);
        println!("Port 2 Status:    {:#02X}", rb_gpioe.port_status[2]);
        println!("Port 3 Status:    {:#02X}", rb_gpioe.port_status[3]);
        println!("Port 4 Status:    {:#02X}", rb_gpioe.port_status[4]);
        println!("Port 5 Status:    {:#02X}", rb_gpioe.port_status[5]);
        println!("Port 6 Status:    {:#02X}", rb_gpioe.port_status[6]);
        println!("Port 7 Status:    {:#02X}", rb_gpioe.port_status[7]);
    }
}

pub struct RBdac {
    dac0: u16,
}

impl RBdac {
    pub fn new() -> Self {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
        i2c_mux.select(RB_AD5675_CHANNEL).expect("cannot accesss to PCA9548A");

        let ad5675 = ad5675::AD5675::new(I2C_BUS, RB_AD5675_ADDRESS);

        let dac0 = ad5675.read_dac(0).expect("cannot read AD5675");

        i2c_mux.reset().expect("cannot reset PCA9548A");

        Self {
            dac0,
        }
    }
    pub fn print_rb_dac() {
        let rb_dac = RBdac::new();
        println!("DAC0:     {}", rb_dac.dac0);
    }
}