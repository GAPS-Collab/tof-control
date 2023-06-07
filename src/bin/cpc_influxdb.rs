#![allow(unused)]
use gethostname::gethostname;
use chrono::Utc;
use futures::prelude::*;
use std::thread;
use std::time::Duration;
use influxdb2::models::DataPoint;
use influxdb2::Client;
use influxdb2_derive::WriteDataPoint;

use tof_control::constant::*;
use tof_control::device::{tmp1075, ina219, max7320};

#[tokio::main]
async fn main() {
    loop {
        cpc_influxdb_write().await;
        thread::sleep(Duration::from_secs(60));
    }
}

// #[tokio::main]
async fn cpc_influxdb_write() -> Result<(), Box<dyn std::error::Error>> {
    let hostname = gethostname().into_string().expect("cannot convert hostname");

    let org = "gaps";
    let bucket = "CPC";
    let influx_url = "http://10.97.108.31:8086";
    let token = "WXqbeoDBq0euIIK-1OyBKrJ0R4XwOq5TZaZiIEtKTlcXgqTLo8tMIZgkft3OTgTQD2LQyejRo8h3mrMWsCTfYw==";

    let client = influxdb2::Client::new(influx_url, org, token);

    let cpc_temp = cpc_temperature();
    let (cpc_voltage, cpc_current, cpc_power) = cpc_vcp();

    let points = vec![
        influxdb2::models::DataPoint::builder("temperature")
            .tag("host", &hostname)
            .field("value", cpc_temp)
            .build()?,
        influxdb2::models::DataPoint::builder("voltage")
            .tag("host", &hostname)
            .field("value", cpc_voltage)
            .build()?,
        influxdb2::models::DataPoint::builder("current")
            .tag("host", &hostname)
            .field("value", cpc_current)
            .build()?,
        influxdb2::models::DataPoint::builder("power")
            .tag("host", &hostname)
            .field("value", cpc_power)
            .build()?,
    ];

    client.write(bucket, stream::iter(points)).await?;
    
    Ok(())
}

fn cpc_temperature() -> f64 {
    let cpc_tmp1075 = tmp1075::TMP1075::new(1, CPC_TMP1075_ADDRESS);
    cpc_tmp1075.config().expect("cannot configure TMP1075");
    let cpc_temp = cpc_tmp1075.read().expect("cannot read TMP1075") as f64;

    cpc_temp
}

fn cpc_vcp() -> (f64, f64, f64) {
    let cpc_ina219 = ina219::INA219::new(1, CPC_INA219_ADDRESS, CPC_INA219_RSHUNT, CPC_INA219_MEC);
    cpc_ina219.configure().expect("cannot configure INA219");
    let (cpc_voltage, cpc_current, cpc_power) = cpc_ina219.read_data().expect("cannot read INA219");

    (cpc_voltage as f64, cpc_current as f64, cpc_power as f64)
}