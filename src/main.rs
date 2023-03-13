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
    Init {

    },
    Reset {

    },
    GPIO {

    },
}

fn main() {
    let cli = Cli::parse();

    match cli.board {
        Board::RB => {
            match cli.action {
                Action::Init {  } => {
                    Command::new("rbsetup").arg("-i").arg("2").stdout(Stdio::null()).spawn().expect("failed to edxecute rbsetup command");
                    thread::sleep(Duration::from_secs(3));
                    rb_init::initialize();
                    if cli.print {
                        println!("RB is initialized.");
                    }
                },
                Action::Read {  } => {
                    println!("--- RB Information ---");
                    rb_info::RBinfo::print_rb_info();
                    println!("--- RB Clock Synthesizer ---");
                    rb_clk::RBclk::print_rb_clk();
                    println!("--- RB Temperature ---");
                    rb_temp::RBtemp::print_rb_temp();
                    println!("--- RB Pressure & Humidity ---");
                    rb_ph::RBph::print_rb_ph();
                    println!("--- RB Magnetic Field ---");
                    rb_mag::RBmag::print_rb_magnetic();
                    println!("--- RB Voltage, Current and Power ---");
                    rb_vcp::RBvcp::print_rb_vcp();
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
                },
                Action::GPIO {  } => {
                    // RBgpioe::print_rb_gpioe();
                    rb_gpioe::RBgpioe::print_rb_gpioe();
                    // RBdac::dac_test();
                }
                _ => {
                    println!("bad argument");
                },
            }
        },
        Board::PB => {
            match cli.action {
                Action::Init {  } => {
                    pb_init::initialize();
                    if cli.print {
                        println!("PB is initialized.");
                    }
                },
                Action::Read {  } => {
                    println!("--- PB Temperature ---");
                    pb_temp::PBtemp::print_pb_temp();
                    println!("--- PB Voltage, Current and Power ---");
                    pb_vcp::PBvcp::print_pb_vcp();
                },
                _ => {
                    println!("bad argument");
                },
            }
        },
        Board::LTB => {
            match cli.action {
                Action::Init {  } => {
                    ltb_init::initialize();
                    if cli.print {
                        println!("LTB is initialized.");
                    }
                },
                Action::Read {  } => {
                    println!("--- LTB Temperature ---");
                    ltb_temp::LTBtemp::print_ltb_temp();
                    println!("--- LTB Threshold ---");
                    ltb_dac::LTBdac::print_ltb_dac();
                },
                Action::Set {  } => {
                    ltb_dac::LTBdac::set_threshold();
                },
                Action::Reset {  } => {
                    ltb_dac::LTBdac::reset_threshold();
                },
                _ => {
                    println!("bad argument");
                },
            }
        },
        Board::Preamp => {
            match cli.action {
                Action::Init {  } => {
                    preamp_init::initialize();
                    if cli.print {
                        println!("Preamps are initialized.");
                    }
                },
                Action::Read {  } => {
                    println!("--- Preamp Board Temperature ---");
                    preamp_temp::PreampTemp::print_preamp_temp();
                    println!("--- Preamp Board Bias Voltage ---");
                    preamp_bias::PreampBiasRead::print_preamp_bias();
                },
                Action::Set {  } => {
                    preamp_bias::PreampBiasSet::set_bias();
                }
                Action::Reset {  } => {
                    preamp_bias::PreampBiasSet::reset_bias();
                },
                _ => {
                    println!("bad argument");
                },
            }
        },
        Board::ALL => {
            match cli.action {
                Action::Init {  } => {
                    Command::new("rbsetup").arg("-i").arg("2").stdout(Stdio::null()).spawn().expect("failed to edxecute rbsetup command");
                    thread::sleep(Duration::from_secs(3));
                    rb_init::initialize();
                    if cli.print {
                        println!("RB is initialized.");
                    }
                    pb_init::initialize();
                    if cli.print {
                        println!("PB is initialized.");
                    }
                    ltb_init::initialize();
                    if cli.print {
                        println!("LTB is initialized.");
                    }
                    preamp_init::initialize();
                    if cli.print {
                        println!("Preamps are initialized.");
                    }
                },
                Action::Read {  } => {
                    println!("########## Readout Board Sensors ########################");
                    println!("");
                    println!("--- RB Information ---");
                    rb_info::RBinfo::print_rb_info();
                    println!("--- RB Clock Synthesizer ---");
                    rb_clk::RBclk::print_rb_clk();
                    println!("--- RB Temperature ---");
                    rb_temp::RBtemp::print_rb_temp();
                    println!("--- RB Pressure & Humidity ---");
                    rb_ph::RBph::print_rb_ph();
                    println!("--- RB Magnetic Field ---");
                    rb_mag::RBmag::print_rb_magnetic();
                    println!("--- RB Voltage, Current and Power ---");
                    rb_vcp::RBvcp::print_rb_vcp();
                    println!("");
                    println!("########## Power Board Sensors ##########################");
                    println!("");
                    println!("--- PB Temperature ---");
                    pb_temp::PBtemp::print_pb_temp();
                    println!("--- PB Voltage, Current and Power ---");
                    pb_vcp::PBvcp::print_pb_vcp();
                    println!("");
                    println!("########## Local Trigger Board Sensors ##################");
                    println!("");
                    println!("--- LTB Temperature ---");
                    ltb_temp::LTBtemp::print_ltb_temp();
                    println!("--- LTB Threshold ---");
                    ltb_dac::LTBdac::print_ltb_dac();
                    println!("");
                    println!("########## Preamp Board Sensors #########################");
                    println!("");
                    println!("--- Preamp Board Temperature ---");
                    preamp_temp::PreampTemp::print_preamp_temp();
                    println!("--- Preamp Board Bias Voltage ---");
                    preamp_bias::PreampBiasRead::print_preamp_bias();

                }
                _ => {
                    println!("bad argument");
                },
            }
        }
    }
}
