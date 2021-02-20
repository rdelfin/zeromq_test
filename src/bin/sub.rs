use std::{
    error,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::SystemTime,
};
use tonic::{transport::Channel, Request};
use zeromq_test::data::{topic_publisher_client::TopicPublisherClient, Image, SubscribeRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Collecting updates from image server...");

    let mut latencies: Vec<f64> = vec![];

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let mut client = TopicPublisherClient::connect("http://[::1]:50052").await?;

    let mut stream = client
        .subscribe(Request::new(SubscribeRequest {}))
        .await?
        .into_inner();

    while running.load(Ordering::SeqCst) {
        let img = match stream.message().await? {
            Some(i) => i,
            None => {
                break;
            }
        };
        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_nanos() as u64;
        let diff = (ts - img.timestamp) as f64 / 1e6;
        latencies.push(diff);
        println!(
            "Got image. Latency: {:.4}ms width: {}, height: {}, channels: {}, bytes: {}",
            diff,
            img.width,
            img.height,
            img.channels,
            img.data.len()
        );
    }

    println!(
        "Average latencies: {:.4}ms",
        latencies.iter().fold(0., |prev, l| prev + l) / (latencies.len() as f64)
    );
    Ok(())
}
