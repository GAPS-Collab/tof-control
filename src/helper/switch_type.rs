#[derive(Debug)]
pub struct AllSwitchData {
    pub switch1: Option<SwitchData>,
    pub switch2: Option<SwitchData>,
    pub switch3: Option<SwitchData>,
}

#[derive(Debug)]
pub struct SwitchData {
    pub info: SwitchInfo,
    pub port: SwitchPort,
}

#[derive(Debug)]
pub struct SwitchInfo {
    pub hostname: String,
    pub uptime: String, // VTSSDisplayString (OCTET STRING) (SIZE(0..10)). Hint: 255a
    pub mac_address: String,
    pub cpu_load: [u8; 3], // [100ms, 1s, 10s]
}

#[derive(Debug)]
pub struct SwitchPort {
    pub link: [u8; 16], // TruthValue (INTEGER) {true(1), false(2) }
    pub speed: [u8; 16], // VTSSPortStatusSpeed (INTEGER) {undefined(0), speed10M(1), speed100M(2), speed1G(3), speed2G5(4), speed5G(5), speed10G(6), speed12G(7) }
    pub full_duplex: [u8; 16], // TruthValue (INTEGER) {true(1), false(2) }
}

/// Switch Error Type
#[derive(Debug)]
pub enum SwitchError {
    SNMP(snmp::SnmpError),
    ParseInt(std::num::ParseIntError),
    IO(std::io::Error),
    Address,
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