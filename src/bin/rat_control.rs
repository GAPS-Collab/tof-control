use clap::{Parser, ValueEnum};

use tof_control::helper::{
    rb_type::RBInfo,
    ltb_type::{LTBTemp, LTBThreshold},
    preamp_type::{PreampTemp, PreampSetBias, PreampReadBias},
};
use tof_control::ltb_control::{ltb_threshold, ltb_init};
use tof_control::preamp_control::preamp_init;

#[derive(Parser, Debug)]
#[command(author = "Takeru Hayashi", version, about, long_about = None)]
struct Cli {
    #[arg(short = 'b', long = "board", help = "Board to operate (rb, pb, ltb, or preamp)")]
    board: Board,
    #[clap(short, long, help = "Show status of board")]
    init: bool,
    #[clap(short, long, help = "Set Value")]
    set: Option<f32>,
    #[clap(short, long, help = "Get Value")]
    get: bool,
    #[arg(short, long, help = "Verbose mode")]
    verbose: bool,
}

#[derive(ValueEnum, Clone, Debug)]
enum Board {
    RB,
    LTB,
    PB,
    Preamp,
}

fn main() {
    let cli = Cli::parse();

    let board = &cli.board;

    let sub_board = RBInfo::new().sub_board;

    match board {
        Board::RB => {
            println!("RB funcstions are not implemented yet.");
        }
        Board::LTB => {
            if sub_board != 1 {
                println!("LTB is not connected.");
                std::process::exit(0);
            }
            if let Some(voltage) = cli.set {
                for i in 0..3 {
                    match ltb_threshold::set_threshold(i, voltage) {
                        Ok(_) => {},
                        Err(e) => {
                            eprintln!("{:?}", e);
                            std::process::exit(1);
                        }
                    }
                }
                println!("All thresholds are set to {:.1} mV.", voltage);
            }
            if cli.init {
                match ltb_init::initialize() {
                    Ok(_) => {},
                    Err(e) => eprintln!("{:?}", e),
                }
                println!("All thresholds are initialized.");
                println!("\tThreshold 0:        40.0[mV]");
                println!("\tThreshold 1:        32.0[mV]");
                println!("\tThreshold 2:        375.0[mV]");
            }
            if cli.get {
                get_ltb_sensor();
            }
        }
        Board::PB => {
            if sub_board != 2 {
                println!("PB is not connected.");
                std::process::exit(0);
            }
            println!("PB funcstions are not implemented yet.");
        }
        Board::Preamp => {
            if sub_board != 2 {
                println!("Preamps are not connected.");
                std::process::exit(0);
            }
            if let Some(voltage) = cli.set {
                match PreampSetBias::set_manual_bias(None, voltage) {
                    Ok(_) => {},
                    Err(e) => {
                        eprintln!("{:?}", e);
                        std::process::exit(1);
                    }
                }
                println!("All SiPM bias voltages are set to {:.1} mV.", voltage);
            }
            if cli.init {
                match PreampSetBias::set_manual_bias(None, 0.0) {
                    Ok(_) => {},
                    Err(e) => {
                        eprintln!("{:?}", e);
                        std::process::exit(1);
                    }
                }
                match preamp_init::initialize() {
                    Ok(_) => {},
                    Err(e) => {
                        eprintln!("{:?}", e);
                        std::process::exit(1);
                    }
                }
                println!("All SiPM bias are initialized (58.0 V with temperature compensated).");
            }
            if cli.get {
                get_preamp_sensor();
            }
        }
    }
}

fn get_ltb_sensor() {
    let ltb_temp = LTBTemp::new();
    let ltb_threshold = LTBThreshold::new();

    println!("LTB Temperature");
    println!("\tTrenz Temp:             {:.3}[°C]", ltb_temp.trenz_temp);
    println!("\tBoard Temp:             {:.3}[°C]", ltb_temp.board_temp);

    println!("LTB Threshold");
    println!("\tThreshold 0:            {:.3}[mV]", ltb_threshold.thresholds[0]);
    println!("\tThreshold 1:            {:.3}[mV]", ltb_threshold.thresholds[1]);
    println!("\tThreshold 2:            {:.3}[mV]", ltb_threshold.thresholds[2]);
}

