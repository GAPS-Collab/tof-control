use sysinfo::{System, SystemExt, DiskExt, CpuExt, RefreshKind, CpuRefreshKind};
// use std::{thread, time::Duration};

use crate::helper::cpu_type::CPUInfo;

impl CPUInfo {
    pub fn new() -> Self {
        let cpu_info = Self::read_info();

        cpu_info
    }
    pub fn read_info() -> CPUInfo {
        let mut sys = System::new_all();

        let uptime = Self::read_uptime(&mut sys);
        let disk_usage = Self::read_disk_usage(&mut sys);
        // Self::read_cpu_load(&mut sys);
        // Self::read_cpu_load();
        // sys.refresh_cpu();
        let cpu_freq = Self::read_cpu_freq(&mut sys);

        CPUInfo {
            uptime,
            disk_usage,
            cpu_freq,
        }
    }
    pub fn read_uptime(sys: &System) -> u32 {
        let uptime = sys.uptime() as u32;

        uptime
    }
    pub fn read_disk_usage(sys: &System) -> u8 {
        let mut available_space = Default::default();
        let mut total_space = Default::default();
        for disk in sys.disks() {
            if disk.mount_point().as_os_str() == "/" {
                available_space = disk.available_space();
                total_space = disk.total_space();
            }
        }

        let disk_usage: f32 = (1.0 - (available_space as f32 / total_space as f32)) * 100.0;

        disk_usage as u8
    }
    // pub fn read_cpu_load(sys: &mut System) {
    pub fn read_cpu_load() {
        // println!("{:?}", sys.cpus());
        // println!("Load Average: {:?}", sys.load_average());
        // let cpu0_load = sys.load_average();
        let mut sys = System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));
        // sys.refresh_cpu_specifics(CpuRefreshKind::everything());
        // for cpu in sys.cpus() {
        //     cpu.cpu_usage();
        // }
        let mut l = 0;
        while l < 4 {
            sys.refresh_cpu_specifics(CpuRefreshKind::everything());
            for cpu in sys.cpus() {
                println!("Usage: {:?}", cpu.cpu_usage());
                println!("Frequency: {:?}", cpu.frequency());
            }
            l += 1;
            // thread::sleep(Duration::from_millis(1000));
        }
        // sys.refresh_cpu_specifics(CpuRefreshKind::everything());
        // for cpu in sys.cpus() {
        //     println!("Usage: {:?}", cpu.cpu_usage());
        //     println!("Frequency: {:?}", cpu.frequency());
        // }
    }
    pub fn read_cpu_freq(sys: &System) -> [u32; 4] {
        let mut cpu0_freq: u32 = Default::default();
        let mut cpu1_freq: u32 = Default::default();
        let mut cpu2_freq: u32 = Default::default();
        let mut cpu3_freq: u32 = Default::default();
        for cpu in sys.cpus() {
            let cpu_name = cpu.name();
            match cpu_name {
                "cpu0" => {
                    cpu0_freq = cpu.frequency() as u32;
                }
                "cpu1" => {
                    cpu1_freq = cpu.frequency() as u32;
                }
                "cpu2" => {
                    cpu2_freq = cpu.frequency() as u32;
                }
                "cpu3" => {
                    cpu3_freq = cpu.frequency() as u32;
                }
                _ => {}
            }
        }

        let cpu_freq = [cpu0_freq, cpu1_freq, cpu2_freq, cpu3_freq];

        cpu_freq
    }
}