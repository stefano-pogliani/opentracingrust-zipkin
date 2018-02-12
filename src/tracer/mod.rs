use crossbeam_channel::unbounded;

use opentracingrust::ImplContextBox;

use opentracingrust::ExtractFormat;
use opentracingrust::InjectFormat;

use opentracingrust::Result;
use opentracingrust::Span;
use opentracingrust::SpanContext;
use opentracingrust::SpanReceiver;
use opentracingrust::SpanSender;
use opentracingrust::StartOptions;

use opentracingrust::Tracer;
use opentracingrust::TracerInterface;


mod context;
mod error;
mod extract;
mod inject;
mod trace_id;

pub use self::context::ZipkinContext;
pub use self::context::ZipkinContextOptions;


/// A Zipkin backed OpenTracingRust tracer.
///
/// Use a collector to send the finished spans to Zipkin.
///
/// Sampling policy:
///
///   * Any span inherits the sampling state from its references.
///   * Root spans are always sampled: currently sampling policies are not supported.
pub struct ZipkinTracer {
    sender: SpanSender,
}

impl ZipkinTracer {
    /// Creates a new zipkin tracer.
    pub fn new() -> (Tracer, SpanReceiver) {
        let (sender, receiver) = unbounded();
        let tracer = Tracer::new(ZipkinTracer {
            sender,
        });
        (tracer, receiver)
    }
}

impl TracerInterface for ZipkinTracer {
    /// TODO: document formats.
    fn extract(&self, fmt: ExtractFormat) -> Result<Option<SpanContext>> {
        match fmt {
            ExtractFormat::Binary(carrier) => extract::binary(carrier),
            ExtractFormat::HttpHeaders(carrier) => extract::http_headers(carrier),
            ExtractFormat::TextMap(carrier) => extract::http_headers(carrier),
        }
    }

    /// TODO: document formats.
    fn inject(&self, context: &SpanContext, fmt: InjectFormat) -> Result<()> {
        match fmt {
            InjectFormat::Binary(carrier) => inject::binary(context, carrier),
            InjectFormat::HttpHeaders(carrier) => inject::http_headers(context, carrier),
            InjectFormat::TextMap(carrier) => inject::http_headers(context, carrier),
        }
    }

    fn span(&self, name: &str, options: StartOptions) -> Span {
        let context = ZipkinContext::new();
        let context = SpanContext::new(ImplContextBox::new(context));
        Span::new(name, context, options, self.sender.clone())
    }
}
