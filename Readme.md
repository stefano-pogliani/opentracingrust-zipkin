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
use opentracingrust_zipkin::KafkaCollector;


fn main() {
    let (tracer, receiver) = ZipkinTracer::new();
    GlobalTracer::init(tracer);

    let collector = KafkaCollector::new("localhost:9092");
    let reporter = ReporterThread::new(move |span| {
        collector.collect(span).unwrap();
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
#thrift -r --out /thrift/src/thrift_gen --gen rs /thrift/models/zipkinCore.thrift
```
