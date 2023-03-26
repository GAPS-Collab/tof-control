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
                            rb_gpioe::disable_nb3v9312c();
                            rb_gpioe::rf_input_select(0);
                            // rb_gpioe::read_port();
                        },
                        Input::SMA => {
                            rb_gpioe::disable_nb3v9312c();
                            rb_gpioe::rf_input_select(1);
                            // rb_gpioe::read_port();
                        },
                        Input::TCA => {
                            rb_gpioe::enable_nb3v9312c();
                            rb_gpioe::rf_input_select(2);
                            // rb_gpioe::read_port();
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
