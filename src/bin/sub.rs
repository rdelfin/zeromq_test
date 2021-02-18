use prost::Message;
use std::env;
use zeromq_test::data::{Action, Image};

fn main() {
    println!("Collecting updates from action server...");

    let context = zmq::Context::new();
    let subscriber = context.socket(zmq::SUB).unwrap();
    assert!(subscriber.connect("ipc://action.ipc").is_ok());

    assert!(subscriber.set_subscribe(b"").is_ok());

    loop {
        let msg = subscriber.recv_bytes(0).unwrap();
        let action = Action::decode(&msg[..]).unwrap();
        println!(
            "Got action: angle: {}, speed: {}",
            action.angle, action.speed
        );
    }
}
