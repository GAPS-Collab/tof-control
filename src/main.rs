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
    Initialize {

    },
    Clock {

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
    CLK,
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
                                RBtemperature::print_rb_temp();
                            }
                        },
                        Sensor::CS => {
                            if cli.print {
                                RBvcp::print_rb_vcp();
                            }
                        },
                        Sensor::PS => {
                            if cli.print {
                                RBpressure::print_rb_press();
                            }
                        },
                        Sensor::HS => {
                            if cli.print {
                                RBhumidity::print_rb_hum();
                            }
                        },
                        Sensor::MS => {
                            if cli.print {
                                RBmagnetic::print_rb_magnetic();
                            }
                        },
                        Sensor::CLK => {
                            if cli.print {
                                RBclk::print_rb_clk();
                            }
                        }
                        _ => println!("bad argument"),
                    }
                },
                Action::Initialize {  } => {
                    rb_control::initialize();
                },
                Action::Clock {  } => {
                    RBclk::print_config();
                },
            }
        },
        Board::PB => {
            match cli.action {
                Action::Read { sensor } => {
                    match sensor {
                        Sensor::TS => {
                            if cli.print {
                                PBtemperature::print_pb_temp();
                            }
                        },
                        Sensor::CS => {
                            if cli.print {
                                PBvcp::print_pb_vcp();
                            }
                        }
                        _ => println!("bad argument"),
                    }
                },
                Action::Initialize {  } => {
                    pb_control::initialize();
                },
                Action::Clock {  } => {
                   todo!();
                },
            }
        },
        Board::LTB => {
            match cli.action {
                Action::Read { sensor } => {
                    match sensor {
                        Sensor::TS => {
                            if cli.print {
                                LTBtemperature::print_ltb_temp();
                            }
                        },
                        _ => println!("bad argument"),
                    }
                },
                Action::Initialize {  } => {
                    todo!();
                },
                Action::Clock {  } => {
                    todo!();
                 },
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
                },
                Action::Initialize {  } => {
                    todo!();
                },
                Action::Clock {  } => {
                    todo!();
                 },
            }
        }
        // Board::PB => PBTemp::print_pb_temp(),
        // Board::LTB => LTBTemp::print_ltb_temp(),
        // Board::Preamp => PreampTemp::print_preamp_temp(),
        // _ => println!("bad argument"),
    };
}
