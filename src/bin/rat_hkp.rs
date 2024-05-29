#![allow(unused)]
use std::str;

fn main() -> std::io::Result<()> {
    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::STREAM)?;
    socket.bind("tcp://10.0.1.137:25200")?;
    loop {
        let data = socket.recv_multipart(0)?;
        println!("Identity: {:?} Message: {}", data[0], str::from_utf8(&data[1]).unwrap());
    }

    Ok(())
}