use std::fs::OpenOptions;
use std::io;
use std::path::Path;
use once_cell::sync::Lazy;

use crate::constant::I2C_BUS;

static I2C_LOCK_PATH: Lazy<String> = Lazy::new(|| {
    format!("/var/lock/i2c-{}.lock", I2C_BUS)
});

pub fn with_i2c_bus_lock<T, E, F>(f: F) -> Result<T, E>
where
    F: FnOnce() -> Result<T, E>,
    E: From<io::Error>,
{
    let lock_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(Path::new(&*I2C_LOCK_PATH))?;

    lock_file.lock()?;

    let result = f();

    let unlock_res = lock_file.unlock();

    match (result, unlock_res) {
        (Ok(val), Ok(())) => Ok(val),
        (Ok(_), Err(e)) => Err(E::from(e)),
        (Err(e), _) => Err(e),
    }
}