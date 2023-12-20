use clap::{Parser, Subcommand, ValueEnum};
use std::fs::OpenOptions;
use std::io::prelude::*;
use chrono::Utc;

use tof_control::helper::rb_type::{RBInfo, RBTemp, RBVcp, RBPh, RBMag};
use tof_control::rb_control::{rb_init, rb_mode};

#[derive(Parser, Debug)]
#[command(author = "Takeru Hayashi", version = "0.1.0", about, long_about = None)]
struct Cli {
    #[arg(short = 'b', long = "board", help = "Board to operate (rb, pb, ltb, or preamp)")]
    board: Board,
    // #[clap(subcommand)]
    // command: Commands,
    #[arg(short, long, help = "Verbose mode")]
    verbose: bool,
}

// #[derive(Subcommand, Debug)]
// enum Commands {
//     #[clap(short_flag = 'r')]
//     Read {
//         #[arg(ignore_case = true, value_enum)]
//         sensor: Option<Sensor>,
//     },
//     #[clap(short_flag = 'm')]
//     Mode {
//         #[arg(ignore_case = true, value_enum)]
//         mode: Mode,
//     },
//     #[clap(short_flag = 'i')]
//     Initialize {
//     },
// }

#[derive(ValueEnum, Clone, Debug)]
enum Board {
    RB,
    LTB,
    PB,
    Preamp,
}

fn main() {
    let cli = Cli::parse();

    match cli.board {
        Board::RB {}
    }
}