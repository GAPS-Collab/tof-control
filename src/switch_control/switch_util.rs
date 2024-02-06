use snmp::{SyncSession, Value};

use crate::helper::switch_type::SwitchError;

pub fn convert_oid(oid_str: &str) -> Result<Vec<u32>, SwitchError> {
    let mut oid_trim = oid_str.trim();
    oid_trim = oid_trim.chars().next().map(|c| &oid_trim[c.len_utf8()..]).unwrap_or(oid_trim);
    let oid_char: Vec<&str> = oid_trim.split(".").collect();

    let mut oid: Vec<u32> = Default::default();
    for c in oid_char {
        oid.push(c.parse::<u32>()?);
    }

    Ok(oid)
}

pub fn read_snmp_octetstring(oid_str: &str, session: &mut SyncSession) -> Result<String, SwitchError> {
    let oid = convert_oid(oid_str)?;
    let mut response = session.getnext(&oid)?;
    let mut value: String = Default::default();
    if let Some((_oid, Value::OctetString(sys_descr))) = response.varbinds.next() {
        value = String::from_utf8_lossy(sys_descr).to_string();
    }

    Ok(value)
}