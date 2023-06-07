#![allow(unused)]
use gethostname::gethostname;
use chrono::Utc;
use futures::prelude::*;
use influxdb2::models::DataPoint;
use influxdb2::Client;
use influxdb2_derive::WriteDataPoint;

use crate::pb_control::*;
use crate::constant::*;

pub struct PBInfluxDB {
    pub org: &'static str,
    pub url: &'static str,
    pub token: &'static str,
    pub bucket: String,
}

impl PBInfluxDB {
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
    pub async fn pb_influxdb_write(&self, rat_id: &str, test: String) -> Result<(), Box<dyn std::error::Error>> {
        let hostname = gethostname().into_string().expect("cannot convert hostname");
        
        let client = influxdb2::Client::new(self.url, self.org, self.token);

        let mut pb_temp_points = Self::pb_temp_influxdb(&hostname, &rat_id, &test).expect("cannot get PB temperature InfluxDB data points");
        let mut pb_vcp_points = Self::pb_vcp_influxdb(&hostname, &rat_id, &test).expect("cannot get PB VCP InfluxDB data points");

        let mut points: Vec<DataPoint> = Vec::new();
        points.append(&mut pb_temp_points);
        points.append(&mut pb_vcp_points);

        client.write(&self.bucket, stream::iter(points)).await?;

        Ok(())
    }
    fn pb_temp_influxdb(hostname: &str, rat_id: &str, test: &str) -> Result<Vec<DataPoint>, Box<dyn std::error::Error>> {
        let pb_temp = pb_temp::PBtemp::new();

        let pb_temp_points = vec![
        influxdb2::models::DataPoint::builder("pds_temperature")
            .tag("host", hostname)
            .tag("rat_id", rat_id).tag("test", test)
            .field("value", pb_temp.pds_temp as f64)
            .build()?,
        influxdb2::models::DataPoint::builder("pas_temperature")
            .tag("host", hostname)
            .tag("rat_id", rat_id).tag("test", test)
            .field("value", pb_temp.pas_temp as f64)
            .build()?,
        influxdb2::models::DataPoint::builder("nas_temperature")
            .tag("host", hostname)
            .tag("rat_id", rat_id).tag("test", test)
            .field("value", pb_temp.nas_temp as f64)
            .build()?,
        influxdb2::models::DataPoint::builder("shv_temperature")
            .tag("host", hostname)
            .tag("rat_id", rat_id).tag("test", test)
            .field("value", pb_temp.shv_temp as f64)
            .build()?,
        ];

        Ok((pb_temp_points))
    }
    fn pb_vcp_influxdb(hostname: &str, rat_id: &str, test: &str) -> Result<Vec<DataPoint>, Box<dyn std::error::Error>> {
        let pb_vcp = pb_vcp::PBvcp::new();

        let pb_vcp_points = vec![
        influxdb2::models::DataPoint::builder("p3v4f_ltb_voltage")
            .tag("host", hostname)
            .tag("rat_id", rat_id).tag("test", test)
            .field("value", pb_vcp.p3v4f_ltb_voltage as f64)
            .build()?,
        influxdb2::models::DataPoint::builder("p3v4f_ltb_current")
            .tag("host", hostname)
            .tag("rat_id", rat_id).tag("test", test)
            .field("value", pb_vcp.p3v4f_ltb_current as f64)
            .build()?,
        influxdb2::models::DataPoint::builder("p3v4f_ltb_power")
            .tag("host", hostname)
            .tag("rat_id", rat_id).tag("test", test)
            .field("value", pb_vcp.p3v4f_ltb_power as f64)
            .build()?,
        influxdb2::models::DataPoint::builder("p3v4d_ltb_voltage")
            .tag("host", hostname)
            .tag("rat_id", rat_id).tag("test", test)
            .field("value", pb_vcp.p3v4d_ltb_voltage as f64)
            .build()?,
        influxdb2::models::DataPoint::builder("p3v4d_ltb_current")
            .tag("host", hostname)
            .tag("rat_id", rat_id).tag("test", test)
            .field("value", pb_vcp.p3v4d_ltb_current as f64)
            .build()?,
        influxdb2::models::DataPoint::builder("p3v4d_ltb_power")
            .tag("host", hostname)
            .tag("rat_id", rat_id).tag("test", test)
            .field("value", pb_vcp.p3v4d_ltb_power as f64)
            .build()?,
        influxdb2::models::DataPoint::builder("p3v6_ltb_voltage")
            .tag("host", hostname)
            .tag("rat_id", rat_id).tag("test", test)
            .field("value", pb_vcp.p3v6_ltb_voltage as f64)
            .build()?,
        influxdb2::models::DataPoint::builder("p3v6_ltb_current")
            .tag("host", hostname)
            .tag("rat_id", rat_id).tag("test", test)
            .field("value", pb_vcp.p3v6_ltb_current as f64)
            .build()?,
        influxdb2::models::DataPoint::builder("p3v6_ltb_power")
            .tag("host", hostname)
            .tag("rat_id", rat_id).tag("test", test)
            .field("value", pb_vcp.p3v6_ltb_power as f64)
            .build()?,
        influxdb2::models::DataPoint::builder("n1v6_ltb_voltage")
            .tag("host", hostname)
            .tag("rat_id", rat_id).tag("test", test)
            .field("value", pb_vcp.n1v6_ltb_voltage as f64)
            .build()?,
        influxdb2::models::DataPoint::builder("n1v6_ltb_current")
            .tag("host", hostname)
            .tag("rat_id", rat_id).tag("test", test)
            .field("value", pb_vcp.n1v6_ltb_current as f64)
            .build()?,
        influxdb2::models::DataPoint::builder("n1v6_ltb_power")
            .tag("host", hostname)
            .tag("rat_id", rat_id).tag("test", test)
            .field("value", pb_vcp.n1v6_ltb_power as f64)
            .build()?,
        influxdb2::models::DataPoint::builder("p3v6_preamp_voltage")
            .tag("host", hostname)
            .tag("rat_id", rat_id).tag("test", test)
            .field("value", pb_vcp.p3v6_preamp_voltage as f64)
            .build()?,
        influxdb2::models::DataPoint::builder("p3v6_preamp_current")
            .tag("host", hostname)
            .tag("rat_id", rat_id).tag("test", test)
            .field("value", pb_vcp.p3v6_preamp_current as f64)
            .build()?,
        influxdb2::models::DataPoint::builder("p3v6_preamp_power")
            .tag("host", hostname)
            .tag("rat_id", rat_id).tag("test", test)
            .field("value", pb_vcp.p3v6_preamp_power as f64)
            .build()?,
        influxdb2::models::DataPoint::builder("n1v6_preamp_voltage")
            .tag("host", hostname)
            .tag("rat_id", rat_id).tag("test", test)
            .field("value", pb_vcp.n1v6_preamp_voltage as f64)
            .build()?,
        influxdb2::models::DataPoint::builder("n1v6_preamp_current")
            .tag("host", hostname)
            .tag("rat_id", rat_id).tag("test", test)
            .field("value", pb_vcp.n1v6_preamp_current as f64)
            .build()?,
        influxdb2::models::DataPoint::builder("n1v6_preamp_power")
            .tag("host", hostname)
            .tag("rat_id", rat_id).tag("test", test)
            .field("value", pb_vcp.n1v6_preamp_power as f64)
            .build()?,
        ];

        Ok((pb_vcp_points))
    }
}