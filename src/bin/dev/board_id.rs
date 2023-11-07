use gethostname::gethostname;

use tof_control::constant::*;
use tof_control::memory::*;

fn main() {
    let hostname = gethostname()
        .into_string()
        .expect("cannot convert hostname");
    let board_id: u32 = hostname.replace("tof-rb", "").parse().unwrap();
    write_control_reg(BOARD_ID, board_id).expect("cannot write BOARD_ID");

    let board_id_reg = read_control_reg(BOARD_ID).expect("cannot read BOARD_ID");
    println!("{}", board_id_reg);
}
