#![allow(unused)]
use std::thread;
use std::time::Duration;

use crate::constant::*;
use crate::device::cy8c9560a::CY8C9560A;
use crate::device::{pca9548a, cy8c9560a};

/*
Ports used for Readout Board V2.5.2
cy8c9560A
//////////////////////////////////////////
//Output
//////////////////////////////////////////

GP0[7]
GP0[7]: Si5345_FINC

GP1[]

GP2[1:0]
GP2[1]: HMC849_EN3
GP2[0]: HMC849_VCTL3

GP3[5:4,2:0]
GP3[5]: MARS_WDI_GE
GP3[4]: ~VCAL_RST
GP3[2]: Si5345_FDEC
GP3[1]: ~Si5345_OE
GP3[0]: ~Si5345_RST

GP4[7:2]
GP4[7]: HMC849_VCTL6
GP4[6]: HMC849_EN6
GP4[5]: HMC849_VCTL7
GP4[4]: HMC849_EN7
GP4[3]: HMC849_VCTL8
GP4[2]: HMC849_EN8

GP5[5:4,1:0]
GP5[5]: HMC849_EN5
GP5[4]: HMC849_VCTL5
GP5[1]: HMC849_VCTL4
GP5[0]: HMC849_EN4

GP6[]

GP7[7:0]
GP7[7]: TCA_CLK_SC_EN
GP7[6]: TCA_CLK_OUT_EN
GP7[5]: HMC849_EN0
GP7[4]: HMC849_VCTL0
GP7[3]: HMC849_EN1
GP7[2]: HMC849_VCTL1
GP7[1]: HMC849_EN2
GP7[0]: HMC849_VCTL2

Initialization Value:
GP0: 0x00
GP1: 0xFF
GP2: 0x03
GP3: 0x13
GP4: 0xFC
GP5: 0x33
GP6: 0xFF
GP7: 0x3F

*/

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
    // pub fn initialize() {
    //     let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
    //     i2c_mux.select(RB_CY8C9560A_CHANNEL).expect("cannot accesss to PCA9548A");

    //     let cy8c9560a = cy8c9560a::CY8C9560A::new(I2C_BUS, RB_CY8C9560A_ADDRESS);
        
    //     // Port 0
    //     cy8c9560a.select_port(0).expect("cannot select port 0 on CY8C9560A");
    //     cy8c9560a.set_interrupt_mask_port(0x00).expect("cannot set interrupt mask for port 0 on CY8C9560A");
    //     cy8c9560a.set_pin_direction(0x00).expect("cannot set pin direction for port 0 on CY8C9560A");
    //     cy8c9560a.set_drive_mode(4).expect("cannot set drive mode for port 0 on CY8C9560A");

    //     // Port 1
    //     cy8c9560a.select_port(1).expect("cannot select port 1 on CY8C9560A");
    //     cy8c9560a.set_interrupt_mask_port(0x00).expect("cannot set interrupt mask for port 1 on CY8C9560A");
    //     cy8c9560a.set_pin_direction(0x00).expect("cannot set pin direction for port 1 on CY8C9560A");
    //     cy8c9560a.set_drive_mode(1).expect("cannot set drive mode for port 1 on CY8C9560A");

    //     // Port 2
    //     cy8c9560a.select_port(2).expect("cannot select port 2 on CY8C9560A");
    //     cy8c9560a.set_interrupt_mask_port(0x00).expect("cannot set interrupt mask for port 2 on CY8C9560A");
    //     cy8c9560a.set_pin_direction(0x00).expect("cannot set pin direction for port 2 on CY8C9560A");
    //     cy8c9560a.set_drive_mode(4).expect("cannot set drive mode for port 2 on CY8C9560A");

    //     // Port 3
    //     cy8c9560a.select_port(3).expect("cannot select port 3 on CY8C9560A");
    //     cy8c9560a.set_interrupt_mask_port(0x00).expect("cannot set interrupt mask for port 3 on CY8C9560A");
    //     cy8c9560a.set_pin_direction(0x00).expect("cannot set pin direction for port 3 on CY8C9560A");
    //     cy8c9560a.set_drive_mode(4).expect("cannot set drive mode for port 3 on CY8C9560A");

    //     // Port 4
    //     cy8c9560a.select_port(4).expect("cannot select port 4 on CY8C9560A");
    //     cy8c9560a.set_interrupt_mask_port(0x00).expect("cannot set interrupt mask for port 4 on CY8C9560A");
    //     cy8c9560a.set_pin_direction(0x00).expect("cannot set pin direction for port 4 on CY8C9560A");
    //     cy8c9560a.set_drive_mode(4).expect("cannot set drive mode for port 4 on CY8C9560A");

    //     // Port 5
    //     cy8c9560a.select_port(5).expect("cannot select port 5 on CY8C9560A");
    //     cy8c9560a.set_interrupt_mask_port(0x00).expect("cannot set interrupt mask for port 5 on CY8C9560A");
    //     cy8c9560a.set_pin_direction(0x00).expect("cannot set pin direction for port 5 on CY8C9560A");
    //     cy8c9560a.set_drive_mode(4).expect("cannot set drive mode for port 5 on CY8C9560A");

    //     // Port 6
    //     cy8c9560a.select_port(6).expect("cannot select port 6 on CY8C9560A");
    //     cy8c9560a.set_interrupt_mask_port(0x00).expect("cannot set interrupt mask for port 6 on CY8C9560A");
    //     cy8c9560a.set_pin_direction(0x00).expect("cannot set pin direction for port 6 on CY8C9560A");
    //     cy8c9560a.set_drive_mode(1).expect("cannot set drive mode for port 6 on CY8C9560A");

    //     // Port 7
    //     cy8c9560a.select_port(7).expect("cannot select port 7 on CY8C9560A");
    //     cy8c9560a.set_interrupt_mask_port(0x00).expect("cannot set interrupt mask for port 7 on CY8C9560A");
    //     cy8c9560a.set_pin_direction(0x00).expect("cannot set pin direction for port 7 on CY8C9560A");
    //     cy8c9560a.set_drive_mode(4).expect("cannot set drive mode for port 7 on CY8C9560A");

    //     // Set ouput ports
    //     cy8c9560a.set_output_port(0, 0x00).expect("cannot set ouput for port 0 on CY8C9560A");
    //     cy8c9560a.set_output_port(1, 0xFF).expect("cannot set ouput for port 1 on CY8C9560A");
    //     cy8c9560a.set_output_port(2, 0x03).expect("cannot set ouput for port 2 on CY8C9560A");
    //     cy8c9560a.set_output_port(3, 0x03).expect("cannot set ouput for port 3 on CY8C9560A");
    //     cy8c9560a.set_output_port(4, 0xFC).expect("cannot set ouput for port 4 on CY8C9560A");
    //     cy8c9560a.set_output_port(5, 0x33).expect("cannot set ouput for port 5 on CY8C9560A");
    //     cy8c9560a.set_output_port(6, 0xFF).expect("cannot set ouput for port 6 on CY8C9560A");
    //     cy8c9560a.set_output_port(7, 0x3F).expect("cannot set ouput for port 7 on CY8C9560A");

    //     i2c_mux.reset().expect("cannot reset PCA9548A");
    // }
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

