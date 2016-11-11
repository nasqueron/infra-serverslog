/*  -------------------------------------------------------------
    Servers log microservice
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    Project:        Nasqueron
    Created:        2016-11-10
    License:        BSD-2-Clause
    -------------------------------------------------------------    */

use iron::prelude::*;
use router::Router;

use std::env;

/*  -------------------------------------------------------------
    HTTP server

    Spawns a HTTP server listens to $BIND environment variable
    or by default to *:3000.

    The server is based on the Iron framework.

    Controller logic is defined in the web_handlers module.
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -    */

/// Prepares an Iron handler.
fn initialize_handler() -> Router {
    router!(
        index: get "/" => ::web_handlers::get,
        status: get "/status" => ::web_handlers::alive
    )
}

/// Gets the address the HTTP server should bind to.
fn get_bind_address() -> String {
    env::var("BIND").unwrap_or("0.0.0.0:3000".to_string())
}

/// Runs an HTTP server.
pub fn run() {
    let listen = get_bind_address();

    info!("Starting HTTP server");
    info!("Listen to http://{}", listen);

    let _server = Iron::new(initialize_handler()).http(&*listen);
    if let Err(err) = _server {
        error!("{}", err)
    }
}

/*  -------------------------------------------------------------
    Tests
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -    */

#[cfg(test)]
mod tests {
    extern crate iron_test;

    use super::get_bind_address;
    use super::initialize_handler;

    use iron::Headers;
    use iron_test::request;
    use iron_test::response::extract_body_to_bytes;

    #[test]
    fn get_bind_address_returns_expected_default_value() {
        assert_eq!("0.0.0.0:3000", get_bind_address());
    }

    #[test]
    fn test_alive() {
        let response = request::get("http://localhost:3000/status",
                                    Headers::new(),
                                    &initialize_handler());
        let result = extract_body_to_bytes(response.unwrap());

        assert_eq!(result, b"ALIVE");
    }
}
