#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate kafka;

use std::time::Duration;

use kafka::producer::{Producer, RequiredAcks};

#[get("/ping")]
fn ping() -> &'static str {
    "OK"
}

fn main() {

    let kafka_server = "rocket-kafka_kafka:9092";

    let producer = Producer::from_hosts(vec![kafka_server.to_owned()])
        .with_ack_timeout(Duration::from_secs(5))
        .with_required_acks(RequiredAcks::One)
        .create();

    rocket::ignite().mount(
        "/api/1",
        routes![ping]
    ).launch();
}