pub fn read_port() {
    let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
    i2c_mux.select(RB_CY8C9560A_CHANNEL).expect("cannot accesss to PCA9548A");

    let cy8c9560a = cy8c9560a::CY8C9560A::new(I2C_BUS, RB_CY8C9560A_ADDRESS);
    let mut gp0 = cy8c9560a.read_port_status(0).expect("cannot read CY8C9560A");
    let mut gp1 = cy8c9560a.read_port_status(1).expect("cannot read CY8C9560A");
    let mut gp2 = cy8c9560a.read_port_status(2).expect("cannot read CY8C9560A");
    let mut gp3 = cy8c9560a.read_port_status(3).expect("cannot read CY8C9560A");
    let mut gp4 = cy8c9560a.read_port_status(4).expect("cannot read CY8C9560A");
    let mut gp5 = cy8c9560a.read_port_status(5).expect("cannot read CY8C9560A");
    let mut gp6 = cy8c9560a.read_port_status(6).expect("cannot read CY8C9560A");
    let mut gp7 = cy8c9560a.read_port_status(7).expect("cannot read CY8C9560A");
    println!("GP0: {:#02X}", gp0);
    println!("GP1: {:#02X}", gp1);
    println!("GP2: {:#02X}", gp2);
    println!("GP3: {:#02X}", gp3);
    println!("GP4: {:#02X}", gp4);
    println!("GP5: {:#02X}", gp5);
    println!("GP6: {:#02X}", gp6);
    println!("GP7: {:#02X}", gp7);
    
    i2c_mux.reset().expect("cannot reset PCA9548A");
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
    cy8c9560a.set_output_port(3, 0x13).expect("cannot set ouput for port 3 on CY8C9560A");
    cy8c9560a.set_output_port(4, 0xFC).expect("cannot set ouput for port 4 on CY8C9560A");
    cy8c9560a.set_output_port(5, 0x33).expect("cannot set ouput for port 5 on CY8C9560A");
    cy8c9560a.set_output_port(6, 0xFF).expect("cannot set ouput for port 6 on CY8C9560A");
    cy8c9560a.set_output_port(7, 0x3F).expect("cannot set ouput for port 7 on CY8C9560A");

    i2c_mux.reset().expect("cannot reset PCA9548A");
}

