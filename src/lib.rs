//! Zipkin tracer implementation for [OpenTracingRust](https://crates.io/crates/opentracingrust/all)
//!
//! See the examples for usage.
extern crate byteorder;
extern crate crossbeam_channel;
extern crate data_encoding;
extern crate opentracingrust;
extern crate rand;

// Needed by thrift
extern crate ordered_float;
extern crate thrift;
extern crate try_from;


mod collectors;
mod thrift_gen;
mod tracer;


pub use self::tracer::ZipkinTracer;
pub use self::tracer::ZipkinContextOptions;

pub use self::collectors::kafka::KafkaCollector;
