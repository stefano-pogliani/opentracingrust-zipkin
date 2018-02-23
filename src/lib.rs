//! Zipkin tracer implementation for [OpenTracingRust](https://crates.io/crates/opentracingrust/all)
//!
//! See the examples for usage.
#![doc(html_root_url = "https://docs.rs/opentracingrust_zipkin/0.1.0")]
extern crate byteorder;
extern crate crossbeam_channel;
extern crate data_encoding;
extern crate kafka;
extern crate opentracingrust;
extern crate rand;
extern crate serde;
extern crate serde_json;

// Needed by thrift
extern crate ordered_float;
extern crate thrift;
extern crate try_from;


mod collectors;
mod thrift_gen;
mod tracer;


pub use self::thrift_gen::zipkin_core::Endpoint as ZipkinEndpoint;
pub use self::tracer::ZipkinTracer;
pub use self::collectors::kafka::KafkaCollector;
