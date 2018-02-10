use std::io::Write;

use opentracingrust::MapCarrier;
use opentracingrust::Result;
use opentracingrust::SpanContext;


/// TODO
pub fn binary(context: &SpanContext, carrier: Box<&mut Write>) -> Result<()> {
    // TODO
    Ok(())
}


/// TODO
pub fn http_headers(context: &SpanContext, carrier: Box<&mut MapCarrier>) -> Result<()> {
    // TODO
    Ok(())
}
