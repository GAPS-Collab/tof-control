use tof_control::helper::pb_type::PBTemp;

fn main() {
    // let temperature = PBTemp::new();
    if let Ok(temperature) = PBTemp::new() {
        println!("{:?}", temperature)
    } else {
        println!("Error")
    }
}

// fn pb_temp() -> PBTemp {
//     let temperature = PBTemp::new();
//     match temperature {
//         Ok(temperature) => {
//             temperature
//         },
//         Err(_) => {
//             PBTemp {
//                 pds_temp: f32::MAX,
//                 pas_temp: f32::MAX,
//                 nas_temp: f32::MAX,
//                 shv_temp: f32::MAX,
//             }
//         }
//     }
// }