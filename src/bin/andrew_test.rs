use std::process::Command;
use std::{error::Error, fs::{rename,File}, io::BufReader, path::Path};
use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use chrono::prelude::*;
// use indicatif::ProgressBar;

use tof_control::constant::*;
use tof_control::memory::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Trigger Mode")]
    trigger: Option<Trigger>,
    #[arg(long, help = "Number of Events to Take")]
    nevent: Option<usize>,
}

#[derive(ValueEnum, Clone, Debug)]
enum Trigger {
    SELF,
    MTB,
}

fn main() {

    let datetime = Utc::now();

    let args = Args::parse();

    let mut nevent = Default::default();
    match args.nevent {
        Some(n) => {
            nevent = n;
        }
        None => {
            nevent = 1000;
        }
    }

    let liftof_rb_path = "/home/gaps/test/bin/liftof-rb-debug";
    let config_path = "/home/gaps/test/config/andrew-test.json";

    match args.trigger {
        Some(trigger) => {
            match trigger {
                Trigger::SELF => {
                    println!("Self Trigger Mode");
                    edit_config(config_path, false, nevent);
                    match run_liftof(liftof_rb_path, config_path) {
                        Ok(_) => {
                            println!("DAQ Complete (Self Trigger)!");
                            mv_file(datetime).unwrap();
                        }
                        Err(e) => {
                            eprintln!("Failed to take data: {:?}", e);
                        }
                    }
                }
                Trigger::MTB => {
                    println!("MTB Trigger Mode");
                    edit_config(config_path, true, nevent);
                    match run_liftof(liftof_rb_path, config_path) {
                        Ok(_) => {
                            println!("DAQ Complete (MTB Trigger)!");
                            mv_file(datetime).unwrap();
                        }
                        Err(e) => {
                            eprintln!("Failed to take data: {:?}", e);
                        }
                    }
                }
            }
        }
        None => {
            println!("No trigger specified. Use self trigger...")
        }
    }
}

// fn progress_bar(nevent: usize) {
//     let bar = ProgressBar::new(nevent as u64);
//     for _ in 0..nevent {
//         std::thread::sleep(std::time::Duration::from_millis(10));
//         bar.inc(1);
//     }
//     bar.finish();
// }

fn mv_file(datetime: DateTime<Utc>) -> std::io::Result<()> {
    let board_id = read_control_reg(BOARD_ID).unwrap();
    let original_file = format!("/home/gaps/test/rb_{}.tof.gaps", board_id);
    let new_file = format!("/home/gaps/test/data/tof-rb{}_andrew_test-{}.tof.gaps",board_id, datetime.format("%Y%m%d_%H%M%S"));
    let _ = rename(original_file, new_file);

    Ok(())
}

fn run_liftof(bin: &str, config: &str) -> Result<(), Box<dyn Error>> {
    let _command = Command::new(bin)
                                    .arg("-r")
                                    .arg(config)
                                    .arg("--to-local-file")
                                    .output()?;
    
    Ok(())
}

fn edit_config(file: &str, mode: bool, nevents: usize) {
    match mode {
        true => {
            // MTB Trigger Mode
            match load_config_json(file) {
                Ok(config) => {
                    let new_config = Config {
                        nevents: nevents,
                        is_active: config.is_active,
                        nseconds: config.nseconds,
                        stream_any: config.stream_any,
                        trigger_poisson_rate: 0,
                        trigger_fixed_rate: 0,
                        latch_to_mtb: true,
                        data_type: config.data_type,
                        rb_buff_size: config.rb_buff_size,
                    };

                    match save_config_json(file, &new_config) {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("Failed to edit configuration file: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to load configuration file: {:?}", e);
                }
            }
        }
        false => {
            // Self Trigger Mode
            match load_config_json(file) {
                Ok(config) => {
                    let new_config = Config {
                        nevents: nevents,
                        is_active: config.is_active,
                        nseconds: config.nseconds,
                        stream_any: config.stream_any,
                        trigger_poisson_rate: 0,
                        trigger_fixed_rate: 100,
                        latch_to_mtb: false,
                        data_type: config.data_type,
                        rb_buff_size: config.rb_buff_size,
                    };

                    match save_config_json(file, &new_config) {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("Failed to edit configuration file: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to load configuration file: {:?}", e);
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    nevents: usize,
    is_active: bool,
    nseconds: usize,
    stream_any: bool,
    trigger_poisson_rate: usize,
    trigger_fixed_rate: usize,
    latch_to_mtb: bool,
    data_type: String,
    rb_buff_size: usize,
}

fn load_config_json<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let config = serde_json::from_reader(reader)?;

    Ok(config)
}

fn save_config_json<P: AsRef<Path>>(path: P, config: &Config) -> Result<(), Box<dyn Error>> {
    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, config)?;

    Ok(())
}