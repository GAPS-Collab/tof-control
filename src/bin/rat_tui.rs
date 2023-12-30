
use std::{error::Error, time::Duration};

use tof_control::tui::rat_crossterm;

fn main() -> Result<(), Box<dyn Error>> {
    let tick_rate = Duration::from_millis(250);

    rat_crossterm::run(tick_rate)?;

    Ok(())
}