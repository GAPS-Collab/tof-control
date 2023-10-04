use tof_control::helper::rb_type::{RBGPIOeError, RBClkError};
use tof_control::rb_control::{rb_clk, rb_gpioe};

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long, help = "run verbose mode")]
    verbose: bool,
}

fn main() {

    let args = Args::parse();

    // Reset GPIO Expander
    if args.verbose { println!("Start resetting GPIO Expander...") };
    match reset_gpio(args.verbose) {
        Ok(_) => {
            if args.verbose { println!("Done resetting GPIO Expander") };
        }
        Err(e) => {
            eprintln!("Failed to reset GPIO Expander: {:?}", e);
            std::process::exit(1);
        }
    }

    // Reset Clock Synthesizer
    if args.verbose { println!("Start resetting clock synthesizer...") };
    match reset_clk_synth(args.verbose) {
        Ok(_) => {
            if args.verbose { println!("Done resetting clock synthesizer") };
        }
        Err(e) => {
            eprintln!("Failed to reset clock synthesizer: {:?}", e);
            std::process::exit(1);
        }
    }

    // Configure GPIO Expander
    if args.verbose { println!("Start configuring GPIO Expander...") };
    match program_gpioe(args.verbose) {
        Ok(_) => {
            if args.verbose { println!("Done configuring GPIO Expander") };
        }
        Err(e) => {
            eprintln!("Failed to configure GPIO Expander: {:?}", e);
            std::process::exit(1);
        }
    }

    // configure Clock Synthesizer
    if args.verbose { println!("Start configuring clock synthesizer...") };
    match program_clk_synth(args.verbose) {
        Ok(_) => {
            if args.verbose { println!("Done configuring clock synthesizer") };
        },
        Err(e) => {
            eprintln!("Failed to configure clock synthesizer: {:?}", e);
            std::process::exit(1);
        }
    }

    // Completion message
    if args.verbose { println!("Congratulations! RB was successfully programmed!") };
}

fn program_gpioe(verbose: bool) -> Result<(), RBGPIOeError> {
    // Initialize GPIO Expander
    if verbose { println!("Initializing GPIO Expander...") }
    rb_gpioe::initialize_gpioe()?;
    if verbose { println!("Done initializing GPIO Expander") }

    // Set SMA for DRS Input
    if verbose { println!("Setting SMA for DRS input...") }
    rb_gpioe::rf_input_select_gpioe(2)?;
    if verbose { println!("Done setting for DRS input") }

    // Enable clock synthesizer output
    if verbose { println!("Enabling clock synthesizer output...") }
    rb_gpioe::enable_si5345b_gpioe()?;
    if verbose { println!("Done enabling clock synthesizer output") }

    // Program EEPROM
    // Ask user to ensure to proceed
    let mut input = String::new();
    use std::io::Write;
    while input != "YES" {
        print!("Enter \"YES\" to proceed to program GPIO Expander EEPROM: ");
        let _ = std::io::stdout().flush();
        input.clear();
        std::io::stdin().read_line(&mut input).ok();
        input = input.trim().to_string();
    }
    if verbose { println!("Programming EEPROM...") }
    rb_gpioe::program_eeprom_gpioe()?;
    if verbose { println!("Done programming EEPROM") }

    Ok(())
}

fn reset_gpio(verbose: bool) -> Result<(), RBGPIOeError> {
    // Ask user to ensure to proceed
    let mut input = String::new();
    use std::io::Write;
    while input != "YES" {
        print!("Enter \"YES\" to proceed to reset GPIO Expander: ");
        let _ = std::io::stdout().flush();
        input.clear();
        std::io::stdin().read_line(&mut input).ok();
        input = input.trim().to_string();
    }
    // Reset current GPIOe configuration
    if verbose { println!("Resetting current GPIO Expander configuration...") }
    rb_gpioe::reset_gpioe()?;
    if verbose { println!("Done resetting GPIO Expander configuration") }

    // Reset currnet GPIOe EEPROM configuration
    if verbose { println!("Resetting current GPIO Expander EEPROM configuration...") }
    rb_gpioe::reset_eeprom_gpioe()?;
    if verbose { println!("Done resetting GPIO Expander EEPROM configuration") }

    Ok(())
}

fn program_clk_synth(verbose: bool) -> Result<(), RBClkError> {
    // Initialize Clock Synthesizer
    if verbose { println!("Initializing clock synthesizer...") }
    rb_clk::configure_clk_synth()?;
    if verbose { println!("Done initializing clock synthesizer") }

    // Program NVM
    // Ask user to ensure to proceed
    let mut input = String::new();
    use std::io::Write;
    while input != "YES" {
        print!("Enter \"YES\" to proceed to program clock synthesizer NVM: ");
        let _ = std::io::stdout().flush();
        input.clear();
        std::io::stdin().read_line(&mut input).ok();
        input = input.trim().to_string();
    }
    rb_clk::program_nvm_clk_synth(verbose)?;

    Ok(())
}

fn reset_clk_synth(verbose: bool) -> Result<(), RBClkError> {
    // Ask user to ensure to proceed
    let mut input = String::new();
    use std::io::Write;
    while input != "YES" {
        print!("Enter \"YES\" to proceed to reset clock synthesizer: ");
        let _ = std::io::stdout().flush();
        input.clear();
        std::io::stdin().read_line(&mut input).ok();
        input = input.trim().to_string();
    }
    // Reset current Clock Synthesizer configuration (Soft Reset)
    if verbose { println!("Resetting current clock synthesizer configuration...") }
    rb_clk::reset_clk_synth(0)?;
    if verbose { println!("Done resetting clock synthesizer configuration") }

    Ok(())
}