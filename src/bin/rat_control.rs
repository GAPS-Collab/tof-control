use clap::{Parser, ValueEnum};

use tof_control::helper::{
    rb_type::{RBMoniData, RBInfo},
    ltb_type::LTBMoniData,
};

#[derive(Parser, Debug)]
#[command(author = "Takeru Hayashi", version, about, long_about = None)]
struct Args {
    #[arg(short = 'b', long = "board", help = "Board to control (rb, pb, ltb, or preamp)")]
    board: Option<Board>,
    #[arg(long, help = "Print sensor data")]
    sensor: bool,
    #[arg(long, help = "Print in JSON format")]
    json: bool,
}

#[derive(ValueEnum, Clone, Debug)]
enum Board {
    RB,
    LTB,
    PB,
    PA,
}

fn main() {
    let args = Args::parse();

    let json = args.json;

    let sub_board = RBInfo::new().sub_board;

    if let Some(board) = &args.board {
        match board {
            Board::RB => {
                rb_handler(&args, json);
            }
            Board::LTB => {
                if sub_board != 1 {
                    println!("LTB is not connected.");
                    std::process::exit(0);
                } else {
                    ltb_handler(&args, json);
                }
            }
            Board::PB => {
                if sub_board != 2 {
                    println!("PB is not connected.");
                    std::process::exit(0);
                } else {
                    println!("PB");
                }
            }
            Board::PA => {
                if sub_board != 2 {
                    println!("Preamps are not connected.");
                    std::process::exit(0);
                } else {
                    println!("PA");
                }

            }
        }
    }
}

fn rb_handler(args: &Args, json: bool) {
    if args.sensor {
        let rb_moni_data = RBMoniData::new();
        if json {
            rb_moni_data.print_json();
        } else {
            rb_moni_data.print();
        }
    }
}

fn ltb_handler(args: &Args, json: bool) {
    if args.sensor {
        let ltb_moni_data = LTBMoniData::new();
        if json {
            ltb_moni_data.print_json();
        } else {
            ltb_moni_data.print();
        }
    }
}