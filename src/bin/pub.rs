use prost::Message;
use std::{error, fs, path::PathBuf, time::SystemTime};
use structopt::StructOpt;
use zeromq_test::data::{Action, Image};
use zmq::Context;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "image_pub")]
struct Opt {
    #[structopt(short = "p", long)]
    image_path: PathBuf,

    #[structopt(short, long)]
    extension: String,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let opt = Opt::from_args();
    let context = Context::new();
    let publisher = context.socket(zmq::PUB).unwrap();

    assert!(publisher.bind("ipc://camera.ipc").is_ok());

    let images = load_images(opt.image_path, &opt.extension)?;
    let mut idx = 0;

    loop {
        idx = (idx + 1) % images.len();
        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_nanos() as u64;

        let image = image_from_data(&images[idx], ts);
        let mut buf: Vec<u8> = Vec::new();
        image.encode(&mut buf).unwrap();
        publisher.send(&buf, 0).unwrap();
    }
}

fn load_images(dir: PathBuf, ext: &str) -> Result<Vec<Vec<u8>>, Box<dyn error::Error>> {
    Ok(fs::read_dir(dir.as_path())?
        .filter(|e| match e {
            Ok(entry) => {
                let is_file = {
                    match entry.file_type() {
                        Ok(ftype) => ftype.is_file(),
                        Err(_) => false,
                    }
                };
                let correct_ext = { entry.path().extension().unwrap() == ext };
                is_file && correct_ext
            }
            Err(_) => false,
        })
        .map(|e| fs::read(e.unwrap().path().as_path()))
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .map(|data| data[32..].into())
        .collect())
}

fn image_from_data(data: &Vec<u8>, ts: u64) -> Image {
    Image {
        timestamp: ts,
        width: 2048,
        height: 1280,
        channels: 3,
        data: data.clone(),
    }
}
