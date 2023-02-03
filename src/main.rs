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

use clap::{Parser, ArgGroup, ValueEnum, Subcommand, command};

use rb_control::*;
use pb_control::*;
use ltb_control::*;
use preamp_control::*;

#[derive(Debug, Parser)]
#[command(author = "Takeru Hayashi", version, about, long_about = None)]
struct Cli {
    #[arg(short = 'b', long = "board", help = "Board to operate (rb, pb, ltb, or preamp)")]
    board: Board,
    #[clap(subcommand)]
    action: Action,
    #[arg(short = 'p', long = "print", help = "print output of operation")]
    print: bool,
}

#[derive(Debug, Clone, ValueEnum)]
enum Board {
    ALL,
    RB,
    PB,
    LTB,
    Preamp,
}

#[derive(Debug, Subcommand)]
enum Action {
    Read {
        // #[arg(short = 's', long = "sensor", required = true, ignore_case = true, help = "read sensor of the board")]
        // sensor: Sensor,
    },
    Set {

    },
    Initialize {

    },
    Reset {

    },
}

fn main() {
    let cli = Cli::parse();

    match cli.board {
        Board::RB => {
            match cli.action {
                Action::Initialize {  } => {
                    Command::new("rbsetup").arg("-i").arg("2").stdout(Stdio::null()).spawn().expect("failed to edxecute rbsetup command");
                    thread::sleep(Duration::from_secs(3));
                    rb_control::initialize();
                    if cli.print {
                        println!("RB is initialized.");
                    }
                },
                Action::Read {  } => {
                    println!("--- RB Clock Synthesizer ---");
                    RBclk::print_rb_clk();
                    println!("--- RB Temperature ---");
                    RBtemperature::print_rb_temp();
                    println!("--- RB Humidity ---");
                    RBhumidity::print_rb_hum();
                    println!("--- RB Pressure ---");
                    RBpressure::print_rb_press();
                    println!("--- RB Voltage, Current and Power ---");
                    RBvcp::print_rb_vcp();
                    println!("--- RB Magnetic Field ---");
                    RBmagnetic::print_rb_magnetic();
                },
                Action::Set {  } => {
                    Command::new("rbsetup").arg("-i").arg("2").stdout(Stdio::null()).spawn().expect("failed to edxecute rbsetup command");
                    thread::sleep(Duration::from_secs(3));
                    if cli.print {
                        println!("RB is set.");
                    }
                },
                Action::Reset {  } => {
                    Command::new("rbsetup").arg("-i").arg("2").stdout(Stdio::null()).spawn().expect("failed to edxecute rbsetup command");
                    thread::sleep(Duration::from_secs(3));
                    if cli.print {
                        println!("RB is reset.");
                    }
                }
                _ => {
                    println!("bad argument");
                },
            }
        },
        Board::PB => {
            match cli.action {
                Action::Initialize {  } => {
                    pb_control::initialize();
                    if cli.print {
                        println!("PB is initialized.");
                    }
                },
                Action::Read {  } => {
                    println!("--- PB Temperature ---");
                    pb_control::PBtemperature::print_pb_temp();
                    println!("--- RB Voltage, Current and Power ---");
                    pb_control::PBvcp::print_pb_vcp();
                },
                _ => {
                    println!("bad argument");
                },
            }
        },
        Board::LTB => {
            match cli.action {
                Action::Initialize {  } => {
                    ltb_control::initialize();
                    if cli.print {
                        println!("LTB is initialized.");
                    }
                },
                Action::Read {  } => {
                    println!("--- LTB Temperature ---");
                    ltb_control::LTBtemperature::print_ltb_temp();
                    println!("--- LTB Threshold ---");
                    ltb_control::LTBdac::print_ltb_dac();
                },
                Action::Set {  } => {
                    ltb_control::LTBdac::set_threshold();
                },
                Action::Reset {  } => {
                    ltb_control::LTBdac::reset_threshold();
                },
                _ => {
                    println!("bad argument");
                },
            }
        },
        Board::Preamp => {
            match cli.action {
                Action::Initialize {  } => {
                    preamp_control::initialize();
                    if cli.print {
                        println!("Preamps are initialized.");
                    }
                },
                Action::Read {  } => {
                    println!("--- Preamp Board Temperature ---");
                    preamp_control::PreampTemp::print_preamp_temp();
                    println!("--- Preamp Board Bias Voltage ---");
                    preamp_control::PreampBiasRead::print_preamp_bias();
                },
                Action::Set {  } => {
                    preamp_control::PreampBiasSet::set_bias();
                }
                Action::Reset {  } => {
                    preamp_control::PreampBiasSet::reset_bias();
                },
                _ => {
                    println!("bad argument");
                },
            }
        },
        Board::ALL => {
            match cli.action {
                Action::Initialize {  } => {
                    Command::new("rbsetup").arg("-i").arg("2").stdout(Stdio::null()).spawn().expect("failed to edxecute rbsetup command");
                    thread::sleep(Duration::from_secs(3));
                    rb_control::initialize();
                    if cli.print {
                        println!("RB is initialized.");
                    }
                    pb_control::initialize();
                    if cli.print {
                        println!("PB is initialized.");
                    }
                    ltb_control::initialize();
                    if cli.print {
                        println!("LTB is initialized.");
                    }
                    preamp_control::initialize();
                    if cli.print {
                        println!("Preamps are initialized.");
                    }
                },
                _ => {
                    println!("bad argument");
                },
            }
        }
    }
}