pub fn reset_si5345b() {
    let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
    i2c_mux.select(RB_CY8C9560A_CHANNEL).expect("cannot accesss to PCA9548A");

    let cy8c9560a = cy8c9560a::CY8C9560A::new(I2C_BUS, RB_CY8C9560A_ADDRESS);

    let mut value = cy8c9560a.read_port_status(3).expect("cannot read CY8C9560A");
    value = value ^ 0x01;
    cy8c9560a.set_output_port(3, value).expect("cannot set ouput for port 3 on CY8C9560A");

    value = cy8c9560a.read_port_status(3).expect("cannot read CY8C9560A");
    value = value | 0x01;
    cy8c9560a.set_output_port(3, value).expect("cannot set ouput for port 3 on CY8C9560A");

    i2c_mux.reset().expect("cannot reset PCA9548A");

    thread::sleep(Duration::from_millis(2000));
}

pub fn enable_si5345b() {
    let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
    i2c_mux.select(RB_CY8C9560A_CHANNEL).expect("cannot accesss to PCA9548A");

    let cy8c9560a = cy8c9560a::CY8C9560A::new(I2C_BUS, RB_CY8C9560A_ADDRESS);

    let mut value = cy8c9560a.read_port_status(3).expect("cannot read CY8C9560A");
    value = (value & !0x02) | 0 << 1;
    cy8c9560a.set_output_port(3, value).expect("cannot set ouput for port 3 on CY8C9560A");


    i2c_mux.reset().expect("cannot reset PCA9548A");
}

pub fn enable_ad5675() {
    let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
    i2c_mux.select(RB_CY8C9560A_CHANNEL).expect("cannot accesss to PCA9548A");

    let cy8c9560a = cy8c9560a::CY8C9560A::new(I2C_BUS, RB_CY8C9560A_ADDRESS);
    let mut value = cy8c9560a.read_port_status(3).expect("cannot read CY8C9560A");
    value = (value & !0x10) | 1 << 4;

    cy8c9560a.set_output_port(3, value).expect("cannot set ouput for port 3 on CY8C9560A");

    i2c_mux.reset().expect("cannot reset PCA9548A");
}

