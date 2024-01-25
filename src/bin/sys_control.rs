use clap::{Parser, ValueEnum};

use tof_control::helper::cpu_type::{CPUInfo, CPUTemp, CPUTempDebug};

#[derive(Parser, Debug)]
#[command(author = "Takeru Hayashi", version, about, long_about = None)]
struct Cli {
    #[arg(short = 'b', long = "board", help = "Board to operate (cpu, mtb, or switch)")]
    board: Board,
    #[clap(short, long, help = "Set Value")]
    set: Option<f32>,
    #[clap(short, long, help = "Get Value")]
    get: bool,
    #[arg(short, long, help = "Verbose mode")]
    verbose: bool,
}


#[derive(ValueEnum, Clone, Debug)]
enum Board {
    CPU,
    MTB,
    SWITCH,
}

fn main() {
    let cli = Cli::parse();

    let board = &cli.board;

    match board {
        Board::CPU => {
            if cli.get {
                get_cpu_info();
            }
        }
        Board::MTB => {
            println!("MTB");
        }
        Board::SWITCH => {
            println!("SWITCH");
        }
    }
}

fn get_cpu_info() {
    let cpu_info = CPUInfo::new();

    println!("CPU Information");
    println!("\tUptime              : {}[s]", cpu_info.uptime);
    println!("\tCPU0 Frequency      : {}[Hz]", cpu_info.cpu_freq[0]);
    println!("\tCPU1 Frequency      : {}[Hz]", cpu_info.cpu_freq[1]);
    println!("\tCPU2 Frequency      : {}[Hz]", cpu_info.cpu_freq[2]);
    println!("\tCPU3 Frequency      : {}[Hz]", cpu_info.cpu_freq[3]);
}