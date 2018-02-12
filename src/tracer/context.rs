use rand::random;

use opentracingrust::SpanReference;
use opentracingrust::SpanReferenceAware;

use super::trace_id::TraceID;


/// Zipkin specific `SpanContext`.
///
/// Carries information about the current trace.
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
    /// Returns a new context with the default options.
    pub fn new() -> ZipkinContext {
        ZipkinContext::new_with_options(ZipkinContextOptions::default())
    }

    /// Returns a new context with the given options.
    pub fn new_with_options(options: ZipkinContextOptions) -> ZipkinContext {
        let span_id = options.span_id.unwrap_or_else(random::<u64>);
        let trace_id = options.trace_id.unwrap_or_else(TraceID::new);
        ZipkinContext {
            debug: options.debug,
            sampled: options.sampled,
            span_id,
            trace_id,
        }
    }
}

impl ZipkinContext {
    /// Is the debug flag set?
    pub fn debug(&self) -> bool {
        self.debug
    }

    /// Is the context sampled?
    pub fn sampled(&self) -> bool {
        self.sampled
    }

    /// Access the context's span ID.
    pub fn span_id(&self) -> u64 {
        self.span_id
    }

    /// Access the context's trace ID.
    pub fn trace_id(&self) -> &TraceID {
        &self.trace_id
    }
}

impl SpanReferenceAware for ZipkinContext {
    fn reference_span(&mut self, reference: &SpanReference) {
        match *reference {
            SpanReference::FollowsFrom(ref context) |
            SpanReference::ChildOf(ref context) => {
                let context = context.impl_context::<ZipkinContext>().unwrap();
                self.debug = context.debug;
                self.sampled = context.sampled;
                self.trace_id = context.trace_id.clone();
            }
        }
    }
}


/// Additional options to seed a new span with.
pub struct ZipkinContextOptions {
    debug: bool,
    sampled: bool,
    span_id: Option<u64>,
    trace_id: Option<TraceID>,
}

impl ZipkinContextOptions {
    /// Sets the desired debug flag.
    pub fn debug(mut self, debug: bool) -> ZipkinContextOptions {
        self.debug = debug;
        self
    }

    /// Sets the desired sampling flag.
    pub fn sampled(mut self, sampled: bool) -> ZipkinContextOptions {
        self.sampled = sampled;
        self
    }

    /// Sets the desired span id.
    pub fn span_id(mut self, span_id: u64) -> ZipkinContextOptions {
        self.span_id = Some(span_id);
        self
    }

    pub fn trace_id(mut self, trace_id: TraceID) -> ZipkinContextOptions {
        self.trace_id = Some(trace_id);
        self
    }
}

impl Default for ZipkinContextOptions {
    fn default() -> ZipkinContextOptions {
        ZipkinContextOptions {
            debug: false,
            sampled: true,
            span_id: None,
            trace_id: None,
        }
    }
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::TraceID;
    use super::ZipkinContext;
    use super::ZipkinContextOptions;

    #[test]
    fn new_span_with_defaults() {
        let context = ZipkinContext::new();
        assert_eq!(context.debug, false);
        assert_eq!(context.sampled, true);
    }

    #[test]
    fn new_span_with_options() {
        let options = ZipkinContextOptions::default()
            .debug(true)
            .sampled(false)
            .span_id(42)
            .trace_id(TraceID::from_str("0102030405060708090a0b0c0d0e0f10").unwrap());
        let context = ZipkinContext::new_with_options(options);
        assert_eq!(context.debug, true);
        assert_eq!(context.sampled, false);
        assert_eq!(context.span_id, 42);
        assert_eq!(
            context.trace_id,
            TraceID::from_str("0102030405060708090a0b0c0d0e0f10").unwrap()
        );
    }

    mod references {
        use std::str::FromStr;
        use opentracingrust::ImplContextBox;
        use opentracingrust::SpanContext;
        use opentracingrust::SpanReference;
        use opentracingrust::SpanReferenceAware;

        use super::TraceID;
        use super::ZipkinContext;
        use super::ZipkinContextOptions;

        fn mock_context() -> SpanContext {
            let options = ZipkinContextOptions::default()
                .debug(true)
                .sampled(false)
                .trace_id(TraceID::from_str("0102030405060708090a0b0c0d0e0f10").unwrap());
            SpanContext::new(ImplContextBox::new(ZipkinContext::new_with_options(options)))
        }

        #[test]
        fn child_of_updates_details() {
            let child_of = mock_context();
            let child_of = SpanReference::ChildOf(child_of);
            let mut context = ZipkinContext::new();
            context.reference_span(&child_of);
            assert_eq!(context.debug, true);
            assert_eq!(context.sampled, false);
            assert_eq!(
                context.trace_id,
                TraceID::from_str("0102030405060708090a0b0c0d0e0f10").unwrap()
            );
        }

        #[test]
        fn follows_from_updates_details() {
            let follows_from = mock_context();
            let follows_from = SpanReference::FollowsFrom(follows_from);
            let mut context = ZipkinContext::new();
            context.reference_span(&follows_from);
            assert_eq!(context.debug, true);
            assert_eq!(context.sampled, false);
            assert_eq!(
                context.trace_id,
                TraceID::from_str("0102030405060708090a0b0c0d0e0f10").unwrap()
            );
        }
    }
}
