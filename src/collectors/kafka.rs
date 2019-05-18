use std::time::Duration;

use kafka::Result;
use kafka::producer::Producer;
use kafka::producer::Record;
use kafka::producer::RequiredAcks;

use opentracingrust::FinishedSpan;

use thrift::protocol::TBinaryOutputProtocol;
use thrift::protocol::TOutputProtocol;
use thrift::transport::TBufferedWriteTransport;

use super::super::ZipkinEndpoint;
use super::thrift_encode;


/// Collects finished spans to Zipkin over a Kafka topic.
pub struct KafkaCollector {
    endpoint: ZipkinEndpoint,
    producer: Producer,
    topic: String,
}

impl KafkaCollector {
    /// Create a new connector seeded by the given hosts.
    pub fn new(endpoint: ZipkinEndpoint, topic: String, hosts: Vec<String>) -> KafkaCollector {
        let producer = Producer::from_hosts(hosts)
            .with_ack_timeout(Duration::from_secs(1))
            .with_required_acks(RequiredAcks::One)
            .create()
            .unwrap();
        KafkaCollector {
            endpoint,
            producer,
            topic,
        }
    }

    /// Sends a finished span to Zipkin.
    // TODO: Change returned error type to wrap kafa and encoding errors.
    pub fn collect(&mut self, span: FinishedSpan) -> Result<()> {
        let encoded = thrift_encode(&span, &self.endpoint);
        let mut buffer: Vec<u8> = Vec::new();

        // Encode the thrift struct to memory.
        // Scoped so the mutable refernece to the buffer is released.
        {
            let transport = TBufferedWriteTransport::new(&mut buffer);
            let mut protocol = TBinaryOutputProtocol::new(transport, true);
            // TODO: propagate errors
            encoded.write_to_out_protocol(&mut protocol).unwrap();
            protocol.flush().unwrap();
        }

        // Send the message to kafka.
        let record = Record::from_value(&self.topic, buffer);
        self.producer.send(&record)?;
        Ok(())
    }
}