// GP7[7]: TCA_CLK_SC_EN
// GP7[6]: TCA_CLK_OUT_EN
pub fn enable_nb3v9312c() {
    let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
    i2c_mux.select(RB_CY8C9560A_CHANNEL).expect("cannot accesss to PCA9548A");

    let cy8c9560a = cy8c9560a::CY8C9560A::new(I2C_BUS, RB_CY8C9560A_ADDRESS);
    let mut value = cy8c9560a.read_port_status(7).expect("cannot read CY8C9560A");
    // value |= (value | 0x80) | 1 << 7;
    // cy8c9560a.set_output_port(7, value).expect("cannot set ouput for port 7 on CY8C9560A");
    // value = cy8c9560a.read_port_status(7).expect("cannot read CY8C9560A");
    // value &= (value | 0x40) | 1 << 6;
    // cy8c9560a.set_output_port(7, value).expect("cannot set ouput for port 7 on CY8C9560A");
    // println!("{:#02X}", value);
    value = value | 0xC0;
    cy8c9560a.set_output_port(7, value).expect("cannot set ouput for port 7 on CY8C9560A");
    
    i2c_mux.reset().expect("cannot reset PCA9548A");
}
pub fn disable_nb3v9312c() {
    let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
    i2c_mux.select(RB_CY8C9560A_CHANNEL).expect("cannot accesss to PCA9548A");

    let cy8c9560a = cy8c9560a::CY8C9560A::new(I2C_BUS, RB_CY8C9560A_ADDRESS);
    let mut value = cy8c9560a.read_port_status(7).expect("cannot read CY8C9560A");
    // value |= (value & !0x80) | 0 << 7;
    // cy8c9560a.set_output_port(7, value).expect("cannot set ouput for port 7 on CY8C9560A");
    // value = cy8c9560a.read_port_status(7).expect("cannot read CY8C9560A");
    // value &= (value & !0x40) | 0 << 6;cy8c9560a.set_output_port(7, value).expect("cannot set ouput for port 7 on CY8C9560A");
    // cy8c9560a.set_output_port(7, value).expect("cannot set ouput for port 7 on CY8C9560A");
    // println!("{:#02X}", value);
    value = value & 0x3F;
    cy8c9560a.set_output_port(7, value).expect("cannot set ouput for port 7 on CY8C9560A");
    
    i2c_mux.reset().expect("cannot reset PCA9548A");
}

/*
  HMC849 Truth Table:
 |_VCTL__|__EN__|  |_RFC -> RF1_|_RFC -> RF2_|
    0    |   0           OFF    |     ON
    1    |   0           ON           OFF
    0    |   1           OFF          OFF
    1    |   1           OFF          OFF
    
    hmcChannel: 0 - 8
    mode: 0: RFC = OFF  (No Connection)
          1: RFC -> RF1 (TCA Calibration Input)
          2: RFC -> RF2 (SMA Input)

*/

pub fn rf_input_select(mode: u8) {
    let i2c_mux = pca9548a::PCA9548A::new(I2C_BUS, RB_PCA9548A_ADDRESS_2);
    i2c_mux.select(RB_CY8C9560A_CHANNEL).expect("cannot accesss to PCA9548A");

    let cy8c9560a = cy8c9560a::CY8C9560A::new(I2C_BUS, RB_CY8C9560A_ADDRESS);

    ch1_input_select(cy8c9560a, mode);
    ch2_input_select(cy8c9560a, mode);
    ch3_input_select(cy8c9560a, mode);
    ch4_input_select(cy8c9560a, mode);
    ch5_input_select(cy8c9560a, mode);
    ch6_input_select(cy8c9560a, mode);
    ch7_input_select(cy8c9560a, mode);
    ch8_input_select(cy8c9560a, mode);
    ch9_input_select(cy8c9560a, mode);

    // // channel 9 should 0 for voltage cal, 2 otherwise
    // if mode == 0 {
    //     ch9_input_select(cy8c9560a, 0);
    // } else {
    //     ch9_input_select(cy8c9560a, 2);
    // }

}

