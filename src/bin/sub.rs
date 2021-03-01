use capnp::{message::ReaderOptions, serialize};
use lcm::Lcm;
use std::{
    cell::RefCell,
    error,
    rc::Rc,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::{Duration, SystemTime},
};
use zeromq_test::lcm_structs::Image;

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Collecting updates from action server...");

    let latencies: Rc<RefCell<Vec<f64>>> = Rc::new(RefCell::new(vec![]));

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    fetch_images(latencies.clone(), running);

    println!(
        "Average latencies: {:.4}ms",
        latencies.borrow().iter().fold(0., |prev, l| prev + l) / (latencies.borrow().len() as f64)
    );
    Ok(())
}

fn fetch_images(
    latencies: Rc<RefCell<Vec<f64>>>,
    running: Arc<AtomicBool>,
) -> Result<(), Box<dyn error::Error>> {
    let mut lcm = Lcm::new()?;
    lcm.subscribe("cameras", move |img: Image| {
        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        let diff = (ts - (img.timestamp as u64)) as f64 / 1e6;
        latencies.borrow_mut().push(diff);
        println!(
            "Got image. Latency: {:.4}ms width: {}, height: {}, channels: {}, bytes: {}",
            diff,
            img.width,
            img.height,
            img.channels,
            img.data.len()
        );
    });

    while running.load(Ordering::SeqCst) {
        if let Err(_) = lcm.handle_timeout(Duration::from_millis(200)) {
            println!("Error handling message");
            continue;
        }
    }

    Ok(())
}
