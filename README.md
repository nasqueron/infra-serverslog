# Servers log microservice

Collects servers log entries and publish them as JSON

## Status

This is a work in progress, currently developed.

It's currently only suitable for the initial development.

## Run the service

The service can be configured
through the following environment variables:

  * BIND — the address the HTTP server should listen to,
           as a `ìp:port` expression (default: 0.0.0.0:3000)
  * RUST_LOG — the level of logging,
               `ìnfo` is recommended to get events stream (no default)
  * STORE — the path to the SQLite database file
            to store log (default: ./log.db)

## Build instructions

### Dependencies

To compile the service, you need:

  * Rust (we use the nightly distribution channel)
  * Cargo
  * SQLite 3 (we link against it, so the .so is required)

### Compile

Use `cargo build` to compile and link the debug version.

For the release, use `cargo build --release`.

### Bypass Cargo

If you bypass Cargo to compile, define the following
environment variables:

  * CARGO_PKG_NAME
  * CARGO_PKG_VERSION

### Tests

To run the tests, use `cargo test`.

Note this is an executable, not a library.
As such, if you want to add a test, you can't use a tests/ folder:
the independent test crate won't be able to import the crate.
