use maud::*;
use std::borrow::Cow;

use templates::markup::Css;

/// Master page
pub struct Master {
    title: String,
    head: Option<Markup>,
    body: Option<Markup>,
}

impl Master {
    pub fn new<T: Into<String>>(title: T) -> Master {
        Master {
            title: title.into(),
            head: None,
            body: None
        }
    }

    pub fn with_head(mut self, head: Markup) -> Master {
        self.head = Some(head);
        self
    }

    pub fn with_body(mut self, body: Markup) -> Master {
        self.body = Some(body);
        self
    }
}

impl RenderOnce for Master {
    fn render_once(self) -> Markup {
        html! {
            html {
                head {
                    title (self.title)
                    (Css("/s/css/main.css"))
                    @if let Some(m) = self.head { (m) }
                }
                body {
                    div.body {
                        @if let Some(m) = self.body { (m) }
                    }
                }
            }
        }
    }
}