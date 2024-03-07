use clap::Parser;

use tof_control::helper::switch_type::AllSwitchData;
use tof_control::switch_control;

#[derive(Parser, Debug)]
#[command(author = "Takeru Hayashi", version, about, long_about = None)]
struct Cli {
    #[clap(short, long, help = "Get Value")]
    get: bool,
    #[clap(short, long, help = "Set Value")]
    clear: bool,
    #[arg(short, long, help = "Verbose mode")]
    verbose: bool,
}
fn main() {
    let cli = Cli::parse();

    if cli.get {
        AllSwitchData::print_all_switch();
    } else if cli.clear {
        match switch_control::clear_port_statistics_all() {
            Ok(_) => {
                println!("Port Statistics Cleared");
            }
            Err(e) => {
                eprintln!("Failed clearing port statistics: {:?}", e);
            }
        }
    } else {
        println!("No argument is given.");
        println!("\t-g/--get to display the statistics");
        println!("\t-c/--clear to clear the statistics");
    }
}