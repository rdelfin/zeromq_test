use prost::Message;
use rand::{
    distributions::{Distribution, Uniform},
    thread_rng,
};
use zeromq_test::data::{Action, Image};
use zmq::Context;

fn main() {
    let context = Context::new();
    let publisher = context.socket(zmq::PUB).unwrap();

    assert!(publisher.bind("tcp://*:5556").is_ok());
    assert!(publisher.bind("ipc://action.ipc").is_ok());

    let mut rng = rand::thread_rng();
    let angle_dist = Uniform::new(0.0, 2. * std::f64::consts::PI);
    let speed_dist = Uniform::new(0.0, 5. as f64);

    loop {
        let action = Action {
            angle: angle_dist.sample(&mut rng),
            speed: speed_dist.sample(&mut rng),
        };
        let mut buf: Vec<u8> = Vec::new();
        action.encode(&mut buf).unwrap();
        publisher.send(&buf, 0).unwrap();
    }
}
