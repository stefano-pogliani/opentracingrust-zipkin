OpenTracingRust Zipkin
======================
[Zipkin](https://zipkin.io/) tracer for [OpenTracingRust](https://docs.rs/opentracingrust/0.3.0/opentracingrust/).


Usage
-----
Once your `Cargo.toml` includes a dependency on `opentracingrust_zipkin`
create a tracer and a collector and initialise the system:

```rust
extern crate opentracingrust;
extern crate opentracingrust_zipkin;

use opentracingrust::utils::GlobalTracer;
use opentracingrust::utils::ReporterThread;

use opentracingrust_zipkin::ZipkinTracer;
use opentracingrust_zipkin::ZipkinEndpoint;
use opentracingrust_zipkin::KafkaCollector;


fn main() {
    let (tracer, receiver) = ZipkinTracer::new();
    GlobalTracer::init(tracer);

    let mut collector = KafkaCollector::new(
        ZipkinEndpoint::new(None, None, Some(String::from("zipkin-example")), None),
        String::from("zipkin"),  // Kafka topic
        vec![String::from("127.0.0.1:9092")]  // Kafka seed
    );
    let reporter = ReporterThread::new(receiver, move |span| {
        match collector.collect(span) {
            Err(err) => println!("Failed to report span: {:?}", err),
            _ => (),
        }
    });
}
```


Thrift
------
This crate uses thrift to send messages to Zipkin and for the bynary encoding.

The thift-generated rust modules are checked into source control so that the
crate can be compiled without having to install a rust-enabled thrift compiler.

This crate comes with a Dockerfile that installs and configures thrift with rust support.
The commands below can be used to re-generate the rust code from thrift definitions:

```bash
cd thrift/
docker build --rm --tag thrift-compiler .
docker run --rm -it -v $PWD:/thrift/models -v $PWD/../src:/thrift/src thrift-compiler bash
thrift -r --out /thrift/src/thrift_gen --gen rs /thrift/models/binary_format.thrift
thrift -r --out /thrift/src/thrift_gen --gen rs /thrift/models/zipkinCore.thrift
```


Docker Zipkin Playground
------------------------
To play around with the tracer and aid development you can run a
zipkin-in-a-box with docker and docker-compose.

The following steps are for use with the provided docker scripts:

  1. `git clone https://github.com/openzipkin/docker-zipkin.git`
  2. Update the `KAFKA_ADVERTISED_HOST_NAME`:
    1. Set it to `kafka-zookeeper` to make all containers properly work with each other.
    2. Edit `docker-zipkin/docker-compose-kafka10.yml` at line 15.
    3. New line should be `KAFKA_ADVERTISED_HOST_NAME=kafka-zookeeper`.
  3. If you want to access kafka from outside the docker-compose hosts (likely so):
    1. Find the IP of the `kafka-zookeeper` container.
    2. Edit `/etc/hosts`.
    3. Add `<IP> kafka-zookeeper`.
  4. Start all the zipkin processes: `docker-compose -f docker-compose.yml -f docker-compose-kafka10.yml -f docker-compose-ui.yml up`
