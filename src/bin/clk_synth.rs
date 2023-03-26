use std::thread;
use std::time::Duration;
use csv;
// use i2cdev::core::*;
// use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

// use tof_control::constant::*;
// use tof_control::device::{pca9548a, si5345b};

fn main() {
    let si5345b_csv = include_str!("../config/rb_config/si5345b.csv");
    let mut reader = csv::ReaderBuilder::new()
        .comment(Some(b'#'))
        .escape(Some(b'\\'))
        .flexible(true)
        .from_reader(si5345b_csv.as_bytes());

    for (i, record) in reader.records().enumerate() {
        let record = record.expect("failed to convert record");
        let address = i64::from_str_radix(&record[0].trim_start_matches("0x"), 16).expect("cannot convert register from address");
        let data = i64::from_str_radix(&record[1].trim_start_matches("0x"), 16).expect("cannot convert register from data");
        let page = address >> 8;
        let register = address & 0xFF;
        println!("index: {}, page: {:#02X}, reg: {:#02X}, value: {:#02X}", i, page, register, data);
        
        // dev.smbus_write_byte_data(SET_PAGE as u8, page as u8);
        // dev.smbus_write_byte_data(register as u8, data as u8);

        if i == 2 {
            thread::sleep(Duration::from_millis(300));
            println!("preamble done");
        }

    }
    
    println!("Done!");
}