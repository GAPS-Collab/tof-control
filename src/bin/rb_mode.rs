use clap::{Parser, Subcommand, ValueEnum};

use tof_control::rb_control::rb_mode;

#[derive(Parser, Debug)]
#[command(author = "Takeru Hayashi", version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
    #[arg(short, long, help = "Verbose mode")]
    verbose: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(short_flag = 's')]
    Set {
        #[arg(ignore_case = true, value_enum)]
        mode: Mode,
    },
    #[clap(short_flag = 'r')]
    Read {
    },
    Verify {
        #[arg(ignore_case = true, value_enum)]
        mode: Mode,
    },
}

#[derive(ValueEnum, Clone, Debug)]
enum Mode {
    NOI,
    VCAL,
    TCAL,
    SMA,
}
fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Set { mode } => {
            match mode {
                Mode::NOI => {
                    match rb_mode::select_noi_mode() {
                        Ok(_) => { println!("RB is now NOI mode.") }
                        Err(e) => { eprintln!("Mode select error: {}", e) }
                    }
                }
                Mode::VCAL => {
                    match rb_mode::select_vcal_mode() {
                        Ok(_) => { println!("RB is now VCAL mode.") }
                        Err(e) => { eprintln!("Mode select error: {}", e) }
                    }
                }
                Mode::TCAL => {
                    match rb_mode::select_tcal_mode() {
                        Ok(_) => { println!("RB is now TCAL mode.") }
                        Err(e) => { eprintln!("Mode select error: {}", e) }
                    }
                }
                Mode::SMA => {
                    match rb_mode::select_sma_mode() {
                        Ok(_) => { println!("RB is now SMA mode.") }
                        Err(e) => { eprintln!("Mode select error: {}", e) }
                    }
                }
            }
        }
        Commands::Read { } => {
            match rb_mode::read_input_mode() {
                Ok(mode) => {
                    println!("Current mode: {}", mode);
                }
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                }
            }
        }
        Commands::Verify { mode } => {
            match mode {
                Mode::NOI => {
                    match rb_mode::verify_input_mode("NOI") {
                        Ok(bool) => {
                            if bool {
                                println!("RB is in NOI mode.");
                            } else {
                                println!("RB is NOT in NOI mode.");
                            }
                        }
                        Err(e) => {
                            eprintln!("Mode verification error: {:?}", e);
                        }
                    }
                }
                Mode::VCAL => {
                    match rb_mode::verify_input_mode("VCAL") {
                        Ok(bool) => {
                            if bool {
                                println!("RB is in VCAL mode.");
                            } else {
                                println!("RB is NOT in VCAL mode.");
                            }
                        }
                        Err(e) => {
                            eprintln!("Mode verification error: {:?}", e);
                        }
                    }
                }
                Mode::TCAL => {
                    match rb_mode::verify_input_mode("TCAL") {
                        Ok(bool) => {
                            if bool {
                                println!("RB is in TCAL mode.");
                            } else {
                                println!("RB is NOT in TCAL mode.");
                            }
                        }
                        Err(e) => {
                            eprintln!("Mode verification error: {:?}", e);
                        }
                    }
                }
                Mode::SMA => {
                    match rb_mode::verify_input_mode("SMA") {
                        Ok(bool) => {
                            if bool {
                                println!("RB is in SMA mode.");
                            } else {
                                println!("RB is NOT in SMA mode.");
                            }
                        }
                        Err(e) => {
                            eprintln!("Mode verification error: {:?}", e);
                        }
                    }
                }
            }
        }
    }

    // if cli.read {
    //     match rb_mode::read_input_mode() {
    //         Ok(mode) => {
    //             println!("Current mode: {}", mode);
    //         }
    //         Err(e) => {
    //             eprintln!("Error: {:?}", e);
    //         }
    //     }
    //     std::process::exit(0);
    // }

    // if cli.verify {
    //     match rb_mode::verify_input_mode("SMA") {
    //         Ok(bool) => {
    //             if bool {
    //                 println!("RB is in the expected input mode.");
    //             } else {
    //                 println!("RB is NOT in the expected input mode.");
    //             }
    //         }
    //         Err(e) => {
    //             eprintln!("Mode verification error: {:?}", e);
    //         }
    //     }
    // }

    // if let Some(mode) = cli.set {
    //     match mode {
    //         Mode::NOI => {
    //             match rb_mode::select_noi_mode() {
    //                 Ok(_) => { println!("RB is now NOI mode.") }
    //                 Err(e) => { eprintln!("Mode select error: {}", e) }
    //             }
    //         }
    //         Mode::VCAL => {
    //             match rb_mode::select_vcal_mode() {
    //                 Ok(_) => { println!("RB is now VCAL mode.") }
    //                 Err(e) => { eprintln!("Mode select error: {}", e) }
    //             }
    //         }
    //         Mode::TCAL => {
    //             match rb_mode::select_tcal_mode() {
    //                 Ok(_) => { println!("RB is now TCAL mode.") }
    //                 Err(e) => { eprintln!("Mode select error: {}", e) }
    //             }
    //         }
    //         Mode::SMA => {
    //             match rb_mode::select_sma_mode() {
    //                 Ok(_) => { println!("RB is now SMA mode.") }
    //                 Err(e) => { eprintln!("Mode select error: {}", e) }
    //             }
    //         }
    //     }
    // } else {
    //     eprintln!("No argument is given.");
    //     eprintln!("\t-s/--set to set RB input mode from NOI, VCAL, TCAL, and SMA.");
    //     std::process::exit(-1);
    // }
}