use clap::{Parser, Subcommand, ValueEnum};
use chrono::prelude::*;
use chrono_tz::America::Los_Angeles;

use tof_control::helper::cpu_type::{CPUInfo, CPUTemp, CPUTempDebug};

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
    #[clap(short_flag = 'r')]
    Read {
        #[arg(ignore_case = true, value_enum)]
        sensor: Option<Sensor>,
    },
    #[clap(short_flag = 'd')]
    Debug {}
}

#[derive(ValueEnum, Clone, Debug)]
enum Sensor {
    Info,
    Temp,
}

fn main() {
    
    let cli = Cli::parse();

    match cli.command {
        Commands::Read { sensor } => {
            match sensor {
                Some(s) => {
                    match s {
                        Sensor::Info => {
                            print_info();
                        }
                        Sensor::Temp => {
                            // print_temp();
                            print_temp_debug();
                        }
                    }
                }
                None => {
                    print_info();
                    // print_temp();
                    print_temp_debug();
                }
            }
        }
        Commands::Debug {} => {
            if cli.verbose {
                println!("%Y %m %d %H %M %S CPU_TEMP CPU0_TEMP CPU1_TEMP MB_TEMP CPU0_FREQ CPU1_FREQ CPU2_FREQ CPU3_FREQ");
                print_debug_mode();
            } else {
                print_debug_mode();
            }
        }
    }
}

fn print_info() {
    let cpu_info = CPUInfo::new();

    println!("CPU Information");
    println!("\tUptime              : {}[s]", cpu_info.uptime);
    println!("\tDisk Usage          : {}[%]", cpu_info.disk_usage);
    println!("\tCPU0 Frequency      : {}[Hz]", cpu_info.cpu_freq[0]);
    println!("\tCPU1 Frequency      : {}[Hz]", cpu_info.cpu_freq[1]);
    println!("\tCPU2 Frequency      : {}[Hz]", cpu_info.cpu_freq[2]);
    println!("\tCPU3 Frequency      : {}[Hz]", cpu_info.cpu_freq[3]);
}

fn print_temp() {
    let cpu_temp = CPUTemp::new();

    println!("CPU Temperature");
    println!("\tCPU0 Temp           : {:.3}[°C]", cpu_temp.cpu0_temp);
    println!("\tCPU1 Temp           : {:.3}[°C]", cpu_temp.cpu1_temp);
}

fn print_temp_debug() {
    let cpu_temp = CPUTempDebug::new();

    println!("CPU Temperature (Debug Mode)");
    println!("\tCPU Temp            : {:.3}[°C]", cpu_temp.cpu_temp);
    println!("\tCPU0 Temp           : {:.3}[°C]", cpu_temp.cpu0_temp);
    println!("\tCPU1 Temp           : {:.3}[°C]", cpu_temp.cpu1_temp);
    println!("\tMother Board Temp   : {:.3}[°C]", cpu_temp.mb_temp);
}

fn print_debug_mode() {
    let datetime = Utc::now().naive_utc();
    let datetime_local = Los_Angeles.from_utc_datetime(&datetime);
    let dt_format = datetime_local.format("%Y %m %d %H %M %S");

    let cpu_temperature = CPUTempDebug::new();
    let cpu_temp = cpu_temperature.cpu_temp;
    let cpu0_temp = cpu_temperature.cpu0_temp;
    let cpu1_temp = cpu_temperature.cpu1_temp;
    let mb_temp = cpu_temperature.mb_temp;

    let cpu_information = CPUInfo::new();
    let cpu_freq = cpu_information.cpu_freq;


    println!("{} {} {} {} {} {} {} {} {}",
        dt_format,
        cpu_temp, cpu0_temp, cpu1_temp, mb_temp,
        cpu_freq[0], cpu_freq[1], cpu_freq[2], cpu_freq[3],
    );
}