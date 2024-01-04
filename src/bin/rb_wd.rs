use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;
use std::fs::OpenOptions;
use std::io::prelude::*;
use chrono::Utc;

fn main() {
    match wd_gpio() {
        Ok(_) => {},
        Err(e) => {
            write_err_log(e.to_string()).unwrap();
            std::process::exit(1);
        }
    }
}

fn wd_gpio() -> Result<(), sysfs_gpio::Error> {
    let wd_gpio = Pin::new(955);

    wd_gpio.with_exported(|| {
        wd_gpio.set_direction(Direction::Out)?;
        loop {
            wd_gpio.set_value(0)?;
            sleep(Duration::from_secs(15));
            wd_gpio.set_value(1)?;
            sleep(Duration::from_secs(15));
        }
    })?;

    Ok(())
}

fn write_err_log(error: String) -> Result<(), std::io::Error> {

    let now = Utc::now().to_rfc3339();
    let err_msg = format!("{}: {}", now, error);
    
    let log_path = "/home/gaps/log/rb-wd.log";
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
        ?;

    writeln!(file, "{}", err_msg)?;

    Ok(())
}