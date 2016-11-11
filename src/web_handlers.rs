/*  -------------------------------------------------------------
    Servers log microservice
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
    Project:        Nasqueron
    Created:        2016-11-10
    License:        BSD-2-Clause
    -------------------------------------------------------------    */

use iron::prelude::*;
use iron::status;

/*  -------------------------------------------------------------
    Handlers for web requests
    - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -    */

/// Serves a 200 ALIVE reply to allow basic service monitoring.
pub fn alive(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "ALIVE")))
}

/// Serves the log as a JSON document.
/// Serves a 503 reply as it's not implemented yet.
pub fn get(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::ServiceUnavailable, "Once upon a time, a log was baked.\n")))
}
