use chrono::{Utc, DateTime};

use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};

use tof_control::helper::{
    rb_type::{RBInfo, RBInfoDebug, RBTempDebug, RBVcp, RBPh, RBMag},
    ltb_type::{LTBTemp, LTBThreshold},
    pb_type::{PBTemp, PBVcp},
    preamp_type::{PreampTemp, PreampSetBias, PreampReadBias},
};
use tof_control::ltb_control::{ltb_threshold, ltb_init};
use tof_control::preamp_control::preamp_init;

#[derive(Parser, Debug)]
#[command(author = "Takeru Hayashi", version, about, long_about = None)]
struct Cli {
    #[arg(short = 'b', long = "board", help = "Board to operate (rb, pb, ltb, or preamp)")]
    board: Option<Board>,
    #[clap(short, long, help = "Show status of board")]
    init: bool,
    #[clap(short, long, help = "Set Value")]
    set: Option<f32>,
    #[clap(short, long, help = "Get Value")]
    get: bool,
    #[clap(short, long, help = "Get JSON")]
    json: bool,
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

    // let board = &cli.board;

    let sub_board = RBInfo::new().sub_board;

    if cli.json {

        if sub_board == 1 {
            let rat_json_rb1 = rat_json_rb1();
            println!("{}", rat_json_rb1);
        } else if sub_board == 2 {
            let rat_json_rb2 = rat_json_rb2();
            println!("{}", rat_json_rb2);
        }
        
        std::process::exit(0);
    }

    if let Some(board) = cli.board {
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

#[derive(Debug, Serialize, Deserialize)]
struct RBJson {
    rb_info: RBInfoDebug,
    rb_temp: RBTempDebug,
    rb_vcp: RBVcp,
    rb_ph: RBPh,
    rb_mag: RBMag,
}

fn rb_json() -> RBJson {
    let rb_info = RBInfoDebug::new();
    let rb_temp = RBTempDebug::new();
    let rb_vcp = RBVcp::new();
    let rb_ph = RBPh::new();
    let rb_mag = RBMag::new();

    let rb_json = RBJson {
        rb_info,
        rb_temp,
        rb_vcp,
        rb_ph,
        rb_mag,
    };
    // let rb_json_str = serde_json::to_string_pretty(&rb_json).unwrap();

    // rb_json_str
    rb_json
}

#[derive(Debug, Serialize, Deserialize)]
struct LTBJson {
    ltb_temp: LTBTemp,
    ltb_threshold: LTBThreshold,
}

fn ltb_json() -> LTBJson {
    let ltb_temp = LTBTemp::new();
    let ltb_threshold = LTBThreshold::new();

    let ltb_json = LTBJson {
        ltb_temp,
        ltb_threshold,
    };
    // let ltb_json_str = serde_json::to_string_pretty(&ltb_json).unwrap();

    // ltb_json_str
    ltb_json
}

#[derive(Debug, Serialize, Deserialize)]
struct PBJson {
    pb_temp: PBTemp,
    pb_vcp: PBVcp,
}

fn pb_json() -> PBJson {
    let pb_temp = PBTemp::new();
    let pb_vcp = PBVcp::new();

    let pb_json = PBJson {
        pb_temp,
        pb_vcp,
    };
    // let pb_json_str = serde_json::to_string_pretty(&pb_json).unwrap();

    // pb_json_str
    pb_json
}

#[derive(Debug, Serialize, Deserialize)]
struct PreampJson {
    preamp_temp: PreampTemp,
    preamp_bias: PreampReadBias,
}

fn preamp_json() -> PreampJson {
    let preamp_temp = PreampTemp::new();
    let preamp_bias = PreampReadBias::new();

    let preamp_json = PreampJson {
        preamp_temp,
        preamp_bias,
    };
    // let preamp_json_str = serde_json::to_string_pretty(&preamp_json).unwrap();

    // preamp_json_str
    preamp_json
}

#[derive(Debug, Serialize, Deserialize)]
struct RATJsonRB1 {
    rat: u8,
    dt: String,
    rb: RBJson,
    ltb: LTBJson,
}
fn rat_json_rb1() -> String {
    let dt = format!("{:?}",Utc::now());
    let rb = rb_json();
    let ltb = ltb_json();

    let rat = rb.rb_info.rat_num;

    let rat_json = RATJsonRB1 {
        rat,
        dt,
        rb,
        ltb,
    };

    let rat_json_str = serde_json::to_string_pretty(&rat_json).unwrap();
    
    rat_json_str
}

#[derive(Debug, Serialize, Deserialize)]
struct RATJsonRB2 {
    rat: u8,
    dt: String,
    rb: RBJson,
    pb: PBJson,
    preamp: PreampJson,
}
fn rat_json_rb2() -> String {
    let dt = format!("{:?}",Utc::now());
    let rb = rb_json();
    let pb = pb_json();
    let preamp = preamp_json();

    let rat = rb.rb_info.rat_num;

    let rat_json = RATJsonRB2 {
        rat,
        dt,
        rb,
        pb,
        preamp,
    };

    let rat_json_str = serde_json::to_string_pretty(&rat_json).unwrap();
    
    rat_json_str
}