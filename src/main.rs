mod constant;
mod device;
mod memory;
mod rb_control;
// mod pb_control;
// mod ltb_control;

use clap::{Parser, ValueEnum, command};

use rb_control::*;
// use pb_control::*;
// use ltb_control::*;

#[derive(Debug, Parser)]
#[command(author = "Takeru Hayashi", version, about, long_about = None)]
struct Args {
    #[arg(short = 'b', long = "bd", help = "Board to operate (rb, pb, ltb, or preamp)")]
    bd: Board,
}

#[derive(Debug, Clone, ValueEnum)]
enum Board {
    rb,
    pb,
    ltb,
    preamp,
}

fn main() {
    let args = Args::parse();
    match args.bd {
        Board::rb => RBTemp::print_rb_temp(),
        // Board::pb => PBTemp::print_pb_temp(),
        // Board::ltb => LTBTemp::print_ltb_temp(),
        _ => println!("bad argument"),
    };
}
