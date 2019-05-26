use std::collections::HashMap;
use std::convert::TryFrom;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use opentracingrust::FinishedSpan;
use opentracingrust::LogValue;
use opentracingrust::TagValue;
use serde_json;

use super::thrift_gen::zipkin_core;
use super::tracer::ZipkinContext;

pub mod http;
#[cfg(feature = "kafka_transport")]
pub mod kafka;

const MICROSECOND: u64 = 1000000;

/// Computes the difference (in micro-seconds) between to system times.
fn compute_duration(start: SystemTime, end: SystemTime) -> i64 {
    let delta = match end.duration_since(start) {
        Ok(n) => n,
        _ => start.duration_since(end).unwrap(),
    };
    let secs = delta.as_secs() * MICROSECOND;
    let micros = u64::from(delta.subsec_micros());
    let delta = secs + micros;
    match i64::try_from(delta) {
        Err(_) => i64::max_value(),
        Ok(delta) => delta,
    }
}

/// Encode a log value into a String.
fn encode_log_value(value: &LogValue) -> String {
    match *value {
        LogValue::Boolean(true) => String::from("true"),
        LogValue::Boolean(false) => String::from("false"),
        LogValue::Float(value) => format!("{}", value),
        LogValue::Integer(value) => format!("{}", value),
        LogValue::String(ref value) => value.clone(),
    }
}

/// Encode a tag value into a bytes buffer.
fn encode_tag_value(value: &TagValue) -> (Vec<u8>, zipkin_core::AnnotationType) {
    let buffer = match *value {
        TagValue::Boolean(true) => String::from("true"),
        TagValue::Boolean(false) => String::from("false"),
        TagValue::Float(value) => format!("{}", value),
        TagValue::Integer(value) => format!("{}", value),
        TagValue::String(ref value) => value.clone(),
    }.into_bytes();
    (buffer, zipkin_core::AnnotationType::STRING)
}

/// Encodes a finished span into a thrift message for Zipkin.
pub fn thrift_encode(span: &FinishedSpan, endpoint: &zipkin_core::Endpoint) -> zipkin_core::Span {
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
    let mut binary_annotations = Vec::new();
    for (tag, value) in span.tags().iter() {
        // TODO: if the tag is a known zipkin tag, convert appropriately.
        let (buffer, value_type) = encode_tag_value(value);
        let annotation = zipkin_core::BinaryAnnotation::new(
            Some(tag.clone()),  // key
            Some(buffer),  // value
            Some(value_type),  // annotation_type
            Some(endpoint.clone())  // host
        );
        binary_annotations.push(annotation);
    }

    // Convert logs into annotations.
    let mut annotations = Vec::new();
    for log in span.logs() {
        let timestamp = compute_duration(UNIX_EPOCH, log.timestamp().unwrap().clone());
        let fields: HashMap<String, String> = log.iter()
            .map(|(key, value)|(key.clone(), encode_log_value(value)))
            .collect();
        // TODO: better error handling
        let fields = serde_json::to_string(&fields).unwrap();
        let annotation = zipkin_core::Annotation::new(
            Some(timestamp),  // timestamp
            Some(fields),  // value
            Some(endpoint.clone())  // host
        );
        annotations.push(annotation);
    }

    // Ensure at least an annotation is present to carry the endpoint information.
    if annotations.len() == 0 && binary_annotations.len() == 0 {
        let annotation = zipkin_core::BinaryAnnotation::new(
            Some("zipkin.endpoint.injected".into()),  // key
            Some("true".into()),  // value
            Some(zipkin_core::AnnotationType::STRING),  // annotation_type
            Some(endpoint.clone())  // host
        );
        binary_annotations.push(annotation);
    }

    // Create a thrift span.
    zipkin_core::Span::new(
        Some(low as i64),  // trace_id
        Some(span.name().clone()),  // name
        Some(context.span_id() as i64),  // id
        context.parent_span_id().map(|id| id as i64),  // parent_id
        Some(annotations),  // annotations
        Some(binary_annotations),  // binary_annotations
        Some(context.debug()),  // debug
        Some(timestamp),  // timestamp
        Some(duration),  // duration
        Some(high as i64)  // trace_id_high
    )
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;
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
        let endpoint = zipkin_core::Endpoint::new(
            None, None, Some(String::from("test-service")), None
        );
        let encoded = thrift_encode(&span, &endpoint);
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
        let timestamp = timestamp.as_secs() * MICROSECOND + u64::from(timestamp.subsec_micros());
        let timestamp = i64::try_from(timestamp).unwrap();
        assert_eq!(encoded.timestamp.unwrap(), timestamp);
    }
}
