use rand::Rng;

fn main() {
    let context = zmq::Context::new();
    let publisher = context.socket(zmq::PUB).unwrap();

    assert!(publisher.bind("tcp://*:5556").is_ok());
    assert!(publisher.bind("ipc://weather.ipc").is_ok());

    let mut rng = rand::thread_rng();

    loop {
        let zipcode = rng.gen_range(0..100_000);
        let temperature = rng.gen_range(-80..135);
        let relhumidity = rng.gen_range(10..60);

        let update = format!("{:05} {} {}", zipcode, temperature, relhumidity);
        publisher.send(&update, 0).unwrap();
    }
}
