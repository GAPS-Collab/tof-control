#![allow(unused)]
mod constant;
mod device;
mod memory;
mod rb_control;
mod pb_control;
mod ltb_control;
mod preamp_control;

use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

use clap::{Parser, ValueEnum};

use rb_control::*;
use pb_control::*;
use ltb_control::*;
use preamp_control::*;

#[derive(Debug, Parser)]
// #[command(author = "Takeru Hayashi", version, about, long_about = None)]
#[clap(name = "tof-control", author, about, version)]
struct Cli {
    #[arg(short = 'b', long = "board", help = "Board to operate (rb, pb, ltb, or preamp)")]
    board: Board,
    #[clap(short, long, help = "Show status of board")]
    init: bool,
    #[clap(long, help = "Show status of board")]
    status: bool,
    #[clap(long, help = "Set")]
    set: bool,
    #[clap(long, help = "Reset")]
    reset: bool,
    #[arg(long, help = "RF Input")]
    input: Option<Input>,
    #[arg(long, help = "Select Mode")]
    mode: Option<Mode>,
}

#[derive(Debug, Clone, ValueEnum)]
enum Board {
    RB,
    PB,
    LTB,
    Preamp,
}

#[derive(Debug, Clone, ValueEnum)]
enum Input {
    OFF,
    SMA,
    TCA,
}

#[derive(Debug, Clone, ValueEnum)]
enum Mode {
    NOI,
    VCAL,
    TCAL,
    SMA,
}

fn main() {
    let cli = Cli::parse();

    match cli.board {
        Board::RB => {
            if cli.init {
                rb_init::initialize();
            }
            if cli.status {
                rb_table::rb_table();
            }
            match &cli.input {
                Some(input) => {
                    match &input {
                        Input::OFF => {
                            rb_input::disable_rf_input();
                        },
                        Input::SMA => {
                            rb_input::enable_sma_input();
                        },
                        Input::TCA => {
                            rb_input::enable_tca_input();
                        }
                    }
                }
                None => {
                }
            }
            match &cli.mode {
                Some(mode) => {
                    match &mode {
                        Mode::NOI => {
                            rb_mode::select_noi_mode();
                        },
                        Mode::VCAL => {
                            rb_mode::select_vcal_mode();
                        },
                        Mode::TCAL => {
                            rb_mode::select_tcal_mode();
                        },
                        Mode::SMA => {
                            rb_mode::select_sma_mode();
                        }
                    }
                }
                None => {
                }
            }
        },
        Board::PB => {
            if cli.init {
                todo!();
            }
            if cli.status {
                pb_table::pb_table();
            }
        },
        Board::LTB => {
            if cli.init {
                ltb_init::initialize();
            }
            if cli.status {
                ltb_table::ltb_table();
            };
            if cli.set {
                ltb_dac::LTBdac::set_threshold();
            };
            if cli.reset {
                ltb_dac::LTBdac::reset_threshold();
            }
        },
        Board::Preamp => {
            if cli.init {
                todo!();
            }
            if cli.status {
                preamp_table::preamp_table();
            }
            if cli.set {
                preamp_bias::PreampBiasSet::set_bias();
            }
            if cli.reset {
                preamp_bias::PreampBiasSet::reset_bias();
            }
        }
    }
}
