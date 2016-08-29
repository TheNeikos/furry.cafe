use std::fmt::{self, Display, Formatter};

#[derive(Copy, Clone)]
pub enum ButtonType {
    Primary, Secondary, Success, Info, Warning, Danger, Link
}

impl ButtonType {
    pub fn as_str(&self) -> &'static str {
        match *self {
            ButtonType::Primary => "btn-primary",
            ButtonType::Secondary => "btn-secondary",
            ButtonType::Success => "btn-success",
            ButtonType::Info => "btn-info",
            ButtonType::Warning => "btn-warning",
            ButtonType::Danger => "btn-danger",
            ButtonType::Link => "btn-link",
        }
    }
}

#[derive(Copy, Clone)]
pub enum RequestMethod {
    Get, Post
}

#[derive(Copy, Clone)]
pub struct Button<'a> {
    label: &'a str,
    path: &'a str,
    typ: ButtonType,
    req_meth: RequestMethod,
}

impl<'a> Button<'a> {
    pub fn new(label: &'a str, path: &'a str) -> Button<'a> {
        Button {
            label: label,
            path: path,
            typ: ButtonType::Secondary,
            req_meth: RequestMethod::Get,
        }
    }

    pub fn with_method(mut self, meth: RequestMethod) -> Button<'a> {
        self.req_meth = meth;
        self
    }
}

impl<'a> Display for Button<'a> {
    fn fmt(&self, mut f: &mut Formatter) -> Result<(), fmt::Error> {
        match self.req_meth {
            RequestMethod::Get => {
                html!(f,
                    a href=^(url!(self.path)) class=^(format!("btn {}", self.typ.as_str())) ^(self.label)
                )
            }
            RequestMethod::Post => {
                html!(f,
                    form method="POST" action=^(url!(self.path)) {
                        input type="submit"  class=^(format!("btn {}", self.typ.as_str())) value=^(self.label) /
                    }
                )
            }
        }
    }
}

