extern crate opentracingrust;
extern crate opentracingrust_zipkin;

use std::time::Duration;
use std::thread;

use opentracingrust::utils::GlobalTracer;
use opentracingrust::utils::ReporterThread;

use opentracingrust_zipkin::ZipkinTracer;
use opentracingrust_zipkin::KafkaCollector;


fn main() {
    // Create a tracer and the reporter thread.
    let (tracer, receiver) = ZipkinTracer::new();
    GlobalTracer::init(tracer);

    let collector = KafkaCollector::new("localhost:9092");
    let mut reporter = ReporterThread::new(receiver, move |span| {
        collector.collect(span).unwrap();
    });
    reporter.stop_delay(Duration::from_secs(2));

    // Now spawn some threads that create spans.
    let mut threads: Vec<thread::JoinHandle<()>> = Vec::new();
    for i in 1..10 {
        let name = format!("Thread#{}", i);
        threads.push(thread::Builder::new().name(name).spawn(move || {
            // Create a span that will send itself when dropped.
            let mut span = GlobalTracer::get().span("thread");
            span.tag("index", i);
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
