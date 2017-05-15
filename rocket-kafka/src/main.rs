#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate kafka;

use std::time::Duration;

use kafka::producer::{Producer, Record, RequiredAcks};

#[post("/message/<message>")]
fn message(message: &str) -> &'static str {

    let kafka_server = "rocket-kafka_kafka:9092";
    let topic = "topic";

    let producer = Producer::from_hosts(vec![kafka_server.to_owned()])
        .with_ack_timeout(Duration::from_secs(5))
        .with_required_acks(RequiredAcks::One)
        .create();

    producer.send(&Record::from_value(topic, message));

    return "OK";
}

fn main() {

    rocket::ignite().mount(
        "/api/1",
        routes![message]
    ).launch();
}
