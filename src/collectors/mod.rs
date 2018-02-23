use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use opentracingrust::FinishedSpan;

use super::thrift_gen::zipkin_core;
use super::tracer::ZipkinContext;

pub mod kafka;


const MICROSECOND: i64 = 1000000;


/// Computes the difference (in micro-seconds) between to system times.
fn compute_duration(start: SystemTime, end: SystemTime) -> i64 {
    let delta: i64 = match end.duration_since(start) {
        Ok(n) => n.as_secs() as i64,
        _ => -(start.duration_since(end).unwrap().as_secs() as i64)
    };
    delta * MICROSECOND
}


/// Encodes a finished span into a thrift message for Zipkin.
pub fn thrift_encode(span: &FinishedSpan) -> zipkin_core::Span {
    // Extract span details.
    let context = span.context().impl_context::<ZipkinContext>().expect(
        "Invalid SpanContext, was it created by ZipkinTracer?"
    );
    let (high, low) = context.trace_id().split();
    let timestamp = compute_duration(UNIX_EPOCH, span.start_time().clone());
    let duration = compute_duration(span.start_time().clone(), span.finish_time().clone());
    let duration = match duration {
        0 => 1,
        d => d,
    };

    // Convert tags into binary annotations.
    for (tag, value) in span.tags().iter() {
        // TODO: if the tag is a known zipkin tag convert appropriately.
        // TODO: convert into a binary annotation.
    }

    //// Convert logs into annotations.
    //for log in span.logs() {
    //    // TODO: convert into an annotation.
    //}

    // Create a thrift span.
    zipkin_core::Span::new(
        Some(low as i64),  // trace_id
        Some(span.name().clone()),  // name
        Some(context.span_id() as i64),  // id
        context.parent_span_id().map(|id| id as i64),  // parent_id
        None,  // annotations
        None,  // binary_annotations
        Some(context.debug()),  // debug
        Some(timestamp),  // timestamp
        Some(duration),  // duration
        Some(high as i64)  // trace_id_high
    )
}


#[cfg(test)]
mod tests {
    use std::time::UNIX_EPOCH;
    use opentracingrust::FinishedSpan;

    use super::super::ZipkinTracer;
    use super::super::tracer::ZipkinContext;
    use super::super::thrift_gen::zipkin_core;

    use super::MICROSECOND;
    use super::thrift_encode;


    fn context(span: &FinishedSpan) -> ZipkinContext {
        let context = span.context();
        context.impl_context::<ZipkinContext>().unwrap().clone()
    }

    fn mock_span() -> FinishedSpan {
        let (tracer, receiver) = ZipkinTracer::new();
        let span = tracer.span("test");
        span.finish().unwrap();
        receiver.recv().unwrap()
    }

    fn mocks() -> (FinishedSpan, ZipkinContext, zipkin_core::Span) {
        let span = mock_span();
        let context = context(&span);
        let encoded = thrift_encode(&span);
        (span, context, encoded)
    }

    #[test]
    fn serialise_ids() {
        let (_, context, encoded) = mocks();
        let (high, low) = context.trace_id().split();
        assert_eq!(encoded.trace_id.unwrap(), low as i64);
        assert_eq!(encoded.trace_id_high.unwrap(), high as i64);
        assert_eq!(encoded.id.unwrap(), context.span_id() as i64);
        assert_eq!(encoded.parent_id, None);
    }

    #[test]
    fn serialise_meta() {
        let (_, _, encoded) = mocks();
        assert_eq!(encoded.debug.unwrap(), false);
    }

    #[test]
    fn serialise_times() {
        let (span, _, encoded) = mocks();
        let timestamp = span.start_time().duration_since(UNIX_EPOCH).unwrap();
        let timestamp = timestamp.as_secs() * 1000000;
        assert!(encoded.duration.unwrap() < 1 * MICROSECOND, "Unlikely long duration");
        assert_eq!(encoded.timestamp.unwrap(), timestamp as i64);
    }
}
