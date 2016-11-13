use maud::*;
use std::borrow::Cow;

/// Master page
pub struct Master<'a> {
    title: Cow<'a, str>,
    head: Option<Markup>,
    body: Option<Markup>,
}

impl <'a> Master<'a> {
    pub fn new(title: Cow<'a, str>) -> Master<'a> {
        Master {
            title: title,
            head: None,
            body: None
        }
    }

    pub fn with_head(mut self, head: Markup) -> Master<'a> {
        self.head = Some(head);
        self
    }

    pub fn with_body(mut self, body: Markup) -> Master<'a> {
        self.body = Some(body);
        self
    }
}

impl <'a> RenderOnce for Master<'a> {
    fn render_once(self) -> Markup {
        html_debug! {
            html {
                head {
                    title (self.title)
                    style ".body { width: 900px; margin: 0 auto; padding: 0 20px; }"
                    @if let Some(m) = self.head {
                        (m)
                    }
                }
                body {
                    div.body {
                        @if let Some(m) = self.body {
                            (m)
                        }
                    }
                }
            }
        }
    }
}