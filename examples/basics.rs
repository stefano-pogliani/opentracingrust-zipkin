extern crate opentracingrust;
extern crate opentracingrust_zipkin;

use std::time::Duration;
use std::thread;

use opentracingrust::Log;
use opentracingrust::utils::GlobalTracer;
use opentracingrust::utils::ReporterThread;

use opentracingrust_zipkin::HttpCollector;
use opentracingrust_zipkin::HttpCollectorOpts;
use opentracingrust_zipkin::ZipkinEndpoint;
use opentracingrust_zipkin::ZipkinTracer;

fn main() {
    // Create a tracer and the reporter thread.
    let (tracer, receiver) = ZipkinTracer::new();
    GlobalTracer::init(tracer);

    println!("Setting up kafka collector ...");
    let options = HttpCollectorOpts::new(
        // Jaeger provides an all-in-one, stateless, zipkin compatible docker image.
        // While not zipkin, it is a very convenient tool for tests.
        //   https://www.jaegertracing.io/docs/1.12/getting-started/
        "http://localhost:9411",
        ZipkinEndpoint::new(None, None, Some(String::from("zipkin-example")), None),
    )
    .flush_count(4)
    .flush_timeout(Duration::from_millis(500));
    let mut collector = HttpCollector::new(options);
    let mut reporter = ReporterThread::new(receiver, move |span| {
        collector.collect(span);
        match collector.lazy_flush() {
            Err(err) => println!("[ERR] Failed to report span: {:?}", err),
            Ok(None) => println!("[OK] Span flushing delayed"),
            Ok(Some(mut response)) => {
                let meta = format!("{:?}", response);
                let body = format!("{:?}", response.text());
                println!("[OK] Response: {} - {}", meta, body);
            }
        }
    });
    reporter.stop_delay(Duration::from_secs(2));

    // Create the root span.
    let mut root_span = GlobalTracer::get().span("root");
    root_span.log(
        Log::new()
            .log("client.name", "opentracingrust")
            .log("client.version", 0.1)
    );
    let root_span = root_span.auto_finish();

    // Now spawn some threads that create spans.
    println!("Spawning threads ...");
    let mut threads: Vec<thread::JoinHandle<()>> = Vec::new();
    for i in 1..10 {
        let name = format!("Thread#{}", i);
        let context = root_span.context().clone();
        threads.push(thread::Builder::new().name(name).spawn(move || {
            // Create a span that will send itself when dropped.
            let mut span = GlobalTracer::get().span("thread");
            span.tag("index", i);
            span.child_of(context);
            let _span = span.auto_finish();

            // Sleep a bit and say good bye.
            thread::sleep(Duration::from_secs(i as u64));
            println!("Thread {} done", i);
        }).expect("Failed to spawn thread"));
    }

    // Wait for "worker" threads.
    for thread in threads {
        thread.join().unwrap();
    }
}
