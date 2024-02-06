use snmp::{SyncSession, Value};
use std::borrow::Borrow;
use std::fmt::Debug;
use std::time::Duration;

use crate::constant::*;
use crate::helper::switch_type::{SwitchPort, SwitchError};
use crate::switch_control::switch_util::convert_oid;

impl SwitchPort {
    pub fn new() -> Self {
        match Self::read_port() {
            Ok(switch_port) => {
                switch_port
            }
            Err(_) => {
                Self {
                    link: [u8::MAX; 16],
                }
            }
        }
    }
    pub fn read_port() -> Result<SwitchPort, SwitchError> {
        let mut snmp_session = SyncSession::new(SWITCH1_ADDRESS, b"public", Some(Duration::from_secs(1)), 0)?;

        let link = Self::read_link(&mut snmp_session)?;

        Ok(
            SwitchPort {
                link,
            }
        )
    }
    pub fn read_link(session: &mut SyncSession) -> Result<[u8; 16], SwitchError> {
        let oid = convert_oid(".1.3.6.1.4.1.38477.1.50.1.11.1.3.1.1.2")?;
        let response = session.getbulk(&[&oid], 0, 16)?;

        let mut link: [u8; 16] = Default::default();
        for (_oid, val) in response.varbinds {
            println!("{:?}", val);
            let val_str = format!("{:?}", val);
            // link[i] = val_str.parse::<u8>()?;
        }
        Ok(link)
    }
    pub fn print_switch_port() {
        let switch_port = Self::new();

        println!("Switch Port");
        println!("\tLink:       {:?}", switch_port.link);
    }
}