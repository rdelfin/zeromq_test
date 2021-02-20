use futures_core::Stream;
use spin_sleep::LoopHelper;
use std::{error, fs, path::PathBuf, pin::Pin, sync::Arc, time::SystemTime};
use structopt::StructOpt;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};
use zeromq_test::data::{
    topic_publisher_server::{TopicPublisher, TopicPublisherServer},
    Image, SubscribeRequest,
};

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "image_pub")]
struct Opt {
    #[structopt(short = "p", long)]
    image_path: PathBuf,

    #[structopt(short, long)]
    extension: String,
}

#[derive(Debug)]
struct TopicPublisherService {
    images: Arc<Vec<Vec<u8>>>,
}

#[tonic::async_trait]
impl TopicPublisher for TopicPublisherService {
    type SubscribeStream =
        Pin<Box<dyn Stream<Item = Result<Image, Status>> + Send + Sync + 'static>>;

    async fn subscribe(
        &self,
        _req: Request<SubscribeRequest>,
    ) -> Result<Response<Self::SubscribeStream>, Status> {
        let (tx, rx) = mpsc::channel(4);
        let images = self.images.clone();

        tokio::spawn(async move {
            let mut loop_helper = LoopHelper::builder()
                .report_interval_s(0.5) // report every half a second
                .build_with_target_rate(25.0);
            let mut idx = 0;

            loop {
                loop_helper.loop_start();
                idx = (idx + 1) % images.len();
                let ts = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos() as u64;

                match tx.send(Ok(image_from_data(&images[idx], ts))).await {
                    Ok(_) => {}
                    Err(_) => {
                        println!("An error has occured sending.");
                        break;
                    }
                }

                if let Some(fps) = loop_helper.report_rate() {
                    println!("FPS: {:.4}", fps)
                }
                loop_helper.loop_sleep();
            }
        });

        Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let opt = Opt::from_args();
    let addr = "[::1]:50052".parse().unwrap();
    Server::builder()
        .add_service(TopicPublisherServer::new(TopicPublisherService {
            images: Arc::new(load_images(opt.image_path, &opt.extension)?),
        }))
        .serve(addr)
        .await?;

    Ok(())
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
        data: data.to_vec(),
    }
}
