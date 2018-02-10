//! TODO
extern crate byteorder;
extern crate crossbeam_channel;
extern crate data_encoding;
extern crate opentracingrust;
extern crate rand;


mod tracer;
mod collectors;


pub use self::tracer::ZipkinTracer;
pub use self::collectors::kafka::KafkaCollector;
