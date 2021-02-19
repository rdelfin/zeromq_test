use prost::Message;
use std::{error, time::SystemTime};
use zeromq_test::data::Image;

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Collecting updates from action server...");

    let context = zmq::Context::new();
    let subscriber = context.socket(zmq::SUB).unwrap();
    assert!(subscriber.connect("ipc://camera.ipc").is_ok());

    assert!(subscriber.set_subscribe(b"").is_ok());

    loop {
        let msg = subscriber.recv_bytes(0).unwrap();
        let image = Image::decode(&msg[..]).unwrap();
        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_nanos() as u64;
        let diff = (ts - image.timestamp) as f64;
        println!(
            "Got image. Latency: {} width: {}, height: {}, channels: {}, bytes: {}",
            diff,
            image.width,
            image.height,
            image.channels,
            image.data.len()
        );
    }
}
