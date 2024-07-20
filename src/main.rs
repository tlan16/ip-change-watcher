use std::time::Duration;

use rdkafka::ClientConfig;
use rdkafka::message::{Header, OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::get_rdkafka_version;

async fn produce(brokers: &str, topic_name: &str) {
  let producer: &FutureProducer = &ClientConfig::new()
    .set("bootstrap.servers", brokers)
    .set("message.timeout.ms", "5000")
    .create()
    .expect("Producer creation error");

  let futures = (0..5)
    .map(|i| async move {
      // The send operation on the topic returns a future, which will be
      // completed once the result or failure from Kafka is received.
      let delivery_status = producer
        .send(
          FutureRecord::to(topic_name)
            .payload(&format!("Message {}", i))
            .key(&format!("Key {}", i))
            .headers(OwnedHeaders::new().insert(Header {
              key: "header_key",
              value: Some("header_value"),
            })),
          Duration::from_secs(0),
        )
        .await;

      // This will be executed when the result is received.
      println!("Delivery status for message {} received", i);
      delivery_status
    })
    .collect::<Vec<_>>();

  // This loop will wait until all delivery statuses have been received.
  for future in futures {
    println!("Future completed. Result: {:?}", future.await);
  }
}


#[tokio::main]
async fn main() {
  if let Some(ip) = public_ip::addr().await {
    println!("public ip address: {:?}", ip);
  } else {
    println!("couldn't get an IP address");
  }

  let (version_n, version_s) = get_rdkafka_version();
  println!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

  let topic = "my-topic";
  let brokers = "localhost:9092";

  produce(brokers, topic).await;
}
