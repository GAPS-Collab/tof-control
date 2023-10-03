#![allow(unused)]
use chrono::Utc;
use futures::prelude::*;
use gethostname::gethostname;
use influxdb2::models::DataPoint;
use influxdb2::Client;
use influxdb2_derive::WriteDataPoint;

use crate::constant::*;
use crate::preamp_control::*;

pub struct PreampInfluxDB {
    pub org: &'static str,
    pub url: &'static str,
    pub token: &'static str,
    pub bucket: String,
}

impl PreampInfluxDB {
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
    pub async fn preamp_influxdb_write(
        &self,
        rat_id: &str,
        test: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let hostname = gethostname()
            .into_string()
            .expect("cannot convert hostname");

        let client = influxdb2::Client::new(self.url, self.org, self.token);

        let mut preamp_temp_points = Self::preamp_temp_influxdb(&hostname, &rat_id, &test)
            .expect("cannot get Preamp temperature InfluxDB data points");
        let mut preamp_bias_points = Self::preamp_bias_influxdb(&hostname, &rat_id, &test)
            .expect("cannot get Preamp bias InfluxDB data points");

        let mut points: Vec<DataPoint> = Vec::new();
        points.append(&mut preamp_temp_points);
        points.append(&mut preamp_bias_points);

        client.write(&self.bucket, stream::iter(points)).await?;

        Ok(())
    }
    fn preamp_temp_influxdb(
        hostname: &str,
        rat_id: &str,
        test: &str,
    ) -> Result<Vec<DataPoint>, Box<dyn std::error::Error>> {
        let preamp_temp = preamp_temp::PreampTemp::new();

        let preamp_temp_points = vec![
            influxdb2::models::DataPoint::builder("preamp_1_temp")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", preamp_temp.preamp_tmp_1 as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("preamp_2_temp")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", preamp_temp.preamp_tmp_2 as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("preamp_3_temp")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", preamp_temp.preamp_tmp_3 as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("preamp_4_temp")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", preamp_temp.preamp_tmp_4 as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("preamp_5_temp")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", preamp_temp.preamp_tmp_5 as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("preamp_6_temp")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", preamp_temp.preamp_tmp_6 as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("preamp_7_temp")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", preamp_temp.preamp_tmp_7 as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("preamp_8_temp")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", preamp_temp.preamp_tmp_8 as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("preamp_9_temp")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", preamp_temp.preamp_tmp_9 as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("preamp_10_temp")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", preamp_temp.preamp_tmp_10 as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("preamp_11_temp")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", preamp_temp.preamp_tmp_11 as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("preamp_12_temp")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", preamp_temp.preamp_tmp_12 as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("preamp_13_temp")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", preamp_temp.preamp_tmp_13 as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("preamp_14_temp")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", preamp_temp.preamp_tmp_14 as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("preamp_15_temp")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", preamp_temp.preamp_tmp_15 as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("preamp_16_temp")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", preamp_temp.preamp_tmp_16 as f64)
                .build()?,
        ];

        Ok((preamp_temp_points))
    }
    fn preamp_bias_influxdb(
        hostname: &str,
        rat_id: &str,
        test: &str,
    ) -> Result<Vec<DataPoint>, Box<dyn std::error::Error>> {
        let preamp_bias = preamp_bias::PreampBiasRead::new();

        let preamp_bias_points = vec![
            influxdb2::models::DataPoint::builder("preamp_bias_1")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", preamp_bias.preamp_bias_read_1 as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("preamp_bias_2")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", preamp_bias.preamp_bias_read_2 as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("preamp_bias_3")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", preamp_bias.preamp_bias_read_3 as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("preamp_bias_4")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", preamp_bias.preamp_bias_read_4 as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("preamp_bias_5")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", preamp_bias.preamp_bias_read_5 as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("preamp_bias_6")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", preamp_bias.preamp_bias_read_6 as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("preamp_bias_7")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", preamp_bias.preamp_bias_read_7 as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("preamp_bias_8")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", preamp_bias.preamp_bias_read_8 as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("preamp_bias_9")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", preamp_bias.preamp_bias_read_9 as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("preamp_bias_10")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", preamp_bias.preamp_bias_read_10 as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("preamp_bias_11")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", preamp_bias.preamp_bias_read_11 as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("preamp_bias_12")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", preamp_bias.preamp_bias_read_12 as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("preamp_bias_13")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", preamp_bias.preamp_bias_read_13 as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("preamp_bias_14")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", preamp_bias.preamp_bias_read_14 as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("preamp_bias_15")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", preamp_bias.preamp_bias_read_15 as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("preamp_bias_16")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", preamp_bias.preamp_bias_read_16 as f64)
                .build()?,
        ];

        Ok((preamp_bias_points))
    }
}
