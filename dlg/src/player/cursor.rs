use std::fmt::Display;

use crate::prelude::*;

// TODO: perhaps it makes sense to bind the cursor to the dialog
#[derive(Debug)]
pub struct Cursor(Section, usize, usize);

impl Cursor {
    fn set_section_internal(&mut self, section: Section) {
        self.0 = section;
        self.1 = 0;
        self.2 = 0;
    }

    pub fn reset(&mut self) {
        self.set_section_internal(Section::Initial);
    }

    pub fn set_section(&mut self, name: String) {
        self.set_section_internal(Section::Named(name));
    }

    #[must_use]
    pub fn section(&self) -> &Section {
        &self.0
    }

    pub fn next_line_index(&mut self) {
        self.1 += 1;
        self.2 = 0;
    }

    #[must_use]
    pub fn line_index(&self) -> usize {
        self.1
    }

    pub fn next_phrase_index(&mut self) {
        self.2 += 1;
    }

    #[must_use]
    pub fn phrase_index(&self) -> usize {
        self.2
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Self(Section::Initial, 0, 0)
    }
}

impl Display for Cursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.0, self.1, self.2)
    }
}
