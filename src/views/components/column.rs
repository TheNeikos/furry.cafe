use maud::Markup;
use maud::Render;

pub struct Column {
    size: usize,
    pull: usize,
    content: Markup,
}

impl Column {
    pub fn new(content: Markup) -> Column {
        Column {
            size: 10,
            pull: 1,
            content: content,
        }
    }

    pub fn custom(size: usize, pull: usize, content: Markup) -> Column {
        Column {
            size: size,
            pull: pull,
            content: content,
        }
    }
}

impl Render for Column {
    fn render_to(&self, mut f: &mut String) {
        f.push_str(&html!(
            div class=(format!("col-xs-12 col-md-{} {}", {
                self.size
            }, {
                if self.pull > 0 {
                    format!("offset-md-{}", self.pull)
                } else {
                    String::new()
                }
            })) (self.content)
        ).into_string());
    }
}

