use std::fmt::Display;

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub enum Section {
    Initial,
    Named(String),
}

impl Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Section::Initial => write!(f, "initial"),
            Section::Named(name) => write!(f, "#{}", name),
        }
    }
}
