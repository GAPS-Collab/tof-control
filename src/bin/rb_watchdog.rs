use std::fs::File;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

fn main() {
    loop {
        switch_watchdog_gpio(true).expect("cannot write gpio");
        thread::sleep(Duration::from_secs(15));
        switch_watchdog_gpio(false).expect("cannot write gpio");
        thread::sleep(Duration::from_secs(15));
    }
}

fn switch_watchdog_gpio(operation: bool) -> std::io::Result<()> {
    let mut file = File::create("/sys/class/gpio/gpio955/value")?;

    match operation {
        true => file.write_all(b"1")?,
        false => file.write_all(b"0")?,
    };

    Ok(())
}
