use std::fmt::Display;

/// Section in dialog
#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub enum Section {
    /// Initial section. Dialog starts from this
    Initial,
    /// Named section in dialog
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
