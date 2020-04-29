use std::io::Read;

use opentracingrust::Error;
use opentracingrust::ImplContextBox;
use opentracingrust::MapCarrier;
use opentracingrust::Result;
use opentracingrust::SpanContext;

use thrift::protocol::TBinaryInputProtocol;
use thrift::transport::TBufferedReadTransport;

use super::context::ZipkinContext;
use super::context::ZipkinContextOptions;

use super::error::data_encoding_error;
use super::error::thrift_error;
use super::trace_id::TraceID;

use super::super::thrift_gen::binary_format;


/// Decode the SpanContext from a thrift structure.
pub fn binary(carrier: Box<&mut dyn Read>) -> Result<Option<SpanContext>> {
    let transport = TBufferedReadTransport::new(carrier);
    let mut protocol = TBinaryInputProtocol::new(transport, true);
    let result = binary_format::SpanContext::read_from_in_protocol(&mut protocol);
    let message = result.map_err(thrift_error)?;

    // Decode the TraceID from a (low, high) tuple.
    let trace_id_low = message.trace_id.ok_or(
        Error::Msg(String::from("Decoded context does not have a TraceID (low)"))
    )?;
    let trace_id_high = message.trace_id_high.ok_or(
        Error::Msg(String::from("Decoded context does not have a TraceID (high)"))
    )?;
    let trace_id = TraceID::join(trace_id_high as u64, trace_id_low as u64);
    let span_id = message.span_id.ok_or(
        Error::Msg(String::from("Decoded context does not have a SpanID"))
    )?;

    // Build the SpanContext.
    let options = ZipkinContextOptions::default()
        .debug(message.flags.unwrap_or(0) == 1)
        .span_id(span_id as u64)
        .trace_id(trace_id);

    let options = match message.parent_span_id {
        None => options,
        Some(parent_span_id) => options.parent_span_id(parent_span_id as u64),
    };
    let options = match message.sampled {
        None => {
            // TODO: once Samplers are implemented, call it here?
            options.sampled(true)
        },
        Some(sampled) => options.sampled(sampled),
    };

    let context = ZipkinContext::new_with_options(options);
    let context = ImplContextBox::new(context);
    let mut context = SpanContext::new(context);

    if let Some(baggage_items) = message.baggage_items {
        for (key, value) in baggage_items {
            context.set_baggage_item(key, value);
        }
    }
    Ok(Some(context))
}


/// Decode the SpanContext from HTTP Headers.
///
/// The decoding is done following the B3 propagation format.
/// See https://github.com/openzipkin/b3-propagation
///
/// Baggage items are expected to be in the format `OT-Baggage-{Key}: {Value}`.
pub fn http_headers(carrier: Box<&dyn MapCarrier>) -> Result<Option<SpanContext>> {
    // Trace ID.
    let trace_id = match carrier.get("X-B3-TraceId") {
        Some(trace_id) => trace_id,
        None => return Ok(None)
    };
    let trace_id: TraceID = trace_id.parse().map_err(data_encoding_error)?;

    // Span ID.
    let span_id = carrier.get("X-B3-SpanId").ok_or(
        Error::Msg(String::from("Decoded context does not have a SpanID"))
    )?;
    let span_id = u64::from_str_radix(&span_id, 16)?;

    // Build the SpanContext.
    let options = ZipkinContextOptions::default()
        .debug(carrier.get("X-B3-Flags").unwrap_or(String::from("0")) == "1")
        .span_id(span_id)
        .trace_id(trace_id);

    // Parent Span ID.
    let options = match carrier.get("X-B3-ParentSpanId") {
        None => options,
        Some(parent_span_id) => {
            let parent_span_id = u64::from_str_radix(&parent_span_id, 16)?;
            options.parent_span_id(parent_span_id)
        }
    };
    let options = match carrier.get("X-B3-Sampled") {
        None => {
            // TODO: once Samplers are implemented, call it here?
            options.sampled(true)
        },
        Some(sampled) => options.sampled(sampled == "1"),
    };

    let context = ZipkinContext::new_with_options(options);
    let context = ImplContextBox::new(context);
    let mut context = SpanContext::new(context);

    for (key, value) in carrier.items() {
        if key.starts_with("OT-Baggage-") {
            context.set_baggage_item(String::from(&key[11..]), value.clone());
        }
    }
    Ok(Some(context))
}


