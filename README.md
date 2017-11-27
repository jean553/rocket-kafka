# rocket-kafka

[![Build Status](https://travis-ci.org/jean553/rocket-kafka.svg?branch=master)](https://travis-ci.org/jean553/rocket-kafka)

## Installation of the containers

```bash
vagrant up
```

## Connect to the development container

```bash
vagrant ssh
```

## Compilation and execution

```bash
cargo run
```

## Create a Kafka topic

Connect to the Kafka container:

```bash
docker exec -it rocket-kafka_kafka /bin/bash
cd /opt/kafka_2.11-0.10.2.0
```

```bash
bin/kafka-topics.sh --create --zookeeper localhost:2181 --replication-factor 1 --partitions 1 --topic topic
```

## Create messages for the producer

```bash
curl -X POST http://localhost:8000/api/1/message/hello-world
```

## Read messages from the producer

Into the Kafka container.

```bash
bin/kafka-console-consumer.sh --bootstrap-server localhost:9092 --topic topic --from-beginning
```
