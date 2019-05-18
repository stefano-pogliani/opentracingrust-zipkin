use std::time::Duration;
use std::time::Instant;

use reqwest::header::CONTENT_TYPE;
use reqwest::header::HeaderMap;
use reqwest::Client;
use reqwest::Response;
use thrift::protocol::TBinaryOutputProtocol;
use thrift::protocol::TListIdentifier;
use thrift::protocol::TOutputProtocol;
use thrift::protocol::TType;
use thrift::transport::TBufferedWriteTransport;

use opentracingrust::FinishedSpan;

use super::super::thrift_gen::zipkin_core::Endpoint;
use super::super::thrift_gen::zipkin_core::Span;
use super::super::Result;
use super::thrift_encode;

/// Submit finished spans to Zipkin over HTTP.
pub struct HttpCollector {
    endpoint: Endpoint,
    flush_count: usize,
    flush_last: Instant,
    flush_timeout: Duration,
    headers: HeaderMap,
    spans: Vec<Span>,
    target_url: String,
}

impl HttpCollector {
    pub fn new(opts: HttpCollectorOpts) -> HttpCollector {
        let target_url = opts.target_url();
        HttpCollector {
            endpoint: opts.endpoint,
            flush_count: opts.flush_count,
            flush_last: Instant::now(),
            flush_timeout: opts.flush_timeout,
            headers: opts.headers,
            spans: Vec::with_capacity(opts.flush_count),
            target_url,
        }
    }

    /// Append a span to the send buffer.
    pub fn collect(&mut self, span: FinishedSpan) {
        let span = thrift_encode(&span, &self.endpoint);
        self.spans.push(span);
    }

    /// Flush all buffered spans to zipkin.
    pub fn flush(&mut self) -> Result<Option<Response>> {
        self.flush_last = Instant::now();
        if self.spans.len() == 0 {
            return Ok(None);
        }

        // Grab the buffered spans and thrift-encode them.
        let mut spans = Vec::new();
        spans.append(&mut self.spans);
        let payload = self.encode(spans)?;

        // POST payload to Zipkin.
        let response = Client::new()
            .post(&self.target_url)
            .headers(self.headers.clone())
            .header(CONTENT_TYPE, "application/x-thrift")
            .body(payload)
            .send()?;
        Ok(Some(response))
    }

    /// Flush buffered spans if enough were collected or if the last flush was too long ago.
    ///
    /// The `flush_count` and `flush_timeout` options can be used to configure
    /// when a best effort flush actually occurs.
    ///
    /// You should call this method after collecting every span and periodically
    /// even if you do not receive spans to ensure the threashold are respected.
    pub fn lazy_flush(&mut self) -> Result<Option<Response>> {
        let flush = self.spans.len() > self.flush_count;
        let flush = flush || self.flush_last.elapsed() > self.flush_timeout;
        if flush {
            return self.flush();
        }
        Ok(None)
    }

    /// Thrift-encode the given spans for POSTing.
    fn encode(&mut self, spans: Vec<Span>) -> Result<Vec<u8>> {
        // Encode the thrift struct to memory.
        // Scoped so the mutable refernece to the buffer is released.
        let mut buffer: Vec<u8> = Vec::new();
        let len: i32 = spans.len() as i32;
        {
            let transport = TBufferedWriteTransport::new(&mut buffer);
            let mut protocol = TBinaryOutputProtocol::new(transport, true);
            protocol.write_list_begin(&TListIdentifier::new(TType::Struct, len))?;
            for span in spans {
                span.write_to_out_protocol(&mut protocol)?;
            }
            protocol.write_list_end()?;
            protocol.flush()?;
        }
        Ok(buffer)
    }
}

impl Drop for HttpCollector {
    fn drop(&mut self) {
        if let Err(error) = self.flush() {
            if !::std::thread::panicking() {
                panic!("Error flushing spans: {:?}", error);
            }
        }
    }
}

/// HTTP collector options.
pub struct HttpCollectorOpts<'a> {
    endpoint: Endpoint,
    flush_count: usize,
    flush_timeout: Duration,
    headers: HeaderMap,
    target: &'a str,
}

impl<'a> HttpCollectorOpts<'a> {
    /// Default HTTP collector options.
    ///
    /// The target URL to post spans to and the Zipkin endpoint descriptor are required.
    /// The target URL must NOT include the exact API endpoint and `/api/v1/spans` will be added.
    pub fn new<S>(url: S, endpoint: Endpoint) -> HttpCollectorOpts<'a>
    where
        S: Into<&'a str>,
    {
        HttpCollectorOpts {
            endpoint,
            flush_count: 1000,
            flush_timeout: Duration::from_secs(1),
            headers: HeaderMap::new(),
            target: url.into(),
        }
    }

    /// Set the number of buffered spans that should trigger a flush.
    ///
    /// A flush will be performed by the next call to `HttpCollector::lazy_flush` after
    /// the number of spans buffered is above this count.
    pub fn flush_count(mut self, count: usize) -> HttpCollectorOpts<'a> {
        self.flush_count = count;
        self
    }

    /// Set the muximum delay between span flushes.
    ///
    /// A fluss will be performed by the next call to `HttpCollector::lazy_flush`
    /// if there are spans in the buffer and the last flush is older then the timeout.
    pub fn flush_timeout(mut self, timeout: Duration) -> HttpCollectorOpts<'a> {
        self.flush_timeout = timeout;
        self
    }

    /// Set custom headers to attach to POST requests.
    pub fn headers(mut self, headers: HeaderMap) -> HttpCollectorOpts<'a> {
        self.headers = headers;
        self
    }

    /// Return the full URL to POST spans to.
    fn target_url(&self) -> String {
        format!("{}/api/v1/spans", self.target)
    }
}
