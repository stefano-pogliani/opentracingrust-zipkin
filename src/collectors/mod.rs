use opentracingrust::FinishedSpan;
use thrift;

use super::thrift_gen::zipkin_core;

pub mod kafka;


/// Encodes a finished span into a thrift message for Zipkin.
pub fn thrift_encode(span: FinishedSpan) -> thrift::Result<zipkin_core::Span> {
    panic!("TODO")
}


#[cfg(test)]
mod tests {
    // TODO
}
