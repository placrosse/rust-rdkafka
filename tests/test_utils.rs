extern crate rdkafka;
extern crate rand;
extern crate futures;

use rand::Rng;
use futures::*;

use rdkafka::client::Context;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel, TopicConfig};
use rdkafka::consumer::{Consumer, ConsumerContext};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::producer::FutureProducer;
use rdkafka::message::ToBytes;
use rdkafka::statistics::Statistics;

use std::collections::HashMap;

pub fn rand_test_topic() -> String {
    let id = rand::thread_rng()
        .gen_ascii_chars()
        .take(10)
        .collect::<String>();
    format!("__test_{}", id)
}

pub fn rand_test_group() -> String {
    let id = rand::thread_rng()
        .gen_ascii_chars()
        .take(10)
        .collect::<String>();
    format!("__test_{}", id)
}


// pub fn produce_messages<P, K, J, Q>(topic_name: &str, count: i32, value_fn: &P, key_fn: &K, partition: Option<i32>)
//                                     -> HashMap<(i32, i64), i32>
//     where P: Fn(i32) -> J,
//           K: Fn(i32) -> Q,
//           J: ToBytes,
//           Q: ToBytes {
//     produce_messages_with_context(EmptyContext, topic_name, count, value_fn, key_fn, partition)
// }

pub struct TestContext;

impl Context for TestContext {
    fn log(&self, _level: RDKafkaLogLevel, fac: &str, log_message: &str) {
        // log line received, calculate length
        let _n = fac.len() + log_message.len();
    }

    fn stats(&self, _: Statistics) { }
}

impl ConsumerContext for TestContext { }

pub fn produce_messages<P, K, J, Q>(topic_name: &str, count: i32, value_fn: &P, key_fn: &K,
                                    partition: Option<i32>, timestamp: Option<i64>)
        -> HashMap<(i32, i64), i32>
    where P: Fn(i32) -> J,
          K: Fn(i32) -> Q,
          J: ToBytes,
          Q: ToBytes {

    let prod_context = TestContext;

    // Produce some messages
    let producer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("statistics.interval.ms", "10000")
        .set("api.version.request", "true")
        .set_default_topic_config(TopicConfig::new()
            .set("produce.offset.report", "true")
            .set("message.timeout.ms", "30000")
            .finalize())
        .create_with_context::<TestContext, FutureProducer<_>>(prod_context)
        .expect("Producer creation error");

    let futures = (0..count)
        .map(|id| {
            let future = producer.send_copy(topic_name, partition, Some(&value_fn(id)), Some(&key_fn(id)), timestamp)
                .expect("Production failed");
            (id, future)
        }).collect::<Vec<_>>();

    let mut message_map = HashMap::new();
    for (id, future) in futures {
        match future.wait() {
            Ok(report) => match report.result() {
                Err(e) => panic!("Delivery failed: {}", e),
                Ok((partition, offset)) => message_map.insert((partition, offset), id),
            },
            Err(e) => panic!("Waiting for future failed: {}", e)
        };
    }

    message_map
}


// Create consumer
pub fn create_stream_consumer(topic_name: &str, group_id: &str) -> StreamConsumer<TestContext> {
    let cons_context = TestContext;

    let consumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("client.id", "rdkafka_integration_test_client")
        .set("bootstrap.servers", "localhost:9092")
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "false")
        .set("statistics.interval.ms", "10000")
        .set("api.version.request", "true")
        .set_default_topic_config(
            TopicConfig::new()
                .set("auto.offset.reset", "earliest")
                .finalize()
        )
        .create_with_context::<TestContext, StreamConsumer<_>>(cons_context)
        .expect("Consumer creation failed");
    consumer.subscribe(&vec![topic_name]).unwrap();
    consumer
}

pub fn value_fn(id: i32) -> String {
    format!("Message {}", id)
}

pub fn key_fn(id: i32) -> String {
    format!("Key {}", id)
}
