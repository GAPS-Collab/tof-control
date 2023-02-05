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
    Set {

    },
    Initialize {

    },
    Reset {

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
    GPIOE,
    INITIALIZE_GPIOE,
    NO_INPUT_MODE,
    TCAL_MODE,
    NORMAL_MODE,
    EN_TCAL,
    DIS_TCAL,
    DAC,
    LTB_PWR_ON,
    LTB_PWR_OFF,
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
                        },
                        Sensor::GPIOE => {
                            if cli.print {
                                RBgpioe::print_rb_gpioe();
                            }
                        },
                        Sensor::INITIALIZE_GPIOE => {
                            if cli.print {
                                RBgpioe::initialize();
                            }
                        },
                        Sensor::NO_INPUT_MODE => {
                            if cli.print {
                                RBgpioe::set_rf_switch(0);
                            }
                        },
                        Sensor::TCAL_MODE => {
                            if cli.print {
                                RBgpioe::set_rf_switch(1);
                                RBgpioe::enable_tcal_clock(1);
                            }
                        },
                        Sensor::NORMAL_MODE => {
                            if cli.print {
                                RBgpioe::set_rf_switch(2);
                            }
                        },
                        Sensor::EN_TCAL => {
                            if cli.print {
                                RBgpioe::enable_tcal_clock(1);
                            }
                        },
                        Sensor::DIS_TCAL => {
                            if cli.print {
                                RBgpioe::disable_tcal_clock();
                            }
                        }
                        Sensor::DAC => {
                            if cli.print {
                                RBdac::print_rb_dac();
                            }
                        }
                        _ => println!("bad argument"),
                    }
                },
                Action::Set { } => {
                    todo!();
                },
                Action::Initialize {  } => {
                    rb_control::initialize();
                },
                Action::Reset { } => {
                    todo!();
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
                        },
                        Sensor::DAC => {
                            if cli.print {
                                PBdac::print_pb_dac();
                            }
                        },
                        Sensor::LTB_PWR_ON => {
                            if cli.print {
                                LTBpwr::power_on_ltb();
                            }
                        },
                        Sensor::LTB_PWR_OFF => {
                            if cli.print {
                                LTBpwr::power_off_ltb();
                            }
                        }
                        _ => println!("bad argument"),
                    }
                },
                Action::Set { } => {
                    todo!();
                },
                Action::Initialize {  } => {
                    pb_control::initialize();
                },
                Action::Reset { } => {
                    todo!();
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
                        Sensor::DAC => {
                            if cli.print {
                                LTBdac::print_ltb_dac();
                            }
                        }
                        _ => println!("bad argument"),
                    }
                },
                Action::Set { } => {
                    LTBdac::set_threshold();
                },
                Action::Initialize {  } => {
                    todo!();
                },
                Action::Reset { } => {
                    LTBdac::reset_threshold();
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
                Action::Set { } => {
                    PreampBiasSet::set_bias();
                },
                Action::Initialize {  } => {
                    todo!();
                },
                Action::Reset { } => {
                    PreampBiasSet::reset_bias();
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
