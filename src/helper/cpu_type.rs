#[derive(Debug)]
pub struct CPUInfo {
    pub uptime: u32,
}

#[derive(Debug)]
pub struct CPUTemp {
    pub cpu0_temp: f32,
    pub cpu1_temp: f32,
}