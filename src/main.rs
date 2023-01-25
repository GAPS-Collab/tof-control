mod constant;
mod device;
mod memory;
mod rb_control;
mod pb_control;
mod ltb_control;
mod preamp_control;

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
    #[arg(short = 'p', long = "print", action = clap::ArgAction::SetFalse, help = "Print the operation")]
    print: bool,
}

#[derive(Debug, Clone, ValueEnum)]
enum Board {
    RB,
    PB,
    LTB,
    Preamp,
}

#[derive(Debug, Subcommand)]
enum Action {
    Read {
        #[arg(short = 's', long = "sensor", required = true, ignore_case = true, help = "read sensor of the board")]
        sensor: Sensor,
    },
}

#[derive(Debug, Clone, ValueEnum)]
enum Sensor {
    TS,
    CS,
    PS,
    HS,
    MS,
    BIAS,
}

fn main() {
    let cli = Cli::parse();

    match cli.board {
        Board::RB => {
            match cli.action {
                Action::Read { sensor } => {
                    match sensor {
                        Sensor::TS => {
                            if cli.print {
                                RBTemp::print_rb_temp();
                            }
                        },
                        Sensor::CS => {
                            if cli.print {
                                RBvcp::print_rb_vcp();
                            }
                        }
                        Sensor::PS => {
                            if cli.print {
                                RBPress::print_rb_press();
                            }
                        },
                        Sensor::HS => {
                            if cli.print {
                                RBHum::print_rb_hum();
                            }
                        },
                        Sensor::MS => {
                            if cli.print {
                                RBMagnetic::print_rb_magnetic();
                            }
                        }
                        _ => println!("bad argument"),
                    }
                }
            }
        },
        Board::PB => {
            match cli.action {
                Action::Read { sensor } => {
                    match sensor {
                        Sensor::TS => {
                            if cli.print {
                                PBTemp::print_pb_temp();
                            }
                        },
                        _ => println!("bad argument"),
                    }
                }
            }
        },
        Board::LTB => {
            match cli.action {
                Action::Read { sensor } => {
                    match sensor {
                        Sensor::TS => {
                            if cli.print {
                                LTBTemp::print_ltb_temp();
                            }
                        },
                        _ => println!("bad argument"),
                    }
                }
            }
        },
        Board::Preamp => {
            match cli.action {
                Action::Read { sensor } => {
                    match sensor {
                        Sensor::TS => {
                            if cli.print {
                                PreampTemp::print_preamp_temp();
                            }
                        },
                        Sensor::BIAS => {
                            if cli.print {
                                PreampBiasRead::print_preamp_bias();
                            }
                        },
                        _ => println!("bad argument"),
                    }
                }
            }
        }
        // Board::PB => PBTemp::print_pb_temp(),
        // Board::LTB => LTBTemp::print_ltb_temp(),
        // Board::Preamp => PreampTemp::print_preamp_temp(),
        // _ => println!("bad argument"),
    };
}
