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
