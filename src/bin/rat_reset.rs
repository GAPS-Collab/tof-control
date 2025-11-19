use std::fs::OpenOptions;
use std::io::prelude::*;
use clap::Parser;
use chrono::Utc;

use tof_control::rb_control::rb_reset;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {}

fn main() {
    let _args = Args::parse();

    // Reset RB
    reset_rb();
}

fn reset_rb() {
    match rb_reset::reset() {
        Ok(_) => {},
        Err(e) => {
            write_err_log(e.to_string()).unwrap();
            std::process::exit(1);
        }
    }
}

fn write_err_log(error: String) -> Result<(), std::io::Error> {

    let now = Utc::now().to_rfc3339();
    let err_msg = format!("{}: {}", now, error);
    
    let log_path = "/home/gaps/log/rat-reset.log";
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
        ?;

    writeln!(file, "{}", err_msg)?;

    Ok(())
}