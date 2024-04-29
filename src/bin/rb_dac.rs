use tof_control::rb_control::rb_dac;

fn main() {
    match rb_dac::set_dac_500() {
        Ok(_) => {
            println!("The DRS4 range is set to -500mV to 500mV.")
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}