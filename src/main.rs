use cdr::{CdrLe, Infinite};
use serde_derive::{Deserialize, Serialize};
use zenoh::prelude::r#async::*;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
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

    // publisher
    let publisher = session.declare_publisher(key.clone()).res().await.unwrap();
    let data = DataType {
        count: 0,
        payload: vec![97, 97, 97, 97, 97, 97, 97, 97],
    };
    let encoded = cdr::serialize::<_, _, CdrLe>(&data, Infinite).unwrap();
    if let Err(e) = publisher.put(encoded).res().await {
        println!("Error writing {}: {}", publisher.key_expr().as_str(), e);
    }

    // subscriber
    let subscriber = session.declare_subscriber(key).res().await.unwrap();
    let sample = subscriber.recv_async().await;
    let sample = sample.unwrap();
    let result: Result<DataType, _> =
        cdr::deserialize_from(&*sample.payload.contiguous(), cdr::size::Infinite);
    let Ok(result) = result else {return};
    println!("{:?}", result);
}
