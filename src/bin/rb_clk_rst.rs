use tof_control::rb_control::rb_clk;

fn main() {
    
    match rb_clk::reset_clk_synth(1) {
        Ok(_) => {
            println!("Clock Synthesizer was successfully reset (hard reset).")
        }
        Err(e) => {
            eprintln!("Error on resetting clock synthesizer: {:?}", e)
        }
    }
}