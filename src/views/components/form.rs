use maud::{Render};

#[derive(Copy, Clone, Debug)]
pub enum FormMethod {
    Get, Post
}

impl Render for FormMethod {
    fn render_to(&self, mut f: &mut String) {
        f.push_str(match self {
            &FormMethod::Get => "GET",
            &FormMethod::Post => "POST",
        })
    }
}

#[derive(Copy, Clone)]
pub struct Form<'a> {
    method: FormMethod,
    path: &'a str,
    fields: Option<&'a[&'a Render]>,
    encoding: &'a str,
}

impl<'a> Form<'a> {
    pub fn new(method: FormMethod, path: &'a str) -> Form<'a> {
        Form {
            method: method,
            path: path,
            fields: None,
            encoding: ""
        }
    }

    pub fn with_fields(mut self, others: &'a[&'a Render]) -> Form<'a> {
        self.fields = Some(others);
        self
    }

    pub fn with_encoding(mut self, others: &'a str) -> Form<'a> {
        self.encoding = others;
        self
    }
}

impl<'a> Render for Form<'a> {
    fn render_to(&self, mut f: &mut String) {

        f.push_str(&html!(
            form method=(self.method) action=(self.path) enctype=(self.encoding) {
                @if let Some(fields) = self.fields {
                    @for field in fields {
                        (field)
                    }
                }
            }
        ).into_string())
    }
}

#[derive(Copy, Clone)]
pub struct Input<'a> {
    label: &'a str,
    name: &'a str,
    errors: Option<&'a Vec<&'a str>>,
    type_: &'a str,
    class: &'a str,
    value: &'a str,
}

impl<'a> Input<'a> {
    pub fn new(label: &'a str, name: &'a str) -> Input<'a> {
        Input {
            label: label,
            name: name,
            errors: None,
            type_: "text",
            class: "",
            value: "",
        }
    }

    pub fn with_errors(mut self, errors: Option<&'a Vec<&'a str>>) -> Input<'a> {
        self.errors = errors;
        self
    }

    pub fn with_type(mut self, type_: &'a str) -> Input<'a> {
        self.type_ = type_;
        self
    }

    pub fn with_class(mut self, class: &'a str) -> Input<'a> {
        self.class = class;
        self
    }

    pub fn with_value(mut self, value: &'a str) -> Input<'a> {
        self.value = value;
        self
    }
}

impl<'a> Render for Input<'a> {
    fn render_to(&self, mut f: &mut String) {
        f.push_str(&html!(
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
pub struct Textarea<'a> {
    label: &'a str,
    name: &'a str,
    errors: Option<&'a Vec<&'a str>>,
    class: &'a str,
    value: &'a str,
}

impl<'a> Textarea<'a> {
    pub fn new(label: &'a str, name: &'a str) -> Textarea<'a> {
        Textarea {
            label: label,
            name: name,
            errors: None,
            class: "",
            value: "",
        }
    }

    pub fn with_errors(mut self, errors: Option<&'a Vec<&'a str>>) -> Textarea<'a> {
        self.errors = errors;
        self
    }

    pub fn with_class(mut self, class: &'a str) -> Textarea<'a> {
        self.class = class;
        self
    }

    pub fn with_value(mut self, value: &'a str) -> Textarea<'a> {
        self.value = value;
        self
    }
}

impl<'a> Render for Textarea<'a> {
    fn render_to(&self, mut f: &mut String) {
        f.push_str(&html!(
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

#[derive(Clone)]
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

    pub fn add_option(mut self, name: &'a str, val: &'a str) -> Select<'a> {
        self.options.push((name, val));
        self
    }

    pub fn with_selected(mut self, selected: &'a str) -> Select<'a> {
        self.selected = selected;
        self
    }
}

impl<'a> Render for Select<'a> {
    fn render_to(&self, mut f: &mut String) {
        f.push_str(&html!(
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

