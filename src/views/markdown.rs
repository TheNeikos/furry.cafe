use maud::{PreEscaped, RenderOnce};
use maud_pulldown_cmark::Markdown;
use pulldown_cmark::{Parser, Event};

pub fn parse(s: &str) -> PreEscaped<String> {
    let events = Parser::new(s).map(|ev| match ev {
        Event::Html(html) | Event::InlineHtml(html) => Event::Text(html),
        _ => ev
    });

    let mut string = String::new();
    Markdown::from_events(events).render_once(&mut string).unwrap();
    PreEscaped(string)
}
