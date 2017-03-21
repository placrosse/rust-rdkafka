initSidebarItems({"struct":[["BaseProducer","Simple Kafka producer. This producer needs to be `poll`ed at regular intervals in order to serve queued delivery report callbacks. This producer can be cheaply cloned to create a new reference to the same underlying producer. Data production should be done using the `BaseProducerTopic`, that can be created from this producer."],["BaseProducerTopic","Represents a Kafka topic with an associated `BaseProducer`. This struct can be cheaply cloned, generating a new reference to the same underlying resource, and can be used across different threads."],["DeliveryFuture","A future that will receive a `DeliveryReport` containing information on the delivery status of the message."],["DeliveryReport","Information returned by the producer after a message has been delivered or failed to be delivered."],["EmptyProducerContext","Simple empty producer context that can be use when the producer context is not required."],["FutureProducer","A producer with an internal running thread. This producer doesn't need to be polled. The internal thread can be terminated with the `stop` method or moving the `FutureProducer` out of scope."],["FutureProducerContext","The `ProducerContext` used by the `FutureProducer`. This context will use a Future as its `DeliveryContext` and will complete the future when the message is delivered (or failed to)."],["FutureProducerTopic","Represents a Kafka topic created by a `FutureProducer`. This struct can be cheaply cloned, generating a new reference to the same underlying resource, and can be used across different threads."]],"trait":[["ProducerContext","A `ProducerContext` is a `Context` specific for producers. It can be used to store user-specified callbacks, such as `delivery`."]]});