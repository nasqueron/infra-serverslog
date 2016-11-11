/*  -------------------------------------------------------------
    Servers log microservice

    Collects servers log entries and publish them as JSON
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    Project:        Nasqueron
    Created:        2016-11-10
    License:        BSD-2-Clause
    -------------------------------------------------------------    */

/*  -------------------------------------------------------------
    Crates
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -    */

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate iron;
#[macro_use]
extern crate router;

/*  -------------------------------------------------------------
    Modules
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -    */

mod http_server;
mod web_handlers;

/*  -------------------------------------------------------------
    Application entry point
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -    */

/// Initializes the main entry point of the service.
fn main() {
    env_logger::init().unwrap();

    info!("Initializing service {} v{}",
          env!("CARGO_PKG_NAME"),
          env!("CARGO_PKG_VERSION"));

    http_server::run();
}
