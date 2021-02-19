use capnp::{message::ReaderOptions, serialize_packed};
use std::{
    error,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::SystemTime,
};
use zeromq_test::capnp_structs::data::image;

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Collecting updates from action server...");

    let context = zmq::Context::new();
    let subscriber = context.socket(zmq::SUB).unwrap();
    assert!(subscriber.connect("ipc://camera.ipc").is_ok());
    assert!(subscriber.set_subscribe(b"").is_ok());

    let mut latencies: Vec<f64> = vec![];

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");
    while running.load(Ordering::SeqCst) {
        let msg = match subscriber.recv_bytes(0) {
            Ok(m) => m,
            Err(_) => {
                continue;
            }
        };
        let message =
            match serialize_packed::read_message(&mut msg.as_slice(), ReaderOptions::new()) {
                Ok(i) => i,
                Err(_) => {
                    continue;
                }
            };
        let img = match message.get_root::<image::Reader>() {
            Ok(img) => img,
            Err(_) => {
                continue;
            }
        };
        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_nanos() as u64;
        let diff = (ts - img.get_timestamp()) as f64 / 1e6;
        latencies.push(diff);
        println!(
            "Got image. Latency: {:.4}ms width: {}, height: {}, channels: {}, bytes: {}",
            diff,
            img.get_width(),
            img.get_height(),
            img.get_channels(),
            img.get_data()?.len()
        );
    }

    println!(
        "Average latencies: {:.4}ms",
        latencies.iter().fold(0., |prev, l| prev + l) / (latencies.len() as f64)
    );
    Ok(())
}
