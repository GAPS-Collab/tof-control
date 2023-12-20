use clap::{ArgGroup, Parser};

use tof_control::rb_control::rb_clk;

#[derive(Parser, Debug)]
#[command(author = "Takeru Hayashi", version = "0.1.0", about, long_about = None)]
#[command(group(ArgGroup::new("RB Clock Synthesizer Reset").required(true).args(["hard", "soft"])))]
struct Args {
    #[arg(short, long, help = "Hard Reset")]
    hard: bool,
    #[arg(short, long, help = "Soft Reset")]
    soft: bool,
}

fn main() {

    let args = Args::parse();

    if args.hard {
        match rb_clk::reset_clk_synth(1) {
            Ok(_) => {
                println!("Clock Synthesizer was successfully reset (hard reset).")
            }
            Err(e) => {
                eprintln!("Error on resetting clock synthesizer (hard reset): {:?}", e)
            }
        }
    } else {
        match rb_clk::reset_clk_synth(0) {
            Ok(_) => {
                println!("Clock Synthesizer was successfully reset (soft reset).")
            }
            Err(e) => {
                eprintln!("Error on resetting clock synthesizer (soft reset): {:?}", e)
            }
        }
    }
}