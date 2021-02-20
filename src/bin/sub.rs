use prost::Message;
use std::{
    error,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::SystemTime,
};
use zeromq_test::data::Image;

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Collecting updates from action server...");

    let context = zmq::Context::new();
    let img_subscriber = context.socket(zmq::SUB).unwrap();
    let data_subscriber = context.socket(zmq::SUB).unwrap();
    assert!(img_subscriber.connect("ipc://camera.ipc").is_ok());
    assert!(img_subscriber.set_subscribe(b"").is_ok());
    assert!(data_subscriber.connect("ipc://camera_data.ipc").is_ok());
    assert!(data_subscriber.set_subscribe(b"").is_ok());

    let mut latencies: Vec<f64> = vec![];

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");
    while running.load(Ordering::SeqCst) {
        let msg = match img_subscriber.recv_bytes(0) {
            Ok(m) => m,
            Err(_) => {
                continue;
            }
        };
        let image = match Image::decode(&msg[..]) {
            Ok(i) => i,
            Err(_) => {
                continue;
            }
        };
        let data = match data_subscriber.recv_bytes(0) {
            Ok(m) => m,
            Err(_) => {
                continue;
            }
        };
        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_nanos() as u64;
        let diff = (ts - image.timestamp) as f64 / 1e6;
        latencies.push(diff);
        println!(
            "Got image. Latency: {:.4}ms width: {}, height: {}, channels: {}, bytes: {}",
            diff,
            image.width,
            image.height,
            image.channels,
            data.len(),
        );
    }

    println!(
        "Average latencies: {:.4}ms",
        latencies.iter().fold(0., |prev, l| prev + l) / (latencies.len() as f64)
    );
    Ok(())
}
