use opentracingrust::SpanReference;
use opentracingrust::SpanReferenceAware;

use super::trace_id::TraceID;


/// Zipkin specific `SpanContext`.
///
/// # References
///
///   * https://zipkin.io/pages/instrumenting.html#communicating-trace-information
///   * https://github.com/openzipkin/zipkin-api/blob/master/thrift/zipkinCore.thrift
#[derive(Clone)]
pub struct ZipkinContext {
    debug: bool,
    sampled: bool,
    span_id: u64,
    trace_id: TraceID,
}

impl ZipkinContext {
    /// TODO
    pub fn new() -> ZipkinContext {
        let span_id = 0;
        let trace_id = TraceID::new();
        ZipkinContext {
            debug: false,
            sampled: true,
            span_id,
            trace_id,
        }
    }
}

impl SpanReferenceAware for ZipkinContext {
    fn reference_span(&mut self, reference: &SpanReference) {
        // TODO
    }
}


#[cfg(test)]
mod tests {
    // TODO
}
