#![allow(unused)]
use gethostname::gethostname;
use chrono::Utc;
use futures::prelude::*;
use influxdb2::models::DataPoint;
use influxdb2::Client;
use influxdb2_derive::WriteDataPoint;

use crate::ltb_control::*;
use crate::constant::*;

pub struct LTBInfluxDB {
    pub org: &'static str,
    pub url: &'static str,
    pub token: &'static str,
    pub bucket: String,
}

impl LTBInfluxDB {
    pub fn new(bucket: String) -> Self {
        
        let org = INFLUXDB_ORG;
        let url = INFLUXDB_URL;
        let token = INFLUXDB_TOKEN;
        let bucket = bucket;

        Self {
            org,
            url,
            token,
            bucket,
        }
    }
    pub async fn ltb_influxdb_write(&self, rat_id: &str, test: String) -> Result<(), Box<dyn std::error::Error>> {
        let hostname = gethostname().into_string().expect("cannot convert hostname");
        
        let client = influxdb2::Client::new(self.url, self.org, self.token);

        let mut ltb_temp_points = Self::ltb_temp_influxdb(&hostname, &rat_id, &test).expect("cannot get LTB temperature InfluxDB data points");
        let mut ltb_threshold_points = Self::ltb_threshold_influxdb(&hostname, &rat_id, &test).expect("cannot get LTB threshold InfluxDB data points");

        let mut points: Vec<DataPoint> = Vec::new();
        points.append(&mut ltb_temp_points);
        points.append(&mut ltb_threshold_points);

        client.write(&self.bucket, stream::iter(points)).await?;

        Ok(())
    }
    fn ltb_temp_influxdb(hostname: &str, rat_id: &str, test: &str) -> Result<Vec<DataPoint>, Box<dyn std::error::Error>> {
        let ltb_temp = ltb_temp::LTBtemp::new();

        let ltb_temp_points = vec![
        influxdb2::models::DataPoint::builder("ltb_temperature")
            .tag("host", hostname)
            .tag("rat_id", rat_id).tag("test", test)
            .field("value", ltb_temp.ltb_temp as f64)
            .build()?,
        influxdb2::models::DataPoint::builder("trenz_temperature")
            .tag("host", hostname)
            .tag("rat_id", rat_id).tag("test", test)
            .field("value", ltb_temp.trenz_temp as f64)
            .build()?,
        ];

        Ok((ltb_temp_points))
    }
    fn ltb_threshold_influxdb(hostname: &str, rat_id: &str, test: &str) -> Result<Vec<DataPoint>, Box<dyn std::error::Error>> {
        let ltb_threshold = ltb_dac::LTBdac::new();

        let ltb_threshold_points = vec![
        influxdb2::models::DataPoint::builder("threshold_0")
            .tag("host", hostname)
            .tag("rat_id", rat_id).tag("test", test)
            .field("value", ltb_threshold.dac0 as f64)
            .build()?,
            influxdb2::models::DataPoint::builder("threshold_1")
            .tag("host", hostname)
            .tag("rat_id", rat_id).tag("test", test)
            .field("value", ltb_threshold.dac1 as f64)
            .build()?,
            influxdb2::models::DataPoint::builder("threshold_2")
            .tag("host", hostname)
            .tag("rat_id", rat_id).tag("test", test)
            .field("value", ltb_threshold.dac2 as f64)
            .build()?,
        ];

        Ok((ltb_threshold_points))
    }
}