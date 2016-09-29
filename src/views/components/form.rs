use std::fmt::{self, Display, Formatter};

use maud::PreEscaped;

static EMPTY_FIELDS: &'static[&'static (Display + Sync)] = &[];

#[derive(Copy, Clone, Debug)]
pub enum FormMethod {
    Get, Post
}

impl Display for FormMethod {
    fn fmt(&self, mut f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}" , match self {
            &FormMethod::Get => "GET",
            &FormMethod::Post => "POST",
        })
    }
}

#[derive(Copy, Clone)]
pub struct Form<'a, 'b, 'c> {
    method: FormMethod,
    path: &'a str,
    fields: &'b[&'b (Display + Sync)],
    encoding: &'c str,
}

impl<'a, 'b, 'c> Form<'a, 'b, 'c> {
    pub fn new(method: FormMethod, path: &'a str) -> Form<'a, 'b, 'c> {
        Form {
            method: method,
            path: path,
            fields: EMPTY_FIELDS,
            encoding: ""
        }
    }

    pub fn with_fields(mut self, others: &'b[&'b (Display + Sync)]) -> Form<'a, 'b, 'c> {
        self.fields = others;
        self
    }

    pub fn with_encoding(mut self, others: &'c str) -> Form<'a, 'b, 'c> {
        self.encoding = others;
        self
    }
}

impl<'a, 'b, 'c> Display for Form<'a, 'b, 'c> {
    fn fmt(&self, mut f: &mut Formatter) -> Result<(), fmt::Error> {

        f.write_str(&html!(
            form method=(self.method) action=(self.path) enctype=(self.encoding) {
                @for field in self.fields {
                    (PreEscaped(field))
                }
            }
        ).into_string())
    }
}

#[derive(Copy, Clone)]
pub struct Input<'a, 'b, 'c, 'd, 'e, 'f> {
    label: &'a str,
    name: &'b str,
    errors: Option<&'c Vec<&'c str>>,
    type_: &'d str,
    class: &'e str,
    value: &'f str,
}

impl<'a, 'b, 'c, 'd, 'e, 'f> Input<'a, 'b, 'c, 'd, 'e, 'f> {
    pub fn new(label: &'a str, name: &'b str) -> Input<'a, 'b, 'c, 'd, 'e, 'f> {
        Input {
            label: label,
            name: name,
            errors: None,
            type_: "text",
            class: "",
            value: "",
        }
    }

    pub fn with_errors(mut self, errors: Option<&'c Vec<&'c str>>) -> Input<'a, 'b, 'c, 'd, 'e, 'f> {
        self.errors = errors;
        self
    }

    pub fn with_type(mut self, type_: &'d str) -> Input<'a, 'b, 'c, 'd, 'e, 'f> {
        self.type_ = type_;
        self
    }

    pub fn with_class(mut self, class: &'e str) -> Input<'a, 'b, 'c, 'd, 'e, 'f> {
        self.class = class;
        self
    }

    pub fn with_value(mut self, value: &'f str) -> Input<'a, 'b, 'c, 'd, 'e, 'f> {
        self.value = value;
        self
    }
}

impl<'a, 'b, 'c, 'd, 'e, 'f> Display for Input<'a, 'b, 'c, 'd, 'e, 'f> {
    fn fmt(&self, mut f: &mut Formatter) -> Result<(), fmt::Error> {

        f.write_str(&html!(
            div.form-group {
                @if self.label != "" {
                    label for=(self.name) (self.label)
                }
                input type=(self.type_) id=(self.name) name=(self.name) class=(format!("form-control {}", self.class)) value=(self.value) ""
                @if let Some(errors) = self.errors {
                    @for err in errors {
                        p.error.error-text (err)
                    }
                }
            }
        ).into_string())
    }
}

#[derive(Copy, Clone)]
pub struct Textarea<'a, 'b, 'c, 'e, 'f> {
    label: &'a str,
    name: &'b str,
    errors: Option<&'c Vec<&'c str>>,
    class: &'e str,
    value: &'f str,
}

impl<'a, 'b, 'c, 'e, 'f> Textarea<'a, 'b, 'c, 'e, 'f> {
    pub fn new(label: &'a str, name: &'b str) -> Textarea<'a, 'b, 'c, 'e, 'f> {
        Textarea {
            label: label,
            name: name,
            errors: None,
            class: "",
            value: "",
        }
    }

    pub fn with_errors(mut self, errors: Option<&'c Vec<&'c str>>) -> Textarea<'a, 'b, 'c, 'e, 'f> {
        self.errors = errors;
        self
    }

    pub fn with_class(mut self, class: &'e str) -> Textarea<'a, 'b, 'c, 'e, 'f> {
        self.class = class;
        self
    }

    pub fn with_value(mut self, value: &'f str) -> Textarea<'a, 'b, 'c, 'e, 'f> {
        self.value = value;
        self
    }
}

impl<'a, 'b, 'c, 'e, 'f> Display for Textarea<'a, 'b, 'c, 'e, 'f> {
    fn fmt(&self, mut f: &mut Formatter) -> Result<(), fmt::Error> {

        f.write_str(&html!(
            div.form-group {
                @if self.label != "" {
                    label for=(self.name) (self.label)
                }
                textarea id=(self.name) name=(self.name) class=(format!("form-control {}", self.class)) rows=(15) {
                    (self.value)
                }
                @if let Some(errors) = self.errors {
                    @for err in errors {
                        p.error.error-text (err)
                    }
                }
            }
        ).into_string())
    }
}

pub struct Select<'a> {
    label: &'a str,
    name: &'a str,
    options: Vec<(&'a str, &'a str)>,
    selected: &'a str,
}

impl<'a> Select<'a> {
    pub fn new(label: &'a str, name: &'a str) -> Select<'a> {
        Select {
            label: label,
            name: name,
            options: vec![],
            selected: ""
        }
    }

    pub fn add_option(&'a mut self, name: &'a str, val: &'a str) -> &mut Select {
        self.options.push((name, val));
        self
    }

    pub fn with_selected(&'a mut self, selected: &'a str) -> &mut Select {
        self.selected = selected;
        self
    }
}

impl<'a> Display for Select<'a> {
    fn fmt(&self, mut f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str(&html!(
            div.form-group {
                @if self.label != "" {
                    label for=(self.name) (self.label)
                }
                select.custom-select.form-control name=(self.name) {
                    @for opt in &self.options {
                        @if self.selected == opt.1 {
                            option selected="selected" value=(opt.1) (opt.0)
                        } @else {
                            option value=(opt.1) (opt.0)
                        }
                    }
                }
            }
        ).into_string())
    }
}

