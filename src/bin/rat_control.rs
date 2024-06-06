use clap::{Parser, ValueEnum};

use tof_control::RATMoniData;
use tof_control::helper::{
    rb_type::{RBMoniData, RBInfo},
    ltb_type::LTBMoniData,
    pb_type::PBMoniData,
    pa_type::PAMoniData,
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
                ltb_handler(&args, json, sub_board);
            }
            Board::PB => {
                pb_handler(&args, json, sub_board);
            }
            Board::PA => {
                pa_handler(&args, json, sub_board);
            }
        }
    } else {
        rat_handler(&args, json);
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

fn ltb_handler(args: &Args, json: bool, sub_board: u8) {
    if sub_board != 1 {
        println!("LTB is not connected.");
        std::process::exit(0);
    } else {
        if args.sensor {
            let ltb_moni_data = LTBMoniData::new();
            if json {
                ltb_moni_data.print_json();
            } else {
                ltb_moni_data.print();
            }
        }
    }
}

fn pb_handler(args: &Args, json: bool, sub_board: u8) {
    if sub_board != 2 {
        println!("PB is not connected.");
        std::process::exit(0);
    } else {
        if args.sensor {
            let pb_moni_data = PBMoniData::new();
            if json {
                pb_moni_data.print_json();
            } else {
                pb_moni_data.print();
            }
        }
    }
}

fn pa_handler(args: &Args, json: bool, sub_board: u8) {
    if sub_board != 2 {
        println!("Preamps are not connected.");
        std::process::exit(0);
    } else {
        if args.sensor {
            let pa_moni_data = PAMoniData::new();
            if json {
                pa_moni_data.print_json();
            } else {
                pa_moni_data.print();
            }
        }
    }
}

fn rat_handler(args: &Args, json: bool) {
    if args.sensor {
        let rat_moni_data = RATMoniData::new();
        if json {
            rat_moni_data.print_json();
        } else {
            rat_moni_data.print();
        }
    }
}