#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::io::Cursor;
    use std::str::FromStr;

    use thrift::protocol::TBinaryOutputProtocol;
    use thrift::protocol::TOutputProtocol;
    use thrift::transport::TBufferedWriteTransport;

    use super::super::context::ZipkinContext;
    use super::super::trace_id::TraceID;

    use super::binary;
    use super::binary_format;
    use super::http_headers;

    #[test]
    fn test_binary_decoding() {
        // Mock a thrift structure.
        let mut items: BTreeMap<String, String> = BTreeMap::new();
        items.insert(String::from("a"), String::from("1"));
        items.insert(String::from("b"), String::from("2"));
        items.insert(String::from("c"), String::from("3"));
        let thrift_context = binary_format::SpanContext::new(
            Some(651345242494996240), // Trace ID
            Some(72623859790382856),  // Trace ID (High)
            Some(42),                 // Span ID
            Some(42),                 // Parent Span ID
            Some(true),               // Sampled?
            Some(1),                  // Flags
            Some(items)               // Baggage Items
        );

        // Encode it in memory.
        let mut buffer: Vec<u8> = Vec::new();
        {
            let transport = TBufferedWriteTransport::new(&mut buffer);
            let mut protocol = TBinaryOutputProtocol::new(transport, true);
            thrift_context.write_to_out_protocol(&mut protocol).unwrap();
            protocol.flush().unwrap();
        }

        // Decode buffer into a SpanContext.
        let mut cursor = Cursor::new(buffer);
        let context = binary(Box::new(&mut cursor)).unwrap().unwrap();

        // Check content.
        let inner = context.impl_context::<ZipkinContext>().unwrap();
        assert_eq!(inner.debug(), true);
        assert_eq!(inner.parent_span_id().unwrap(), 42);
        assert_eq!(inner.sampled(), true);
        assert_eq!(inner.span_id(), 42);
        assert_eq!(
            inner.trace_id(),
            &TraceID::from_str("0102030405060708090a0b0c0d0e0f10").unwrap()
        );

        let mut items: Vec<(String, String)> = context.baggage_items()
            .map(|(k, v)| (k.clone(), v.clone())).collect();
        items.sort();
        assert_eq!(items, vec![
            (String::from("a"), String::from("1")),
            (String::from("b"), String::from("2")),
            (String::from("c"), String::from("3"))
        ]);
    }

    #[test]
    fn test_http_headers_decoding() {
        // Build a headers map.
        let mut headers: BTreeMap<String, String> = BTreeMap::new();
        headers.insert(
            String::from("X-B3-TraceId"),
            String::from("0102030405060708090a0b0c0d0e0f10")
        );
        headers.insert(String::from("X-B3-SpanId"), String::from("2a"));
        headers.insert(String::from("X-B3-ParentSpanId"), String::from("2a"));
        headers.insert(String::from("X-B3-Flags"), String::from("1"));
        headers.insert(String::from("X-B3-Sampled"), String::from("1"));
        headers.insert(String::from("OT-Baggage-a"), String::from("1"));
        headers.insert(String::from("OT-Baggage-b"), String::from("2"));
        headers.insert(String::from("OT-Baggage-c"), String::from("3"));

        // Check content.
        let context = http_headers(Box::new(&mut headers)).unwrap().unwrap();
        let inner = context.impl_context::<ZipkinContext>().unwrap();
        assert_eq!(
            inner.trace_id(),
            &TraceID::from_str("0102030405060708090a0b0c0d0e0f10").unwrap()
        );
        assert_eq!(inner.span_id(), 42);
        assert_eq!(inner.parent_span_id().unwrap(), 42);
        assert_eq!(inner.debug(), true);
        assert_eq!(inner.sampled(), true);

        let mut items: Vec<(String, String)> = context.baggage_items()
            .map(|(k, v)| (k.clone(), v.clone())).collect();
        items.sort();
        assert_eq!(items, vec![
            (String::from("a"), String::from("1")),
            (String::from("b"), String::from("2")),
            (String::from("c"), String::from("3"))
        ]);
    }
}
