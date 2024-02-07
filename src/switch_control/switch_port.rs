use snmp::SyncSession;
use std::time::Duration;

use crate::constant::*;
use crate::helper::switch_type::{SwitchPort, SwitchError};
use crate::switch_control::switch_util::snmp_getbulk_integer;

impl SwitchPort {
    pub fn new() -> Self {
        match Self::read_port() {
            Ok(switch_port) => {
                switch_port
            }
            Err(_) => {
                Self {
                    link: [u8::MAX; 16],
                    speed: [u8::MAX; 16],
                    full_duplex: [u8::MAX; 16],
                }
            }
        }
    }
    pub fn read_port() -> Result<SwitchPort, SwitchError> {
        let mut snmp_session = SyncSession::new(SWITCH1_ADDRESS, b"public", Some(Duration::from_secs(1)), 0)?;

        let link = Self::read_link(&mut snmp_session)?;
        let speed = Self::read_speed(&mut snmp_session)?;
        let full_duplex = Self::read_full_duplex(&mut snmp_session)?;

        Ok(
            SwitchPort {
                link,
                speed,
                full_duplex,
            }
        )
    }
    pub fn read_link(session: &mut SyncSession) -> Result<[u8; 16], SwitchError> {
        let oid = ".1.3.6.1.4.1.38477.1.50.1.11.1.3.1.1.2";
        let link = snmp_getbulk_integer(oid, session)?;

        Ok(link)
    }
    pub fn read_speed(session: &mut SyncSession) -> Result<[u8; 16], SwitchError> {
        let oid = ".1.3.6.1.4.1.38477.1.50.1.11.1.3.1.1.5";
        let speed = snmp_getbulk_integer(oid, session)?;

        Ok(speed)
    }
    pub fn read_full_duplex(session: &mut SyncSession) -> Result<[u8; 16], SwitchError> {
        let oid = ".1.3.6.1.4.1.38477.1.50.1.11.1.3.1.1.3";
        let fdx = snmp_getbulk_integer(oid, session)?;

        Ok(fdx)
    }

    pub fn print_switch_port() {
        let switch_port = Self::new();

        println!("Switch Port");
        println!("\tLink:           {:?}", switch_port.link);
        println!("\tSpeed:          {:?}", switch_port.speed);
        println!("\tFull Duplex:    {:?}", switch_port.full_duplex);
    }
}