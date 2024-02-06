// #[derive(Debug)]
// pub struct SwitchData {
//     pub switch1: Switch1Data,
// }

// #[derive(Debug)]
// pub struct Switch1Data {
//     pub info: SwitchInfo,
// }

#[derive(Debug)]
pub struct SwitchInfo {
    pub hostname: String,
    pub uptime: String,
    pub mac_address: String,
}

#[derive(Debug)]
pub struct SwitchPort {
    pub link: [u8; 16],
}

/// Switch Error Type
#[derive(Debug)]
pub enum SwitchError {
    SNMP(snmp::SnmpError),
    ParseInt(std::num::ParseIntError),
    IO(std::io::Error),
}

impl From<snmp::SnmpError> for SwitchError {
    fn from(e: snmp::SnmpError) -> Self {
        SwitchError::SNMP(e)
    }
}

impl From<std::num::ParseIntError> for SwitchError {
    fn from(e: std::num::ParseIntError) -> Self {
        SwitchError::ParseInt(e)
    }
}

impl From<std::io::Error> for SwitchError {
    fn from(e: std::io::Error) -> Self {
        SwitchError::IO(e)
    }
}