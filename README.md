# README.md

Welcome to `event-tracker`.

This is a simple program which runs a webserver and allows for log writing and retrieval.

In total, there are three endpoints, root, "/" health, "/health", and events, "/events". root and health are used for querying the status of the system and events which can be used to write events and read back events.

When the server is running it can be accessed at `http://localhost:3000/`

## System Architecture

This program implements a simple web server which writes single logs to a container and reads back a series of logs in a time range and optionally of a specific type.

It has been built on Linux and only tested on one OS.

The program operates on several assumptions, and these have been incorporated into the design.

1) Logs will be timestamped in milliseconds and will not be written any faster than 1kHz (i.e. we do not expect collisions when using millisecond timestamp as a key).
2) Reading logs (in a range) takes precedence over writing logs. This has two design implications:
   1) Logs will be maintained ordered, in a BTree, to allow for fast ordered retrieval.
   2) RwLock has been chosen over Mutex to allow for more reading than writing.

The above can be easily changed to Mutex/HashMap if requirements change. Additionally, if logs are expected more frequently than 1kHz, we can use microseconds.

Other points on design worth mentioning, `web.rs`, `types.rs`, and `storage.rs` have been completely isolated from on another for a modular, decoupled design.

Regarding storage, currently in a BTree. By wrapping the container and exposing read and write functions the underlying storage type can be easily swapped in the future. Options like HashMap, DashMap, or even a non-volatile database are perfectly valid options without out too much fuss.

IP rate limiting has been added using the `tower_governor` crate. It is configurable to allow integration testing to work quickly. Configuration can be found in `lib.rs` as per documentation found on crates.io. To test this, simply run the program, open `http://localhost:3000/` in your browser and refresh quickly (no, this test would not be used in production, but we're running out of daylight here).

## Events

### `write_event`

`write_event` is used to write a single log event to the ledger. It requires three parameters, a `log_type`, a `timestamp` and a `payload`. `log_type` comes from an enumerated list which can be found in `types.rs`. `timestamp` is a `u64` which cannot be greater than the current system time. `payload` is a free-form log which can be any well-formed json message.

An example of `write_event` could be:

```bash
curl -d "{\"log_type\":\"yyz\",\"timestamp\":$(date +%s%3N),\"payload\":{\"name\":\"luke\",\"color\":\"blue\"}}" \
     -H "Content-Type: application/json" \
     -X POST http://localhost:3000/events
```

### `read_events`

`read_events` is used to query the log database. It takes 3 optional parameters; `start`, `end`, and `log_type` as shown below.

```bash
curl -X GET 'http://localhost:3000/events?start=1749207505632&end=1749207515632&log_type=xxx'
```

The parameters are all optional. If a `log_type` is not specified, all log types are returned. If a `start` or `end` are not specified, the range will default to beginning to end.

## Testing

To run the program, simply build then run using.

```bash
cargo b
cargo r
```

With the server running open another terminal and execute the following series of commands. The sequence will write three logs using two different log types. The final command will query the logs for a specific log type (of which two were written).

After querying, you will receive a subset of the logs issued containing only filtered events.

```bash
curl -d "{\"log_type\":\"yyz\",\"timestamp\":$(date +%s%3N),\"payload\":{\"name\":\"luke\",\"color\":\"blue\"}}" \
     -H "Content-Type: application/json" \
     -X POST http://localhost:3000/events
```

```bash
curl -d "{\"log_type\":\"yyz\",\"timestamp\":$(date +%s%3N),\"payload\":{\"name\":\"luke\",\"food\":\"something good\"}}" \
     -H "Content-Type: application/json" \
     -X POST http://localhost:3000/events
```

```bash
curl -d "{\"log_type\":\"xyz\",\"timestamp\":$(date +%s%3N),\"payload\":{\"name\":\"luke\",\"food\":\"something good\"}}" \
     -H "Content-Type: application/json" \
     -X POST http://localhost:3000/events
```

```bash
curl -X GET 'http://localhost:3000/events?log_type=yyz'
```

A full set of unit and integration tests has been setup. Use the following command to execute test suite.

```bash
cargo nextest run
```

For test coverage, you can run the following. Current test coverage is above 90%.

```bash
cargo llvm-cov nextest --html
```
