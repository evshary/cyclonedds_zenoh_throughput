use cdr::{CdrLe, Infinite};
use serde_derive::{Deserialize, Serialize};
use zenoh::prelude::r#async::*;

#[derive(Deserialize, PartialEq, Debug)]
struct DataType {
    count: u64,
    payload: Vec<u8>,
}

#[async_std::main]
async fn main() {
    env_logger::init();
    println!("Opening session...");
    let session = zenoh::open(Config::default()).res().await.unwrap();
    let key = String::from("Throughput");

    // subscriber
    let subscriber = session.declare_subscriber(key).res().await.unwrap();
    let sample = subscriber.recv_async().await;
    let sample = sample.unwrap();
    let result: Result<DataType, _> =
        cdr::deserialize_from(&*sample.payload.contiguous(), cdr::size::Infinite);
    let Ok(result) = result else {return};
    println!("{:?}", result);
}
