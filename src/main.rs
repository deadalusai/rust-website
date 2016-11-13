#![feature(plugin)]
#![plugin(maud_macros)]

// TODO: Only during dev
// Cut down the noise while figuring things out
#![allow(unused_imports)]

extern crate iron;
extern crate iron_pipeline;
extern crate maud;
#[macro_use] extern crate router;

use iron::prelude::*;
use iron::status;

use iron_pipeline::prelude::*;

mod templates;
mod routes;

fn log_request(req: &Request) {
    println!("{} {}", req.method, req.url);
}

fn log_response(res: &Response) {
    match res.status {
        Some(status) => println!("{}", status),
        None         => println!("No response status set")
    }
}

fn main() {

    // Configure request pipeline
    let mut app = Pipeline::new();

    // This middleware runs on all requests
    app.add(HandleNext(|req, next| {
        // Log the request and response
        log_request(req);
        match next.process(req) {
            Ok(res) => {
                log_response(&res);
                Ok(res)
            },
            Err(err) => {
                println!("Error {}", err.error);
                log_response(&err.response);
                // Hack: custom error messages
                if err.error.is::<router::NoRoute>() {
                    let markup = templates::not_found();
                    let res = Response::with((status::NotFound, markup));
                    return Ok(res);
                }
                Err(err)
            }
        }
    }));

    app.add(routes::build());

    // Start webserver
    let port = 1337;
    let _listener =
        Iron::new(app)
            .http(("localhost", port))
            .unwrap();

    println!("Listening on port {}", port);
}
