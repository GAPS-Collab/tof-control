mod constant;
mod device;
mod memory;
mod rb_control;
mod pb_control;
mod ltb_control;
mod preamp_control;

use clap::{Parser, ValueEnum, command};

use rb_control::*;
use pb_control::*;
use ltb_control::*;
use preamp_control::*;

#[derive(Debug, Parser)]
#[command(author = "Takeru Hayashi", version, about, long_about = None)]
struct Args {
    #[arg(short = 'b', long = "bd", help = "Board to operate (rb, pb, ltb, or preamp)")]
    bd: Board,
}

#[derive(Debug, Clone, ValueEnum)]
enum Board {
    RB,
    PB,
    LTB,
    Preamp,
}

fn main() {
    let args = Args::parse();
    match args.bd {
        Board::RB => RBTemp::print_rb_temp(),
        Board::PB => PBTemp::print_pb_temp(),
        Board::LTB => LTBTemp::print_ltb_temp(),
        Board::Preamp => PreampTemp::print_preamp_temp(),
        _ => println!("bad argument"),
    };
}
