use capnp::{
    message::{Builder, HeapAllocator},
    serialize,
};
use spin_sleep::LoopHelper;
use std::{error, fs, path::PathBuf, time::SystemTime};
use structopt::StructOpt;
use zeromq_test::capnp_structs::data::image;
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
    context.set_io_threads(4)?;
    let publisher = context.socket(zmq::PUB).unwrap();

    assert!(publisher.bind("ipc://camera.sock").is_ok());

    let mut loop_helper = LoopHelper::builder()
        .report_interval_s(0.5) // report every half a second
        .build_with_target_rate(25.0);
    let images = load_images(opt.image_path, &opt.extension)?;
    let mut idx = 0;

    loop {
        loop_helper.loop_start();

        idx = (idx + 1) % images.len();
        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_nanos() as u64;

        let img = image_from_data(&images[idx], ts);
        let mut buf: Vec<u8> = Vec::new();
        serialize::write_message(&mut buf, &img)?;
        publisher.send(&buf, 0).unwrap();

        if let Some(fps) = loop_helper.report_rate() {
            println!("FPS: {:.4}", fps)
        }
        loop_helper.loop_sleep();
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

fn image_from_data(data: &Vec<u8>, ts: u64) -> Builder<HeapAllocator> {
    let mut message = Builder::new_default();
    let mut img = message.init_root::<image::Builder>();
    img.set_timestamp(ts);
    img.set_width(2048);
    img.set_height(1280);
    img.set_channels(3);
    img.set_data(data);
    message
}
