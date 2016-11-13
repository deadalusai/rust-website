use iron::prelude::*;
use iron::status;
use router::Router;

use templates;

pub fn build() -> Router {
    router!{
        index: get "/" => index
    }
}

fn index(req: &mut Request) -> IronResult<Response> {
    let markup = templates::hello_world("User");
    Ok(Response::with((status::Ok, markup)))
}