// GP7[5] = EN
// GP7[4] = VCTL
fn ch1_input_select(gpioe: CY8C9560A, mode: u8) {
    let port_status = gpioe.read_port_status(7).expect("cannot read CY8C9560A");

    match mode {
        0 => {
            // let value = (port_status & !0x30) | 0x20;
            let value = port_status | 0x30;
            gpioe.set_output_port(7, value).expect("cannot write CY8C9560A");
        },
        1 => {
            let value = (port_status & !0x30) | 0x10;
            gpioe.set_output_port(7, value).expect("cannot write CY8C9560A");
        },
        2 => {
            let value = (port_status & !0x30) | 0x00;
            gpioe.set_output_port(7, value).expect("cannot write CY8C9560A");
        },
        _ => {
            gpioe.set_output_port(7, port_status).expect("cannot write CY8C9560A");
        }
    }
}

// GP7[3] = EN
// GP7[2] = VCTL     
fn ch2_input_select(gpioe: CY8C9560A, mode: u8) {
    let port_status = gpioe.read_port_status(7).expect("cannot read CY8C9560A");

    match mode {
        0 => {
            // let value = (port_status | 0x0C) & 0x0C;
            let value = port_status | 0x0C;
            gpioe.set_output_port(7, value).expect("cannot write CY8C9560A");
        },
        1 => {
            let value = (port_status & !0x0C) | 0x04;
            gpioe.set_output_port(7, value).expect("cannot write CY8C9560A");
        },
        2 => {
            let value = (port_status & !0x0C) | 0x00;
            gpioe.set_output_port(7, value).expect("cannot write CY8C9560A");
        },
        _ => {
            gpioe.set_output_port(7, port_status).expect("cannot write CY8C9560A");
        }
    }
}

// GP7[1] = EN
// GP7[0] = VCTL
fn ch3_input_select(gpioe: CY8C9560A, mode: u8) {
    let port_status = gpioe.read_port_status(7).expect("cannot read CY8C9560A");

    match mode {
        0 => {
            // let value = (port_status | 0x03) & 0x03;
            let value = port_status | 0x03;
            gpioe.set_output_port(7, value).expect("cannot write CY8C9560A");
        },
        1 => {
            let value = (port_status & !0x03) | 0x01;
            gpioe.set_output_port(7, value).expect("cannot write CY8C9560A");
        },
        2 => {
            let value = (port_status & !0x03) | 0x00;
            gpioe.set_output_port(7, value).expect("cannot write CY8C9560A");
        },
        _ => {
            gpioe.set_output_port(7, port_status).expect("cannot write CY8C9560A");
        }
    }
}

// GP2[1] = EN
// GP2[0] = VCTL
fn ch4_input_select(gpioe: CY8C9560A, mode: u8) {
    let port_status = gpioe.read_port_status(2).expect("cannot read CY8C9560A");

    match mode {
        0 => {
            // let value = (port_status | 0x03) & 0x03;
            let value = port_status | 0x03;
            gpioe.set_output_port(2, value).expect("cannot write CY8C9560A");
        },
        1 => {
            let value = (port_status & !0x03) | 0x01;
            gpioe.set_output_port(2, value).expect("cannot write CY8C9560A");
        },
        2 => {
            let value = (port_status & !0x03) | 0x00;
            gpioe.set_output_port(2, value).expect("cannot write CY8C9560A");
        },
        _ => {
            gpioe.set_output_port(2, port_status).expect("cannot write CY8C9560A");
        }
    }
}

