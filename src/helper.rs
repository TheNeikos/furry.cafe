
pub trait StringHelpers {
    fn possessive(&self) -> String;
}

impl<S: AsRef<str>> StringHelpers for S {
    fn possessive(&self) -> String {
        let s = self.as_ref();
        format!("{}'{}", s, match s.chars().last() {
            Some(s) if s == 's' => "",
            _ => "s",
        })
    }
}
