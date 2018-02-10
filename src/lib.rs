//! TODO
extern crate crossbeam_channel;
extern crate opentracingrust;


mod tracer;
mod collectors;


pub use self::tracer::ZipkinTracer;
pub use self::collectors::kafka::KafkaCollector;
