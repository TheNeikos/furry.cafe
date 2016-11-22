use maud::{PreEscaped, RenderOnce};
use maud_pulldown_cmark::Markdown;
use pulldown_cmark::{Parser, Event, Tag};

pub fn parse(s: &str) -> PreEscaped<String> {
    let events = Parser::new(s).map(|ev| match ev {
        Event::Html(html) | Event::InlineHtml(html) => Event::Text(html),
        Event::Start(Tag::Link(ref d, ref t)) if d.starts_with("javascript:") => Event::Text(t.clone()),
        _ => ev
    });

    let mut string = String::new();
    Markdown::from_events(events).render_once_to(&mut string);
    PreEscaped(string)
}
