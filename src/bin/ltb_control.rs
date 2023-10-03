use tof_control::helper::ltb_type::{LTBMoniData, LTBTemp, LTBThreshold};
use tof_control::ltb_control::ltb_threshold;

fn main() {
    let temperature = ltb_temp();
    let threshold = ltb_threshold();
    println!("{:?}", temperature);
    println!("{:?}", threshold);

    // match ltb_threshold::set_default_threshold() {
    //     Ok(_) => {
    //         println!("LTB Threshold Set!");
    //     },
    //     Err(e) => {
    //         eprintln!("Error: {:?}", e);
    //     }
    // }

}

fn ltb_temp() -> LTBTemp {
    let temperature = LTBTemp::new();
    match temperature {
        Ok(temperature) => {
            temperature
        },
        Err(_) => {
            LTBTemp {
                trenz_temp: f32::MAX,
                ltb_temp: f32::MAX,
            }
        }
    }
}

fn ltb_threshold() -> LTBThreshold {
    let threshold = LTBThreshold::new();
    match threshold {
        Ok(threshold) => {
            threshold
        },
        Err(_) => {
            LTBThreshold {
                threshold_0: f32::MAX,
                threshold_1: f32::MAX,
                threshold_2: f32::MAX,
            }
        }
    }
}