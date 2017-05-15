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

## Create messages for the producer

```bash
curl -X POST http://localhost:8000/api/1/message/hello-world
```
