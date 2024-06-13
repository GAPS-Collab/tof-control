use clap::{Parser, ValueEnum};

use tof_control::RATMoniData;
use tof_control::ltb_control::ltb_threshold;
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
    #[arg(short='c', long="channel", value_delimiter=',', help="Channels to set for LTB or PA")]
    channel: Vec<u8>,
    #[arg(short='v', long="voltage", value_delimiter=',', help="Voltages to set for LTB or PA")]
    voltage: Vec<f32>,
    #[arg(long, help="Set default values for LTB or SiPM voltage for PA")]
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

        // Set Default Threshold Voltages (0: 40.0mV, 1: 32.0mV, 2: 375.0mV)
        if args.default {
            match ltb_threshold::set_default_threshold() {
                Ok(()) => {
                    std::process::exit(0);
                },
                Err(e) => {
                    eprintln!("LTB Threshold Voltage Set Error: {}", e);
                    std::process::exit(1);
                }
            }
        }

        // Reset Threshold Voltage (0.0mV) for All 3 Thresholds
        if args.reset {
            match ltb_threshold::reset_threshold() {
                Ok(()) => {
                    std::process::exit(0);
                },
                Err(e) => {
                    eprintln!("LTB Threshold Voltage Reset Error: {}", e);
                    std::process::exit(1);
                }
            }
        }

        // Set Threshold Voltages
        if args.set {
            if args.channel.len() == 1 {
                // Set Threshold Voltage for Given Channel
                if args.voltage.len() == 1 {
                    match ltb_threshold::set_threshold(args.channel[0], args.voltage[0]) {
                        Ok(()) => {
                            std::process::exit(0);
                        },
                        Err(e) => {
                            eprintln!("LTB Threshold Voltage Set Error: {}", e);
                            std::process::exit(1);
                        }
                    }
                } else {
                    eprintln!("Lenght of threshold voltage must be 1");
                    std::process::exit(1);
                }
            } else if args.channel.len() > 1 {
                eprintln!("Lenght of threshold channel must be 1");
                std::process::exit(1);
            } else {
                if args.voltage.len() == 3 {
                    // Set Threshold Voltages for All 3 Thresholds Simultaneously
                    let mut threshold_voltages: [f32; 3] = Default::default();
                    for (i, v) in args.voltage.iter().enumerate() {
                        threshold_voltages[i] = *v;
                    }
                    match ltb_threshold::set_thresholds(threshold_voltages) {
                        Ok(()) => {
                            std::process::exit(0);
                        },
                        Err(e) => {
                            eprintln!("LTB Threshold Voltage Set Error: {}", e);
                            std::process::exit(1);
                        }
                    }
                } else if args.voltage.len() == 1 {
                    //Set Same Threshold Voltage for All 3 Thresholds Simultaneously
                    let threshold_voltages: [f32; 3] = [args.voltage[0], args.voltage[0], args.voltage[0]];
                    match ltb_threshold::set_thresholds(threshold_voltages) {
                        Ok(()) => {
                            std::process::exit(0);
                        },
                        Err(e) => {
                            eprintln!("LTB Threshold Voltage Set Error: {}", e);
                            std::process::exit(1);
                        }
                    }
                } else {
                    eprintln!("Lenght of threshold voltages must be 1 or 3");
                    std::process::exit(1);
                }
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

        // Set Default SiPM Bias Voltage (58.0V) for All 16 Preamps 
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

        // Reset SiPM Bias Voltage to 0.0V for All 16 Preamps
        if args.reset {
            match PASetBias::reset_bias() {
                Ok(()) => {
                    std::process::exit(0);
                },
                Err(e) => {
                    eprintln!("PA SiPM Bias Voltage Reset Error: {}", e);
                    std::process::exit(1);
                }
            }
        }

        // Set SiPM Bias Voltages
        if args.set {
            if args.voltage.len() == 1 {
                match PASetBias::set_manual_bias(None, args.voltage[0]) {
                    Ok(()) => {
                        std::process::exit(0);
                    },
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
                    Ok(()) => {
                        std::process::exit(0);
                    },
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