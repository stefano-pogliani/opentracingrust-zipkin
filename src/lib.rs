//! Zipkin tracer implementation for [OpenTracingRust](https://crates.io/crates/opentracingrust/all)
//!
//! See the examples for usage.
//!
//! # Official API heads up
//!
//! An Opentracing official API is in the works: https://github.com/opentracing/opentracing-rust
//!
//! Once the official API crate is finished I hope to re-implement this crate to be based off of
//! that API rather then my own implementation of a base crate.
//!
//! For that reason, development of this crate will be limited while I dedicate my
//! time to the development of the offical opentracing-api crate.
//!
//! Lessons learend while developing this crate will be valuable knowledge for opentracing-api.
//! If you find any issue and or usability limit, please let me know!
//!
//! # Example
//!
//! ```ignore
//! extern crate opentracingrust;
//! extern crate opentracingrust_zipkin;
//!
//! use opentracingrust::utils::ReporterThread;
//!
//! use opentracingrust_zipkin::KafkaCollector;
//! use opentracingrust_zipkin::ZipkinEndpoint;
//! use opentracingrust_zipkin::ZipkinTracer;
//!
//! fn main() {
//!     // Create the tracer.
//!     let (tracer, receiver) = ZipkinTracer::new();
//!
//!     // Create a zipkin collector (to send finished spans to zipkin).
//!     let mut collector = KafkaCollector::new(
//!         ZipkinEndpoint::new(None, None, Some(String::from("some-service")), None),
//!         String::from("zipkin"), vec![String::from("127.0.0.1:9092")]
//!     );
//!
//!     // Create a reporter thread to process finished spans.
//!     let mut reporter = ReporterThread::new(receiver, move |span| {
//!         match collector.collect(span) {
//!             Err(err) => println!("Failed to report span: {:?}", err),
//!             _ => (),
//!         }
//!     });
//!
//!     // Your program goes here
//!     // ... snip ...
//! }
//! ```
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
