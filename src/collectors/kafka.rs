use opentracingrust::FinishedSpan;


/// Collects finished spans to Zipkin over a Kafka topic.
pub struct KafkaCollector {
    hosts: Vec<String>,
}

impl KafkaCollector {
    /// Create a new connector seeded by the given hosts.
    pub fn new(hosts: Vec<String>) -> KafkaCollector {
        KafkaCollector {
            hosts
        }
    }
}

impl KafkaCollector {
    /// Sends a finished span to Zipkin.
    pub fn collect(&self, span: FinishedSpan) -> Result<(), ()> {
        // TODO
        Ok(())
    }
}
