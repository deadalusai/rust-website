extern crate iron;
extern crate iron_pipeline;
extern crate maud;

use iron::prelude::*;
use iron::status;
use iron::middleware::{ Handler };

use iron_pipeline::prelude::*;
use iron_pipeline::{ Middleware, PipelineNext };

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
                Err(err)
            }
        }
    }));

    app.add(Handle(|_| {
        Ok(Response::with((status::Ok, "Hello, World")))
    }));

    // Start webserver
    let port = 1337;
    let _listener =
        Iron::new(app)
            .http(("localhost", port))
            .unwrap();

    println!("Listening on port {}", port);
}
