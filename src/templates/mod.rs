pub mod markup;
pub mod master;

use maud::*;

pub use self::master::Master;

pub fn not_found() -> Markup {
    Master::new("404 Not Found")
    .with_body(html! {
        h1 "Whoops! 404"
        p "Whatever you were looking for, we couldn't find it."
    })
    .render_once()
}