use clap::Parser;

use tof_control::rb_control::rb_dac;

#[derive(Parser, Debug)]
#[command(author = "Takeru Hayashi", version, about, long_about = None)]
struct Cli {
    #[clap(short, long, help = "Set all DAC outputs to defatult values")]
    set: bool,
    #[clap(short, long, help = "Read all DAC output values")]
    read: bool,
    #[clap(long, help = "Reset all DAC outputs values to zero")]
    reset: bool,
    #[arg(short, long, help = "Verbose mode")]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.set {
        match rb_dac::set_dac() {
            Ok(_) => {
                println!("All DAC outputs are set to default values.")
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }
        std::process::exit(0);
    }else if cli.read {
        match rb_dac::read_dac() {
            Ok(dac_values) => {
                for (channel, dac) in dac_values.iter().enumerate() {
                    println!("DAC Channel {} Value: {}", channel, dac);
                }
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }
        std::process::exit(0);
    } else if cli.reset {
        match rb_dac::zero_dac() {
            Ok(_) => {
                println!("All DAC outputs are reset to 0mV.")
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }
        std::process::exit(0);
    } else {
        println!("No valid option provided. Use --help for more information.");
        std::process::exit(1);
    }
}