fn get_preamp_sensor() {
    let preamp_temp = PreampTemp::new();
    let preamp_bias = PreampReadBias::new();

    println!("Preamp Temperature");
    println!("\tPreamp 1 Temp:          {:.3}[°C]", preamp_temp.preamp_temps[0]);
    println!("\tPreamp 2 Temp:          {:.3}[°C]", preamp_temp.preamp_temps[1]);
    println!("\tPreamp 3 Temp:          {:.3}[°C]", preamp_temp.preamp_temps[2]);
    println!("\tPreamp 4 Temp:          {:.3}[°C]", preamp_temp.preamp_temps[3]);
    println!("\tPreamp 5 Temp:          {:.3}[°C]", preamp_temp.preamp_temps[4]);
    println!("\tPreamp 6 Temp:          {:.3}[°C]", preamp_temp.preamp_temps[5]);
    println!("\tPreamp 7 Temp:          {:.3}[°C]", preamp_temp.preamp_temps[6]);
    println!("\tPreamp 8 Temp:          {:.3}[°C]", preamp_temp.preamp_temps[7]);
    println!("\tPreamp 9 Temp:          {:.3}[°C]", preamp_temp.preamp_temps[8]);
    println!("\tPreamp 10 Temp:         {:.3}[°C]", preamp_temp.preamp_temps[9]);
    println!("\tPreamp 11 Temp:         {:.3}[°C]", preamp_temp.preamp_temps[10]);
    println!("\tPreamp 12 Temp:         {:.3}[°C]", preamp_temp.preamp_temps[11]);
    println!("\tPreamp 13 Temp:         {:.3}[°C]", preamp_temp.preamp_temps[12]);
    println!("\tPreamp 14 Temp:         {:.3}[°C]", preamp_temp.preamp_temps[13]);
    println!("\tPreamp 15 Temp:         {:.3}[°C]", preamp_temp.preamp_temps[14]);
    println!("\tPreamp 16 Temp:         {:.3}[°C]", preamp_temp.preamp_temps[15]);

    println!("Preamp SiPM Bias Voltage");
    println!("\tPreamp 1 SiPM Bias:     {:.3}[V]", preamp_bias.read_biases[0]);
    println!("\tPreamp 2 SiPM Bias:     {:.3}[V]", preamp_bias.read_biases[1]);
    println!("\tPreamp 3 SiPM Bias:     {:.3}[V]", preamp_bias.read_biases[2]);
    println!("\tPreamp 4 SiPM Bias:     {:.3}[V]", preamp_bias.read_biases[3]);
    println!("\tPreamp 5 SiPM Bias:     {:.3}[V]", preamp_bias.read_biases[4]);
    println!("\tPreamp 6 SiPM Bias:     {:.3}[V]", preamp_bias.read_biases[5]);
    println!("\tPreamp 7 SiPM Bias:     {:.3}[V]", preamp_bias.read_biases[6]);
    println!("\tPreamp 8 SiPM Bias:     {:.3}[V]", preamp_bias.read_biases[7]);
    println!("\tPreamp 9 SiPM Bias:     {:.3}[V]", preamp_bias.read_biases[8]);
    println!("\tPreamp 10 SiPM Bias:    {:.3}[V]", preamp_bias.read_biases[9]);
    println!("\tPreamp 11 SiPM Bias:    {:.3}[V]", preamp_bias.read_biases[10]);
    println!("\tPreamp 12 SiPM Bias:    {:.3}[V]", preamp_bias.read_biases[11]);
    println!("\tPreamp 13 SiPM Bias:    {:.3}[V]", preamp_bias.read_biases[12]);
    println!("\tPreamp 14 SiPM Bias:    {:.3}[V]", preamp_bias.read_biases[13]);
    println!("\tPreamp 15 SiPM Bias:    {:.3}[V]", preamp_bias.read_biases[14]);
    println!("\tPreamp 16 SiPM Bias:    {:.3}[V]", preamp_bias.read_biases[15]);
}