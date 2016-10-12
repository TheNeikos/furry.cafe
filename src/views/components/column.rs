use maud::Markup;

use std::fmt::{self, Display, Formatter};

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

    pub fn new_with_size(size: usize, pull: usize, content: Markup) -> Column {
        Column {
            size: size,
            pull: pull,
            content: content,
        }
    }
}

impl Display for Column {
    fn fmt(&self, mut f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(&html!(
            div class=(format!("{} {}", {
                format!("col-md-{}", self.size)
            }, {
                if self.pull > 0 {
                    format!("offset-md-{}", self.pull)
                } else {
                    String::new()
                }
            })) (self.content)
        ).into_string())
    }
}

