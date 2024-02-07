use snmp::SyncSession;
use std::time::Duration;

use crate::constant::*;
use crate::helper::switch_type::{SwitchInfo, SwitchError};
use super::switch_util::{snmp_get_octetstring, snmp_get_octetstring_raw};

impl SwitchInfo {
    pub fn new() -> Self {
        match Self::read_info() {
            Ok(switch_info) => {
                switch_info
            }
            Err(_) => {
                Self {
                    hostname: "".to_string(),
                    uptime: "".to_string(),
                    mac_address: "".to_string(),
                }
            }
        }
    }
    pub fn read_info() -> Result<SwitchInfo, SwitchError> {
        let mut snmp_session = SyncSession::new(SWITCH1_ADDRESS, b"public", Some(Duration::from_secs(2)), 0)?;

        let hostname = Self::read_hostname(&mut snmp_session)?;
        let uptime = Self::read_uptime(&mut snmp_session)?;
        let mac_address = Self::read_mac_address(&mut snmp_session)?;

        Ok(
            SwitchInfo {
                hostname,
                uptime,
                mac_address,
            }
        )
    }
    pub fn read_hostname(session: &mut SyncSession) -> Result<String, SwitchError> {
        let oid = ".1.3.6.1.4.1.38477.1.50.1.24.1.2.1.1";
        let hostname = snmp_get_octetstring(oid, session)?;

        Ok(hostname)
    }
    pub fn read_uptime(session: &mut SyncSession) -> Result<String, SwitchError> {
        let oid = ".1.3.6.1.4.1.38477.1.50.1.24.1.3.4.1";
        let uptime = snmp_get_octetstring(oid, session)?;

        Ok(uptime)
    }
    pub fn read_mac_address(session: &mut SyncSession) -> Result<String, SwitchError> {
        let oid = ".1.3.6.1.4.1.38477.1.50.1.24.1.3.5.1";
        let mac_address_vec = snmp_get_octetstring_raw(oid, session)?;
        let mut mac_address: String = Default::default();
        let mut vec_len = mac_address_vec.len();
        for octet in mac_address_vec.iter() {
            let octet_hex = format!("{:02X}", octet);
            if vec_len == 1 {
                mac_address = mac_address + &octet_hex;
            } else {
                mac_address = mac_address + &octet_hex + "-";
                vec_len -= 1;
            }
        }
        
        Ok(mac_address)
    }
    pub fn print_switch_info() {
        let switch_info = Self::new();

        println!("Switch Info");
        println!("\tHostname:       {}", switch_info.hostname);
        println!("\tUptime:         {}", switch_info.uptime);
        println!("\tMac Address:    {}", switch_info.mac_address);
    }
}