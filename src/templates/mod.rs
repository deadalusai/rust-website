use maud::*;

pub fn hello_world(name: &str) -> Markup {
    html! {
        p { "Hello, " (name) "." }
    }
}