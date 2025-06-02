# dynlog

A minimal, low-overhead logging crate for Rust applications, designed for IoT and embedded Linux environments. It provides:

- ANSI-colored, structured logs (timestamp, level, file:line, message)
- Runtime log level control via:
  - Touch-file trigger (`--features file-trigger`)
  - Unix domain socket command (`--features socket-trigger`)
- Small binary size and low RAM/CPU usage
- Zero dependencies by default (optional `tokio` and `time`)

## Features

### Core Logging

Logs are printed in the format:

```
2025-06-01T12:34:56.789Z INFO  src/main.rs:42 Your log message
```

- Timestamps follow ISO 8601 format with millisecond precision.
- Log levels are color-coded: red for `error`, yellow for `warn`, green for `info`, etc.
- Output is written to `stdout` using `println!`.

---

## Feature Flags

You can enable one or more features for runtime log control.

### `file-trigger`

Allows controlling the log level by touching a file:

```bash
touch /tmp/<crate-name>-debug
```

This will change the log level to `debug` for the running process.

- Files are checked every 750ms in a background thread.
- Only affects the crate with the matching name (`CARGO_CRATE_NAME`).
- The file is automatically deleted after triggering.

### `socket-trigger`

Allows controlling the log level via Unix domain socket commands:

```bash
echo "info" | socat - /tmp/<crate-name>.sock
```

- Requires the `tokio` runtime.
- Supports commands: `error`, `warn`, `info`, `debug`, `trace`.
- Replies with `ok` or `error: invalid log level`.

---

## Usage Example

```rust
use dynlog::init;
use log::{debug, error, info, trace, warn};

#[tokio::main]
async fn main() {
    init().unwrap();
    dynlog::enable_socket_trigger().await;
    run();
}

fn run() {
    loop {
        info!("Service started");
        debug!("Debugging enabled");
        trace!("Tracing details");
        warn!("Warning issued");
        error!("Error occurred");
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
```

---

## Environment Variable

You can set the initial log level via `RUST_LOG`:

```bash
RUST_LOG=debug ./your-binary
```

---

## Building

Enable features as needed:

```bash
cargo build --release --features file-trigger
cargo build --release --features socket-trigger
cargo build --release --features "file-trigger socket-trigger"
```
