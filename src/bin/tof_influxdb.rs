#![allow(unused)]
use std::thread;
use std::time::Duration;
use gethostname::gethostname;
use tof_control::rb_control::*;
use tof_control::ltb_control::*;
use tof_control::pb_control::*;
use tof_control::preamp_control::*;

// #[tokio::main]
// async fn main() {
//     loop {
//         let rb_influxdb = rb_influxdb::RBInfluxDB::new();
//         rb_influxdb.rb_influxdb_write().await;
//         thread::sleep(Duration::from_secs(600));
//     }
// }

#[tokio::main]
async fn main() {
    // let rat_config = rb_config::RBConfig::new();
    // println!("{:?}", rat_config.rb1_id_arr);
    rb_matching().await;
}

async fn rb_matching() {
    let rb_name = gethostname().into_string().expect("cannot convert hostname").replace("tof-rb", "").parse::<u8>().unwrap();
    // println!("{}", rb_name);
    let rat_config = rb_config::RBConfig::new();

    let mut rb_id: u8 = 0;
    let mut rat_id: u8 = 0;
    let mut ltb_id: u8 = 0;
    let mut pb_id: u8 = 0;

    let mut test = "nts";

    for (i, (rb1_id, rb2_id)) in rat_config.rb1_id_arr.iter().zip(rat_config.rb2_id_arr.iter()).enumerate() {
        if rb1_id == &rb_name {
            rb_id = *rb1_id;
            ltb_id = rat_config.ltb_id_arr[i];
            rat_id = rat_config.rat_id_arr[i];
            
            let rat_id_str = format!("{:02}", rat_id);
            
            let rb_bucket = format!("RB{:02}", rb_id);
            let ltb_bucket = format!("LTB{:02}", ltb_id);

            let rb_influxdb = rb_influxdb::RBInfluxDB::new(rb_bucket);
            rb_influxdb.rb_influxdb_write(&rat_id_str, test.to_string()).await;
            
            let ltb_influxdb = ltb_influxdb::LTBInfluxDB::new(ltb_bucket);
            ltb_influxdb.ltb_influxdb_write(&rat_id_str, test.to_string()).await;

            break;
        } else if rb2_id == &rb_name {
            rb_id = *rb2_id;
            pb_id = rat_config.pb_id_arr[i];
            rat_id = rat_config.rat_id_arr[i];
            let rat_id_str = format!("{:02}", rat_id);

            let rb_bucket = format!("RB{:02}", rb_id);
            let pb_bucket = format!("PB{:02}", pb_id);
            let preamp_bucket = pb_bucket.clone();

            let rb_influxdb = rb_influxdb::RBInfluxDB::new(rb_bucket);
            rb_influxdb.rb_influxdb_write(&rat_id_str, test.to_string()).await;

            let pb_influxdb = pb_influxdb::PBInfluxDB::new(pb_bucket);
            pb_influxdb.pb_influxdb_write(&rat_id_str, test.to_string()).await;

            let preamp_influxdb = preamp_influxdb::PreampInfluxDB::new(preamp_bucket);
            preamp_influxdb.preamp_influxdb_write(&rat_id_str, test.to_string()).await;
            break;
        }
    }

    // println!("{}, {}, {}, {}", rb_id, rat_id, ltb_id, pb_id);

}