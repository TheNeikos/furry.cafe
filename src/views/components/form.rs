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
pub struct Form<'a, 'b> {
    method: FormMethod,
    path: &'a str,
    fields: &'b[&'b (Display + Sync)],
}

impl<'a, 'b> Form<'a, 'b> {
    pub fn new(method: FormMethod, path: &'a str) -> Form<'a, 'b> {
        Form {
            method: method,
            path: path,
            fields: EMPTY_FIELDS,
        }
    }

    pub fn with_fields(mut self, others: &'b[&'b (Display + Sync)]) -> Form<'a, 'b> {
        self.fields = others;
        self
    }
}

impl<'a, 'b> Display for Form<'a, 'b> {
    fn fmt(&self, mut f: &mut Formatter) -> Result<(), fmt::Error> {

        html!(f,
            form method=^(self.method) action=^(self.path) {
                @for field in self.fields {
                    ^(PreEscaped(field))
                }
            }
        )
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

        html!(f,
            div.form-group {
                @if self.label != "" {
                    label for=^(self.name) ^(self.label)
                }
                input type=^(self.type_) id=^(self.name) name=^(self.name) class=^(format!("form-control {}", self.class)) value=^(self.value) ""
                @if let Some(errors) = self.errors {
                    @for err in errors {
                        p.error.error-text ^err
                    }
                }
            }
        )
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

        html!(f,
            div.form-group {
                @if self.label != "" {
                    label for=^(self.name) ^(self.label)
                }
                textarea id=^(self.name) name=^(self.name) class=^(format!("form-control {}", self.class)) rows=15 {
                    ^self.value
                }
                @if let Some(errors) = self.errors {
                    @for err in errors {
                        p.error.error-text ^err
                    }
                }
            }
        )
    }
}


trait FormError {
    fn get_errors(&self) -> &Vec<&str>;
}
