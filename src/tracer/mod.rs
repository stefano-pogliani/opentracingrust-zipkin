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
pub use self::context::ZipkinContext;


/// TODO
pub struct ZipkinTracer {
    sender: SpanSender,
}

impl ZipkinTracer {
    /// TODO
    pub fn new(host: &str) -> (Tracer, SpanReceiver) {
        let (sender, receiver) = unbounded();
        let tracer = Tracer::new(ZipkinTracer {
            sender,
        });
        (tracer, receiver)
    }
}

impl TracerInterface for ZipkinTracer {
    fn extract(&self, fmt: ExtractFormat) -> Result<Option<SpanContext>> {
        // TODO
        Ok(None)
    }

    fn inject(&self, context: &SpanContext, fmt: InjectFormat) -> Result<()> {
        // TODO
        Ok(())
    }

    fn span(&self, name: &str, options: StartOptions) -> Span {
        let context = ZipkinContext::new();
        let context = SpanContext::new(ImplContextBox::new(context));
        Span::new(name, context, options, self.sender.clone())
    }
}
