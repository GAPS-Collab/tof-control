use std::{error::Error, time::Duration};

use tof_control::tui::cpu_crossterm;

fn main() -> Result<(), Box<dyn Error>> {
    let tick_rate = Duration::from_millis(250);
    // let tick_rate = Duration::from_secs(1);

    cpu_crossterm::run(tick_rate)?;

    Ok(())
}