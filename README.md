# cyclonedds_zenoh_throughput

# Usage

## Subscribe

* Build CycloneDDS Throughput example and run

```shell
./build/bin/ThroughputPublisher 8 0 1 0 ""
```

* Build zenoh-bridge-dds and run (Use 0.7.0-rc)

```shell
./target/release/zenoh-bridge-dds
```

* Run

```shell
cargo run
```

## Publish

* Build CycloneDDS Throughput example and run

```shell
./build/bin/ThroughputSubscriber 0 0 ""
```

* Build zenoh-bridge-dds and run (Use 0.7.0-rc)

```shell
./target/release/zenoh-bridge-dds
```

* Run

```shell
cargo run
```

