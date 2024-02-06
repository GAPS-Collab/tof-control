// use snmp::{SyncSession, Value, SnmpError};
// use std::time::Duration;

// fn main() {

//     // let temp_oid = &[1,3,6,1,4,1,38477,1,50,1,28,1,3,3,1,6];
//     // let uptime_oid = &[1,3,6,1,4,1,38477,1,50,1,24,1,3,4,1];
//     // let cpu_load_10sec_oid = &[1,3,6,1,4,1,38477,1,50,1,24,1,3,1,3];
//     let switch1_addr = "10.0.1.11:161";
//     let switch2_addr = "10.0.1.12:161";
//     // let community = b"public";
//     let timeout = Duration::from_secs(2);

//     println!("tof-switch1");
//     // uptime(switch1_addr, timeout);
//     // cpu_load(switch1_addr, timeout);
//     test_query(switch1_addr, timeout);


//     // let mut sess_1 = SyncSession::new(switch1_addr, community, Some(timeout), 0).unwrap();
//     // let mut response = sess_1.getnext(uptime_oid).unwrap();
//     // if let Some((_oid, Value::OctetString(sys_descr))) = response.varbinds.next() {
//     //     println!("\tUptime: {}", String::from_utf8_lossy(sys_descr));
//     // }
//     // response = sess_1.getnext(cpu_load_10sec_oid).unwrap();
//     // if let Some((_oid, Value::OctetString(sys_descr_2))) = response.varbinds.next() {
//     //     println!("\tCPU Load: {}[%]", String::from_utf8_lossy(sys_descr_2));
//     // }

//     println!("tof-switch2");
//     // uptime(switch2_addr, timeout);
//     // cpu_load(switch2_addr, timeout);
//     test_query(switch2_addr, timeout);
//     // let mut sess_2 = SyncSession::new(switch2_addr, community, Some(timeout), 0).unwrap();
//     // let mut response = sess_2.getnext(uptime_oid).unwrap();
//     // if let Some((_oid, Value::OctetString(sys_descr))) = response.varbinds.next() {
//     //     println!("Uptime: {}", String::from_utf8_lossy(sys_descr));
//     // }
// }

// fn uptime(address: &str, timeout: Duration) {
//     let oid = &[1,3,6,1,4,1,38477,1,50,1,24,1,3,4,1];

//     let mut sess = SyncSession::new(address, b"public", Some(timeout), 0).unwrap();
//     let mut response = sess.getnext(oid).unwrap();
//     if let Some((_oid, Value::OctetString(sys_descr))) = response.varbinds.next() {
//         println!("\tUptime: {}", String::from_utf8_lossy(sys_descr));
//     }

//     // Ok(())
// }

// fn cpu_load(address: &str, timeout: Duration) {
//     let oid = &[1,3,6,1,4,1,38477,1,50,1,24,1,3,1,3];

//     let mut sess = SyncSession::new(address, b"public", Some(timeout), 0).unwrap();
//     let mut response = sess.getnext(oid).unwrap();
//     if let Some((_oid, Value::Unsigned32(sys_descr))) = response.varbinds.next() {
//         println!("\tCPU Load: {}[%]", sys_descr);
//     }

//     // Ok(())
// }

// fn test_query(address: &str, timeout: Duration) {
//     let oid_str = " .1.3.6.1.4.1.38477.1.50.1.24.1.3.4.1";
//     let oid = convert_oid(oid_str);

//     let mut sess = SyncSession::new(address, b"public", Some(timeout), 0).unwrap();
//     let mut response = sess.getnext(&oid).unwrap();
//     if let Some((_oid, Value::OctetString(sys_descr))) = response.varbinds.next() {
//         println!("\tTest: {}[?]", String::from_utf8_lossy(sys_descr));
//         // println!("\tTest: {:?}", sys_descr);
//     }

//     // Ok(())
// }

// fn convert_oid(oid: &str) -> Vec<u32> {
//     let mut oid_trim = oid.trim();
//     oid_trim = oid_trim.chars().next().map(|c| &oid_trim[c.len_utf8()..]).unwrap();
//     let oid_char: Vec<&str> = oid_trim.split(".").collect();

//     let mut oid_vec: Vec<u32> = Default::default();
//     for c in oid_char {
//         oid_vec.push(c.parse::<u32>().unwrap());
//     }

//     oid_vec
// }

use tof_control::helper::switch_type::{SwitchInfo, SwitchPort};

fn main() {
    SwitchInfo::print_switch_info();
    SwitchPort::print_switch_port();
    
}