// GP5[1] = EN
// GP5[0] = VCTL
fn ch5_input_select(gpioe: CY8C9560A, mode: u8) {
    let port_status = gpioe.read_port_status(5).expect("cannot read CY8C9560A");

    match mode {
        0 => {
            // let value = (port_status | 0x03) & 0x03;
            let value = port_status | 0x03;
            gpioe.set_output_port(5, value).expect("cannot write CY8C9560A");
        },
        1 => {
            let value = (port_status & !0x03) | 0x02;
            gpioe.set_output_port(5, value).expect("cannot write CY8C9560A");
        },
        2 => {
            let value = (port_status & !0x03) | 0x00;
            gpioe.set_output_port(5, value).expect("cannot write CY8C9560A");
        },
        _ => {
            gpioe.set_output_port(5, port_status).expect("cannot write CY8C9560A");
        }
    }
}

// GP5[5] = EN
// GP5[4] = VCTL
fn ch6_input_select(gpioe: CY8C9560A, mode: u8) {
    let port_status = gpioe.read_port_status(5).expect("cannot read CY8C9560A");

    match mode {
        0 => {
            // let value = (port_status | 0x30) & 0x30;
            let value = port_status | 0x30;
            gpioe.set_output_port(5, value).expect("cannot write CY8C9560A");
        },
        1 => {
            let value = (port_status & !0x30) | 0x10;
            gpioe.set_output_port(5, value).expect("cannot write CY8C9560A");
        },
        2 => {
            let value = (port_status & !0x30) | 0x00;
            gpioe.set_output_port(5, value).expect("cannot write CY8C9560A");
        },
        _ => {
            gpioe.set_output_port(5, port_status).expect("cannot write CY8C9560A");
        }
    }
}

// GP4[7] = EN
// GP4[6] = VCTL
fn ch7_input_select(gpioe: CY8C9560A, mode: u8) {
    let port_status = gpioe.read_port_status(4).expect("cannot read CY8C9560A");

    match mode {
        0 => {
            // let value = (port_status | 0xC0) & 0xC0;
            let value = port_status | 0xC0;
            gpioe.set_output_port(4, value).expect("cannot write CY8C9560A");
        },
        1 => {
            let value = (port_status & !0xC0) | 0x80;
            gpioe.set_output_port(4, value).expect("cannot write CY8C9560A");
        },
        2 => {
            let value = (port_status & !0xC0) | 0x00;
            gpioe.set_output_port(4, value).expect("cannot write CY8C9560A");
        },
        _ => {
            gpioe.set_output_port(4, port_status).expect("cannot write CY8C9560A");
        }
    }
}

// GP4[5] = EN
// GP4[4] = VCTL
fn ch8_input_select(gpioe: CY8C9560A, mode: u8) {
    let port_status = gpioe.read_port_status(4).expect("cannot read CY8C9560A");

    match mode {
        0 => {
            // let value = (port_status | 0x30) & 0x30;
            let value = port_status | 0x30;
            gpioe.set_output_port(4, value).expect("cannot write CY8C9560A");
        },
        1 => {
            let value = (port_status & !0x30) | 0x20;
            gpioe.set_output_port(4, value).expect("cannot write CY8C9560A");
        },
        2 => {
            let value = (port_status & !0x30) | 0x00;
            gpioe.set_output_port(4, value).expect("cannot write CY8C9560A");
        },
        _ => {
            gpioe.set_output_port(4, port_status).expect("cannot write CY8C9560A");
        }
    }
}

// GP4[3] = EN
// GP4[2] = VCTL
fn ch9_input_select(gpioe: CY8C9560A, mode: u8) {
    let port_status = gpioe.read_port_status(4).expect("cannot read CY8C9560A");

    match mode {
        0 => {
            // let value = (port_status | 0x0C) & 0x0C;
            let value = port_status | 0x0C;
            gpioe.set_output_port(4, value).expect("cannot write CY8C9560A");
        },
        1 => {
            let value = (port_status & !0x0C) | 0x08;
            gpioe.set_output_port(4, value).expect("cannot write CY8C9560A");
        },
        2 => {
            let value = (port_status & !0x0C) | 0x00;
            gpioe.set_output_port(4, value).expect("cannot write CY8C9560A");
        },
        _ => {
            gpioe.set_output_port(4, port_status).expect("cannot write CY8C9560A");
        }
    }
}