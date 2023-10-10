#![allow(unused)]
use chrono::Utc;
use futures::prelude::*;
use gethostname::gethostname;
use influxdb2::models::DataPoint;
use influxdb2::Client;
use influxdb2_derive::WriteDataPoint;
use std::thread;
use std::time::Duration;

use tof_control::constant::*;
use tof_control::device::{ina219, max7320, tmp1075};

#[tokio::main]
async fn main() {
    loop {
        tcpc_influxdb_write().await;
        thread::sleep(Duration::from_secs(10));
    }
}

// #[tokio::main]
async fn tcpc_influxdb_write() -> Result<(), Box<dyn std::error::Error>> {
    let hostname = gethostname()
        .into_string()
        .expect("cannot convert hostname");

    let org = "gaps";
    let bucket = "TCPC01";
    let influx_url = "http://10.97.108.31:8086";
    let token =
        "WXqbeoDBq0euIIK-1OyBKrJ0R4XwOq5TZaZiIEtKTlcXgqTLo8tMIZgkft3OTgTQD2LQyejRo8h3mrMWsCTfYw==";

    let client = influxdb2::Client::new(influx_url, org, token);

    let tcpc_temp = tcpc_temperature();
    let (tcpc_voltage, tcpc_current, tcpc_power) = tcpc_vcp();

    let points = vec![
        influxdb2::models::DataPoint::builder("temperature")
            .tag("host", &hostname)
            .field("value", tcpc_temp)
            .build()?,
        influxdb2::models::DataPoint::builder("voltage")
            .tag("host", &hostname)
            .field("value", tcpc_voltage)
            .build()?,
        influxdb2::models::DataPoint::builder("current")
            .tag("host", &hostname)
            .field("value", tcpc_current)
            .build()?,
        influxdb2::models::DataPoint::builder("power")
            .tag("host", &hostname)
            .field("value", tcpc_power)
            .build()?,
    ];

    client.write(bucket, stream::iter(points)).await?;

    Ok(())
}

fn tcpc_temperature() -> f64 {
    let tcpc_tmp1075 = tmp1075::TMP1075::new(1, TCPC_TMP1075_ADDRESS);
    tcpc_tmp1075.config().expect("cannot configure TMP1075");
    let tcpc_temp = tcpc_tmp1075.read().expect("cannot read TMP1075") as f64;

    tcpc_temp
}

fn tcpc_vcp() -> (f64, f64, f64) {
    let tcpc_ina219 =
        ina219::INA219::new(1, TCPC_INA219_ADDRESS, TCPC_INA219_RSHUNT, TCPC_INA219_MEC);
    tcpc_ina219.configure().expect("cannot configure INA219");
    let (tcpc_voltage, tcpc_current, tcpc_power) =
        tcpc_ina219.read_data().expect("cannot read INA219");

    (tcpc_voltage as f64, tcpc_current as f64, tcpc_power as f64)
}
