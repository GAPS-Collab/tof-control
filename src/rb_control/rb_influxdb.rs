#![allow(unused)]
use chrono::Utc;
use futures::prelude::*;
use gethostname::gethostname;
use influxdb2::models::DataPoint;
use influxdb2::Client;
use influxdb2_derive::WriteDataPoint;

use crate::constant::*;
use crate::rb_control::*;

pub struct RBInfluxDB {
    pub org: &'static str,
    pub url: &'static str,
    pub token: &'static str,
    pub bucket: String,
}

impl RBInfluxDB {
    pub fn new(bucket: String) -> Self {
        let rb_name = gethostname()
            .into_string()
            .expect("cannot convert hostname")
            .replace("tof-rb", "RB");

        let org = INFLUXDB_ORG;
        let url = INFLUXDB_URL;
        let token = INFLUXDB_TOKEN;
        let bucket = bucket;
        // let bucket = Self::get_bucket();

        Self {
            org,
            url,
            token,
            bucket,
        }
    }
    // fn get_bucket() -> String {
    //     let hostname = gethostname().into_string().expect("cannot convert hostname");
    //     let bucket = hostname.replace("tof-rb", "RB");

    //     bucket
    // }
    pub async fn rb_influxdb_write(
        &self,
        rat_id: &str,
        test: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let hostname = gethostname()
            .into_string()
            .expect("cannot convert hostname");

        let client = influxdb2::Client::new(self.url, self.org, self.token);

        let mut rb_info_points = Self::rb_info_influxdb(&hostname, &rat_id, &test)
            .expect("cannot get RB info InfluxDB data points");
        let mut rb_temp_points = Self::rb_temp_influxdb(&hostname, &rat_id, &test)
            .expect("cannot get RB temperature InfluxDB data points");
        let mut rb_ph_points = Self::rb_ph_influxdb(&hostname, &rat_id, &test)
            .expect("cannot get RB pressure & humidity InfluxDB data points");
        let mut rb_mag_points = Self::rb_mag_influxdb(&hostname, &rat_id, &test)
            .expect("cannot get RB magnetic field InfluxDB data points");
        let mut rb_vcp_points = Self::rb_vcp_influxdb(&hostname, &rat_id, &test)
            .expect("cannot get RB vcp InfluxDB data points");

        let mut points: Vec<DataPoint> = Vec::new();
        points.append(&mut rb_info_points);
        points.append(&mut rb_temp_points);
        points.append(&mut rb_ph_points);
        points.append(&mut rb_mag_points);
        points.append(&mut rb_vcp_points);

        client.write(&self.bucket, stream::iter(points)).await?;

        Ok(())
    }
    fn rb_info_influxdb(
        hostname: &str,
        rat_id: &str,
        test: &str,
    ) -> Result<Vec<DataPoint>, Box<dyn std::error::Error>> {
        let rb_info = rb_info::RBinfo::new();

        let rb_info_points = vec![
            influxdb2::models::DataPoint::builder("board_id")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", format!("RB{:02}", rb_info.board_id))
                .build()?,
            influxdb2::models::DataPoint::builder("fw_version")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_info.global_ver)
                .build()?,
            influxdb2::models::DataPoint::builder("fw_hash")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", format!("{:02X}", rb_info.global_sha))
                .build()?,
            influxdb2::models::DataPoint::builder("lol_status")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_info.loss_of_lock)
                .build()?,
            influxdb2::models::DataPoint::builder("lol_stability")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_info.loss_of_lock_stable)
                .build()?,
        ];

        Ok((rb_info_points))
    }
    fn rb_temp_influxdb(
        hostname: &str,
        rat_id: &str,
        test: &str,
    ) -> Result<Vec<DataPoint>, Box<dyn std::error::Error>> {
        let rb_temp = rb_temp::RBtemp::new();

        let rb_temp_points = vec![
            influxdb2::models::DataPoint::builder("drs_temperature")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_temp.drs_temp as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("clk_temperature")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_temp.clk_temp as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("adc_temperature")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_temp.adc_temp as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("zynq_temperature")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_temp.zynq_temp as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("lis3mdltr_temperature")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_temp.lis3mdltr_temp as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("bme280_temperature")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_temp.bme280_temp as f64)
                .build()?,
        ];

        Ok((rb_temp_points))
    }
    fn rb_ph_influxdb(
        hostname: &str,
        rat_id: &str,
        test: &str,
    ) -> Result<Vec<DataPoint>, Box<dyn std::error::Error>> {
        let rb_ph = rb_ph::RBph::new();

        let rb_ph_points = vec![
            influxdb2::models::DataPoint::builder("pressure")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_ph.pressure as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("humidity")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_ph.humidity as f64)
                .build()?,
        ];

        Ok((rb_ph_points))
    }
    fn rb_mag_influxdb(
        hostname: &str,
        rat_id: &str,
        test: &str,
    ) -> Result<Vec<DataPoint>, Box<dyn std::error::Error>> {
        let rb_magnetic = rb_mag::RBmag::new();

        let rb_mag_points = vec![
            influxdb2::models::DataPoint::builder("magnetic_x")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_magnetic.magnetic_x as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("magnetic_y")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_magnetic.magnetic_y as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("magnetic_z")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_magnetic.magnetic_z as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("magnetic_t")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_magnetic.magnetic_t as f64)
                .build()?,
        ];

        Ok((rb_mag_points))
    }
    fn rb_vcp_influxdb(
        hostname: &str,
        rat_id: &str,
        test: &str,
    ) -> Result<Vec<DataPoint>, Box<dyn std::error::Error>> {
        let rb_vcp = rb_vcp::RBvcp::new();

        let rb_vcp_points = vec![
            influxdb2::models::DataPoint::builder("drs_dvdd_voltage")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_vcp.drs_dvdd_voltage as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("drs_dvdd_current")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_vcp.drs_dvdd_current as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("drs_dvdd_power")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_vcp.drs_dvdd_power as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("p3v3_voltage")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_vcp.p3v3_voltage as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("p3v3_current")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_vcp.p3v3_current as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("p3v3_power")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_vcp.p3v3_power as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("zynq_voltage")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_vcp.zynq_voltage as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("zynq_current")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_vcp.zynq_current as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("zynq_power")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_vcp.zynq_power as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("p3v5_voltage")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_vcp.p3v5_voltage as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("p3v5_current")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_vcp.p3v5_current as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("p3v5_power")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_vcp.p3v5_power as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("adc_dvdd_voltage")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_vcp.adc_dvdd_voltage as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("adc_dvdd_current")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_vcp.adc_dvdd_current as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("adc_dvdd_power")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_vcp.adc_dvdd_power as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("adc_avdd_voltage")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_vcp.adc_avdd_voltage as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("adc_avdd_current")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_vcp.adc_avdd_current as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("adc_avdd_power")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_vcp.adc_avdd_power as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("drs_avdd_voltage")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_vcp.drs_avdd_voltage as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("drs_avdd_current")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_vcp.drs_avdd_current as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("drs_avdd_power")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_vcp.drs_avdd_power as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("n1v5_voltage")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_vcp.n1v5_voltage as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("n1v5_current")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_vcp.n1v5_current as f64)
                .build()?,
            influxdb2::models::DataPoint::builder("n1v5_power")
                .tag("host", hostname)
                .tag("rat_id", rat_id)
                .tag("test", test)
                .field("value", rb_vcp.n1v5_power as f64)
                .build()?,
        ];

        Ok((rb_vcp_points))
    }
}

// async fn rb_temp_influxdb_write() {
//     let rb_temp = rb_temp::RBtemp::new();

// }
