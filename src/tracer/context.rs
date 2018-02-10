use opentracingrust::SpanReference;
use opentracingrust::SpanReferenceAware;


/// TODO
#[derive(Clone)]
pub struct ZipkinContext {
    // TODO
}

impl ZipkinContext {
    /// TODO
    pub fn new() -> ZipkinContext {
        ZipkinContext {
            // TODO
        }
    }
}

impl SpanReferenceAware for ZipkinContext {
    fn reference_span(&mut self, reference: &SpanReference) {
        // TODO
    }
}
