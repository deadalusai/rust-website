use iron::prelude::*;
use iron::status;

use maud::*;
use templates::Master;

pub fn handle(_: &mut Request) -> IronResult<Response> {

    let markup =
        Master::new("Home".into())
            .with_body(html! {
                h1 { "Home" }
                p { "Hello, user." }
            })
            .render_once();

    Ok(Response::with((status::Ok, markup)))
}