use std::collections::BTreeMap;
use std::io::Write;

use opentracingrust::MapCarrier;
use opentracingrust::Result;
use opentracingrust::SpanContext;

use thrift::protocol::TBinaryOutputProtocol;
use thrift::protocol::TOutputProtocol;
use thrift::transport::TBufferedWriteTransport;

use super::context::ZipkinContext;
use super::error::thrift_error;
use super::super::thrift_gen::binary_format;


/// Encode the SpanContext into a thrift structure.
pub fn binary(context: &SpanContext, carrier: Box<&mut Write>) -> Result<()> {
    let items: BTreeMap<String, String> = context.baggage_items()
        .map(|(k, v)| (k.clone(), v.clone())).collect();
    let inner_context = context.impl_context::<ZipkinContext>().expect(
        "Invalid SpanContext, was it created by ZipkinTracer?"
    );
    let (high, low) = inner_context.trace_id().split();
    let span_id = inner_context.span_id();
    let sampled = inner_context.sampled();
    let flags = match inner_context.debug() {
        false => 0,
        true => 1,
    };
    let thrift_context = binary_format::SpanContext::new(
        Some(high as i64),     // Trace ID
        Some(low as i64),      // Trace ID (High)
        Some(span_id as i64),  // Span ID
        Some(sampled),         // Sampled?
        Some(flags),           // Flags
        Some(items)            // Baggage Items
    );
    let transport = TBufferedWriteTransport::new(carrier);
    let mut protocol = TBinaryOutputProtocol::new(transport, true);
    thrift_context.write_to_out_protocol(&mut protocol).map_err(thrift_error)?;
    protocol.flush().map_err(thrift_error)
}


/// Encode the SpanContext into HTTP Headers following the B3 format.
///
/// See https://github.com/openzipkin/b3-propagation
pub fn http_headers(context: &SpanContext, carrier: Box<&mut MapCarrier>) -> Result<()> {
    // TODO
    Ok(())
}


#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use opentracingrust::ImplContextBox;
    use opentracingrust::SpanContext;

    use thrift::protocol::TBinaryInputProtocol;
    use thrift::transport::TBufferedReadTransport;

    use try_from::TryFrom;

    use super::super::context::ZipkinContext;
    use super::super::context::ZipkinContextOptions;
    use super::super::trace_id::TraceID;
    use super::binary;
    use super::binary_format;

    fn make_context() -> SpanContext {
        let options = ZipkinContextOptions::default()
            .debug(true)
            .sampled(true)
            .span_id(42)
            .trace_id(TraceID::try_from("0102030405060708090a0b0c0d0e0f10").unwrap());
        let context = ZipkinContext::new_with_options(options);
        let context = ImplContextBox::new(context);
        let mut context = SpanContext::new(context);
        context.set_baggage_item(String::from("a"), String::from("1"));
        context.set_baggage_item(String::from("b"), String::from("2"));
        context.set_baggage_item(String::from("c"), String::from("3"));
        context
    }

    #[test]
    fn test_binary_encoding() {
        // Encode the context in memory.
        let context = make_context();
        let mut buffer: Vec<u8> = Vec::new();
        let result = binary(&context, Box::new(&mut buffer));
        result.unwrap();
        assert_ne!(buffer.len(), 0);

        // Decode the context from memory.
        let transport = TBufferedReadTransport::new(Cursor::new(buffer));
        let mut protocol = TBinaryInputProtocol::new(transport, true);
        let result = binary_format::SpanContext::read_from_in_protocol(&mut protocol);
        let context = result.unwrap();

        // Validate content.
        assert_eq!(context.trace_id.unwrap(), 72623859790382856);
        assert_eq!(context.trace_id_high.unwrap(), 651345242494996240);
        assert_eq!(context.span_id.unwrap(), 42);
        assert_eq!(context.sampled.unwrap(), true);
        assert_eq!(context.flags.unwrap(), 1);
        let baggage_items = context.baggage_items.unwrap();
        let mut items: Vec<(String, String)> = baggage_items.iter()
            .map(|(k, v)| (k.clone(), v.clone())).collect();
        items.sort();
        assert_eq!(items, [
            (String::from("a"), String::from("1")),
            (String::from("b"), String::from("2")),
            (String::from("c"), String::from("3")),
        ]);
    }
}
