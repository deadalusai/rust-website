use maud::*;

pub fn not_found() -> Markup {
    html! {
        h1 "Whoops! 404"
        p "Whatever you were looking for, we couldn't find it."
    }
}

pub fn hello_world(name: &str) -> Markup {
    html! {
        p { "Hello, " (name) "." }
    }
}