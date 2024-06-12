use clap::{Parser, ValueEnum};

use tof_control::RATMoniData;
use tof_control::helper::{
    rb_type::{RBMoniData, RBInfo},
    ltb_type::LTBMoniData,
    pb_type::PBMoniData,
    pa_type::{PAMoniData, PASetBias},
};

#[derive(Parser, Debug)]
#[command(author = "Takeru Hayashi", version, about, long_about = None)]
struct Args {
    #[arg(short='b', long="board", help="Board to control (rb, pb, ltb, or preamp)")]
    board: Option<Board>,
    #[arg(short='g', long="get", help="Get sensor data")]
    get: bool,
    #[arg(short='s', long="set", help="Set threshold for LTB or SiPM voltage for PA")]
    set: bool,
    #[arg(short='c', long="channel", help="Channels to set for LTB or PA")]
    channel: Vec<u8>,
    #[arg(short='v', long="voltage", value_delimiter=',', help="Voltages to set for LTB or PA")]
    voltage: Vec<f32>,
    #[arg(long, help="Set default values to the `-s/--set` command")]
    default: bool,
    #[arg(long, help="Reset values (0.0V) for LTB or SiPM voltage for PA")]
    reset: bool,
    #[arg(long, help="Print in JSON format")]
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
    if args.get {
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
        if args.get {
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
        if args.get {
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
        if args.get {
            let pa_moni_data = PAMoniData::new();
            if json {
                pa_moni_data.print_json();
            } else {
                pa_moni_data.print();
            }
        }
        // Set SiPM Bias Voltages
        if args.set {
            // Set Default SiPM Voltage (58.0V)
            if args.default {
                match PASetBias::set_default_bias() {
                    Ok(()) => {
                        std::process::exit(0);
                    },
                    Err(e) => {
                        eprintln!("PA SiPM Bias Voltage Set Error: {}", e);
                        std::process::exit(1);
                    }
                }
            }

            if args.voltage.len() == 1 {
                match PASetBias::set_manual_bias(None, args.voltage[0]) {
                    Ok(()) => {},
                    Err(e) => {
                        eprintln!("PA SiPM Bias Voltage Set Error: {}", e);
                        std::process::exit(1);
                    }
                }
            } else if args.voltage.len() == 16 {
                let mut sipm_voltages: [f32; 16] = Default::default();
                for (i, v) in args.voltage.iter().enumerate() {
                    sipm_voltages[i] = *v;
                }
                match PASetBias::set_manual_biases(sipm_voltages) {
                    Ok(()) => {},
                    Err(e) => {
                        eprintln!("PA SiPM Bias Voltage Set Error: {}", e);
                        std::process::exit(1);
                    }
                }
            } else {
                eprintln!("Lenght of SiPM voltages must be 1 or 16");
                std::process::exit(1);
            }
        }
    }
}

fn rat_handler(args: &Args, json: bool) {
    if args.get {
        let rat_moni_data = RATMoniData::new();
        if json {
            rat_moni_data.print_json();
        } else {
            rat_moni_data.print();
        }
    }
}