use clap::{Parser, ValueEnum};

// use tof_control::helper::{
// }

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

    if let Some(board) = &args.board {
        match board {
            Board::RB => {
                rb_handler(&args, json);
            }
            Board::LTB => {
                println!("LTB");
            }
            Board::PB => {
                println!("PB");
            }
            Board::PA => {
                println!("PA");
            }
        }
    }
}

fn rb_handler(args: &Args, json: bool) {
    if args.sensor {
        if json {
            println!("Print Sensor in JSON");
        } else {
            println!("Print Sensor");
        }
    }
}