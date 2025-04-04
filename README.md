# ![D from Danube](Danube_logo_2.png) Danube

Danube is an open-source distributed Messaging Broker platform (inspired by Apache Pulsar).

Danube aims to be a simple yet powerful, secure, flexible and scalable messaging platform, suitable for event-driven applications. It supports both message queueing and fan-out pub-sub systems, making it versatile for various use cases.

Check-out [the Docs](https://danube-docs.dev-state.com/) for more details of the Danube Architecture and the supported concepts.

## Core Capabilities of the Danube messaging Platform

* [**Topics**](https://danube-docs.dev-state.com/architecture/topics/): A unit of storage that organizes messages into a stream.
  * **Non-partitioned topics**: One topic that is served by a single broker from the cluster.
  * **Partitioned topics**: Divided into partitions, served by different brokers from the cluster, enhancing scalability and fault tolerance.
* [**Message Dispatch**](https://danube-docs.dev-state.com/architecture/dispatch_strategy/):
  * **Non-reliable Message Dispatch**: Messages reside in memory and are promptly distributed to consumers, ideal for scenarios where speed is crucial. The acknowledgement mechanism is ignored.
  * **Reliable Message Dispatch**: The acknowledgement mechanism is used to ensure message delivery. Supports configurable storage options as `Local Disk` and `GRPC connected storages`, ensuring message persistence and durability.
* [**Subscription Types:**](https://danube-docs.dev-state.com/architecture/subscriptions/):
  * Supports various subscription types (**Exclusive**, **Shared**, **Failover**) enabling different messaging patterns such as message queueing and pub-sub.
* **Flexible Message Schemas**
  * Supports multiple message schemas (**Bytes**, **String**, **Int64**, **JSON**) providing flexibility in message format and structure.

## Clients

Allows single or multiple Producers to publish on the Topic and multiple Subscriptions to consume the messages from the Topic.

![Producers  Consumers](https://danube-docs.dev-state.com/architecture/img/producers_consumers.png "Producers Consumers")

You can combine the [Subscription Type mechanisms](https://danube-docs.dev-state.com/architecture/Queuing_PubSub_messaging/) in order to obtain message queueing or fan-out pub-sub messaging systems.

Currently, the Danube client libraries are written in:

* [Rust Client](https://crates.io/crates/danube-client) - the Rust [examples](danube-client/examples/) on how to create and use the Producers / Consumers
* [Go Client](https://pkg.go.dev/github.com/danrusei/danube-go) - the Go [examples](https://github.com/danrusei/danube-go/tree/main/examples) on how to create and use the Producers / Consumers

### Community supported clients

Contributions in other languages, such as Python, Java, etc., are also greatly appreciated. If there are any I'll add in this section.

## Danube CLIs

* **Command-Line Interfaces (CLI)**
  * [**Danube CLI**](https://github.com/danube-messaging/danube/tree/main/danube-cli): For handling message publishing and consumption.
  * [**Danube Admin CLI**](https://github.com/danube-messaging/danube/tree/main/danube-admin-cli): For managing and interacting with the Danube cluster, including broker, namespace, and topic management.

## Getting Started

To run Danube Broker on your local machine, follow the steps below:

### Run Metadata Store (ETCD)

Danube stores **Metadata** in an external database, in order to offer high availability and scalability, currently supported by [ETCD](https://etcd.io/).

```bash
docker run -d --name etcd-danube -p 2379:2379 quay.io/coreos/etcd:latest etcd --advertise-client-urls http://0.0.0.0:2379 --listen-client-urls http://0.0.0.0:2379
```

```bash
$ docker ps
CONTAINER ID   IMAGE                        COMMAND                  CREATED          STATUS          PORTS                                                 NAMES
27792bce6077   quay.io/coreos/etcd:latest   "etcd --advertise-cl…"   35 seconds ago   Up 34 seconds   0.0.0.0:2379->2379/tcp, :::2379->2379/tcp, 2380/tcp   etcd-danube
```

### Run Danube Broker on the Local Machine

Create a local config file with the contents from the [sample config file](https://github.com/danube-messaging/danube/blob/main/config/danube_broker.yml).

```bash
touch danube_broker.yml
```

and use your editor of choice to edit the file, by adding the sample  config file contents.

Download the latest Danube Broker binary from the [releases](https://github.com/danube-messaging/danube/releases) page and run it:

```bash
touch broker.log
```

```bash
RUST_LOG=info ./danube-broker-linux --config-file danube_broker.yml --broker-addr "0.0.0.0:6650" --admin-addr "0.0.0.0:50051" > broker.log 2>&1 &
```

Check the logs:

```bash
tail -n 100 -f broker.log
```

```bash
2025-02-23T05:45:14.019650Z  INFO danube_broker::broker_metrics: Initializing metrics exporter
2025-02-23T05:45:14.021220Z  INFO danube_broker: Initializing ETCD as metadata persistent store
2025-02-23T05:45:14.021658Z  INFO danube_broker: Initializing In-Memory Storage (Cache entries: 100, TTL: 10min) for message persistence
2025-02-23T05:45:14.022061Z  INFO danube_broker: Initializing Danube Message Broker service on 0.0.0.0:6650
2025-02-23T05:45:14.022095Z  INFO danube_broker::danube_service: Initializing Danube cluster 'MY_CLUSTER'
2025-02-23T05:45:14.028492Z  INFO danube_broker::danube_service::local_cache: Initial cache populated
2025-02-23T05:45:14.032731Z  INFO danube_broker::danube_service: Local Cache service initialized and ready
2025-02-23T05:45:14.039726Z  INFO danube_broker::danube_service::broker_register: Broker 15789098141031633884 registered in the cluster
2025-02-23T05:45:14.055625Z  INFO danube_broker::danube_service: Namespace default already exists.
2025-02-23T05:45:14.055672Z  INFO danube_broker::danube_service: Cluster metadata initialization completed successfully
2025-02-23T05:45:14.057516Z  INFO danube_broker::danube_service: Broker gRPC server listening on 0.0.0.0:6650
2025-02-23T05:45:14.057652Z  INFO danube_broker::danube_service: Leader Election service initialized and ready
2025-02-23T05:45:14.068276Z  INFO danube_broker::danube_service: Load Manager service initialized and ready
2025-02-23T05:45:14.072362Z  INFO danube_broker::danube_service: Admin gRPC server listening on 0.0.0.0:50051
```

### Use Danube CLI to Publish and Consume Messages

Download the latest Danube CLI binary from the [releases](https://github.com/danube-messaging/danube/releases) page and run it:

```bash
./danube-cli-linux produce -s http://127.0.0.1:6650 -t /default/demo_topic -c 1000 -m "Hello, Danube!"
```

```bash
Message sent successfully with ID: 9
Message sent successfully with ID: 10
Message sent successfully with ID: 11
Message sent successfully with ID: 12
```

Open a new terminal and run the following command to consume the messages:

```bash
./danube-cli-linux consume -s http://127.0.0.1:6650 -t /default/demo_topic -m my_subscription
```

```bash
Received bytes message: 9, with payload: Hello, Danube!
Received bytes message: 10, with payload: Hello, Danube!
Received bytes message: 11, with payload: Hello, Danube!
Received bytes message: 12, with payload: Hello, Danube!
```

For detailed instructions on how to run Danube Broker on different platforms, please refer to the [documentation](https://danube-docs.dev-state.com/).

### Tear Down

Stop the Danube Broker and the ETCD container:

```bash
pkill danube-broker
```

```bash
docker stop etcd-danube
```

```bash
docker rm -f etcd-danube
```

```bash
ps aux | grep danube-broker
docker ps | grep etcd-danube
```

## Development environment

Continuously working on enhancing and adding new features.

**Contributions are welcome**, check [the open issues](https://github.com/danube-messaging/danube/issues) or report a bug you encountered or a needed feature.

**The crates part of the Danube workspace**:

* danube-broker - The main crate, danube pubsub platform
  * danube-reliable-dispatch - Responsible of reliable dispatching, that stores and forward the messages to the subscribers
  * danube-persistent-storage - Responsible of persistent storage, supports `Local Disk` or `GRPC connected storages`
  * danube-metadata-store - Responsibile of Metadata storage, that stores and syncronizes the metadata across the Danube cluster
* danube-client - An async Rust client library for interacting with Danube Pub/Sub messaging platform
* danube-cli - Client CLI to handle message publishing and consumption
* danube-admin-cli - Admin CLI designed for interacting with and managing the Danube cluster

[Follow the instructions](https://danube-docs.dev-state.com/development/dev_environment/) on how to setup the development environment.
