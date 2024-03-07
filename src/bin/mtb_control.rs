use std::net::{UdpSocket, SocketAddr};
use std::time::Duration;
use hex::FromHex;

// fn main() {
//     let local_addrs = [
//         SocketAddr::from(([0, 0, 0, 0], 50100)),
//         SocketAddr::from(([0, 0, 0, 0], 50101)),
//         SocketAddr::from(([0, 0, 0, 0], 50102)),
//         SocketAddr::from(([0, 0, 0, 0], 50103)),
//         SocketAddr::from(([0, 0, 0, 0], 50104)),
//       ];
//       let local_socket = UdpSocket::bind(&local_addrs[..]);
//       let socket : UdpSocket;
//       match local_socket {
//         Err(err)   => {
//           println!("Can not create local UDP socket for master trigger connection!, err {}", err);
//         //   return Err(err);
//         }
//         Ok(value)  => {
//             println!("Successfully bound UDP socket for master trigger communcations to {:?}", value);
//             socket = value;
//           // this is not strrictly necessary, but 
//           // it is nice to limit communications
//           match socket.set_read_timeout(Some(Duration::from_millis(1))) {
//             Err(err) => println!("Can not set read timeout for Udp socket! Error {err}"),
//             Ok(_)    => ()
//           }
//           match socket.connect("10.0.1.10:50001") {
//             Err(err) => {
//                 println!("Can not connect to master trigger at {}, err {}", "10.0.1.10:50001", err);
//             //   return Err(err);
//             }
//             Ok(_) => {
//                 println!("Successfully connected to the master trigger at {}", "10.0.1.10:50001");
//             }
//           }
//         }
//       }
// }

fn main() {
    let local_addr = "0.0.0.0:51001";
    let mtb_addr = "10.0.1.10:50001";

    let socket = UdpSocket::bind(&local_addr).expect("1");
    socket.set_read_timeout(Some(Duration::from_millis(1))).expect("2");
    socket.connect(&mtb_addr).expect("3");
    // let mut msg: [u8; 4] = Default::default();
    // let msg = hex::decode( "0x488").expect("4");
    let msg = <[u8; 2]>::from_hex("0488").expect("4");
    println!("{:?}", msg);
    socket.send(&msg).expect("5");
    let mut buf = [0; 4];
    let received = socket.recv(&mut buf).expect("6");
    println!("{:?}", received);
}