use std::io::Read;

use opentracingrust::MapCarrier;
use opentracingrust::Result;
use opentracingrust::SpanContext;


/// TODO
pub fn binary(carrier: Box<&mut Read>) -> Result<Option<SpanContext>> {
    // TODO
    Ok(None)
}


/// TODO
pub fn http_headers(carrier: Box<&MapCarrier>) -> Result<Option<SpanContext>> {
    // TODO
    Ok(None)
}
