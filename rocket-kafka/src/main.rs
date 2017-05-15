#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate kafka;

use std::time::Duration;

use kafka::producer::{Producer, Record, RequiredAcks};
use kafka::error::Error as KafkaError;

fn send_message(message: &str) -> Result<(), KafkaError>
{
    let kafka_server = "rocket-kafka_kafka:9092";
    let topic = "topic";

    let mut producer = try!(Producer::from_hosts(vec![kafka_server.to_owned()])
        .with_ack_timeout(Duration::from_secs(5))
        .with_required_acks(RequiredAcks::One)
        .create());

    try!(producer.send(&Record::from_value(topic, message)));

    Ok(())
}

#[post("/message/<message>")]
fn message(message: &str) -> &str {

    if let Err(error) = send_message(message) {
        println!("{}", error);
    }

    "OK"
}

fn main() {

    rocket::ignite().mount(
        "/api/1",
        routes![message]
    ).launch();
}
