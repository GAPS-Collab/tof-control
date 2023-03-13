use crate::constant::*;
use crate::device::{pca9548a, cy8c9560a};

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

        // i2c_mux.reset().expect("cannot reset PCA9548A");

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
    pub fn initialize() {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
        i2c_mux.select(RB_CY8C9560A_CHANNEL).expect("cannot accesss to PCA9548A");

        let cy8c9560a = cy8c9560a::CY8C9560A::new(I2C_BUS, RB_CY8C9560A_ADDRESS);
        
        // Port 0
        cy8c9560a.select_port(0).expect("cannot select port 0 on CY8C9560A");
        cy8c9560a.set_interrupt_mask_port(0x00).expect("cannot set interrupt mask for port 0 on CY8C9560A");
        cy8c9560a.set_pin_direction(0x00).expect("cannot set pin direction for port 0 on CY8C9560A");
        cy8c9560a.set_drive_mode(4).expect("cannot set drive mode for port 0 on CY8C9560A");

        // Port 1
        cy8c9560a.select_port(1).expect("cannot select port 1 on CY8C9560A");
        cy8c9560a.set_interrupt_mask_port(0x00).expect("cannot set interrupt mask for port 1 on CY8C9560A");
        cy8c9560a.set_pin_direction(0x00).expect("cannot set pin direction for port 1 on CY8C9560A");
        cy8c9560a.set_drive_mode(1).expect("cannot set drive mode for port 1 on CY8C9560A");

        // Port 2
        cy8c9560a.select_port(2).expect("cannot select port 2 on CY8C9560A");
        cy8c9560a.set_interrupt_mask_port(0x00).expect("cannot set interrupt mask for port 2 on CY8C9560A");
        cy8c9560a.set_pin_direction(0x00).expect("cannot set pin direction for port 2 on CY8C9560A");
        cy8c9560a.set_drive_mode(4).expect("cannot set drive mode for port 2 on CY8C9560A");

        // Port 3
        cy8c9560a.select_port(3).expect("cannot select port 3 on CY8C9560A");
        cy8c9560a.set_interrupt_mask_port(0x00).expect("cannot set interrupt mask for port 3 on CY8C9560A");
        cy8c9560a.set_pin_direction(0x00).expect("cannot set pin direction for port 3 on CY8C9560A");
        cy8c9560a.set_drive_mode(4).expect("cannot set drive mode for port 3 on CY8C9560A");

        // Port 4
        cy8c9560a.select_port(4).expect("cannot select port 4 on CY8C9560A");
        cy8c9560a.set_interrupt_mask_port(0x00).expect("cannot set interrupt mask for port 4 on CY8C9560A");
        cy8c9560a.set_pin_direction(0x00).expect("cannot set pin direction for port 4 on CY8C9560A");
        cy8c9560a.set_drive_mode(4).expect("cannot set drive mode for port 4 on CY8C9560A");

        // Port 5
        cy8c9560a.select_port(5).expect("cannot select port 5 on CY8C9560A");
        cy8c9560a.set_interrupt_mask_port(0x00).expect("cannot set interrupt mask for port 5 on CY8C9560A");
        cy8c9560a.set_pin_direction(0x00).expect("cannot set pin direction for port 5 on CY8C9560A");
        cy8c9560a.set_drive_mode(4).expect("cannot set drive mode for port 5 on CY8C9560A");

        // Port 6
        cy8c9560a.select_port(6).expect("cannot select port 6 on CY8C9560A");
        cy8c9560a.set_interrupt_mask_port(0x00).expect("cannot set interrupt mask for port 6 on CY8C9560A");
        cy8c9560a.set_pin_direction(0x00).expect("cannot set pin direction for port 6 on CY8C9560A");
        cy8c9560a.set_drive_mode(1).expect("cannot set drive mode for port 6 on CY8C9560A");

        // Port 7
        cy8c9560a.select_port(7).expect("cannot select port 7 on CY8C9560A");
        cy8c9560a.set_interrupt_mask_port(0x00).expect("cannot set interrupt mask for port 7 on CY8C9560A");
        cy8c9560a.set_pin_direction(0x00).expect("cannot set pin direction for port 7 on CY8C9560A");
        cy8c9560a.set_drive_mode(4).expect("cannot set drive mode for port 7 on CY8C9560A");

        // Set ouput ports
        cy8c9560a.set_output_port(0, 0x00).expect("cannot set ouput for port 0 on CY8C9560A");
        cy8c9560a.set_output_port(1, 0xFF).expect("cannot set ouput for port 1 on CY8C9560A");
        cy8c9560a.set_output_port(2, 0x03).expect("cannot set ouput for port 2 on CY8C9560A");
        cy8c9560a.set_output_port(3, 0x03).expect("cannot set ouput for port 3 on CY8C9560A");
        cy8c9560a.set_output_port(4, 0xFC).expect("cannot set ouput for port 4 on CY8C9560A");
        cy8c9560a.set_output_port(5, 0x33).expect("cannot set ouput for port 5 on CY8C9560A");
        cy8c9560a.set_output_port(6, 0xFF).expect("cannot set ouput for port 6 on CY8C9560A");
        cy8c9560a.set_output_port(7, 0x3F).expect("cannot set ouput for port 7 on CY8C9560A");

        i2c_mux.reset().expect("cannot reset PCA9548A");
    }
    pub fn set_rf_switch(mode: u8) {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
        i2c_mux.select(RB_CY8C9560A_CHANNEL).expect("cannot accesss to PCA9548A");

        let cy8c9560a = cy8c9560a::CY8C9560A::new(I2C_BUS, RB_CY8C9560A_ADDRESS);

        cy8c9560a.set_rf_switch(0, mode).expect("cannot set mode for RF switch on CY8C9560A");
        cy8c9560a.set_rf_switch(1, mode).expect("cannot set mode for RF switch on CY8C9560A");
        cy8c9560a.set_rf_switch(2, mode).expect("cannot set mode for RF switch on CY8C9560A");
        cy8c9560a.set_rf_switch(3, mode).expect("cannot set mode for RF switch on CY8C9560A");
        cy8c9560a.set_rf_switch(4, mode).expect("cannot set mode for RF switch on CY8C9560A");
        cy8c9560a.set_rf_switch(5, mode).expect("cannot set mode for RF switch on CY8C9560A");
        cy8c9560a.set_rf_switch(6, mode).expect("cannot set mode for RF switch on CY8C9560A");
        cy8c9560a.set_rf_switch(7, mode).expect("cannot set mode for RF switch on CY8C9560A");
        if mode == 0 {
            cy8c9560a.set_rf_switch(8, 0).expect("cannot set mode for RF switch on CY8C9560A");
        } else {
            cy8c9560a.set_rf_switch(8, 2).expect("cannot set mode for RF switch on CY8C9560A");
        }

        i2c_mux.reset().expect("cannot reset PCA9548A");
    }
    pub fn enable_tcal_clock(mode: u8) {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
        i2c_mux.select(RB_CY8C9560A_CHANNEL).expect("cannot accesss to PCA9548A");

        let cy8c9560a = cy8c9560a::CY8C9560A::new(I2C_BUS, RB_CY8C9560A_ADDRESS);

        if mode == 1 {
            cy8c9560a.enable_tcal_clock().expect("cannot enable timing-calibration clock on CY8C9560A");
        }
    }
    pub fn disable_tcal_clock() {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
        i2c_mux.select(RB_CY8C9560A_CHANNEL).expect("cannot accesss to PCA9548A");

        let cy8c9560a = cy8c9560a::CY8C9560A::new(I2C_BUS, RB_CY8C9560A_ADDRESS);
        cy8c9560a.disable_tcal_clock().expect("cannot disable timing-calibration clock on CY8C9560A");

    }
    pub fn dac_reset() {
        let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
        i2c_mux.select(RB_CY8C9560A_CHANNEL).expect("cannot accesss to PCA9548A");

        let cy8c9560a = cy8c9560a::CY8C9560A::new(I2C_BUS, RB_CY8C9560A_ADDRESS);

        let mut value = cy8c9560a.read_port_status(3).expect("cannot read CY8C9560A");
        value = (value & !0x10) | 0x10;
        
        cy8c9560a.set_output_port(3, value).expect("cannot set ouput for port 3 on CY8C9560A");
        println!("DAC Reset");
    }
}