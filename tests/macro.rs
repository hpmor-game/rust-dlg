use std::collections::HashMap;

use dlg::{parser::State, prelude::*};

#[macro_use]
extern crate dlg;

#[test]
fn test_character_requirements_macro() {
    let actual = character_requirements! {
        "alice" => ["calm", "happy"]
    };

    let mut expected = HashMap::with_capacity(1);
    expected.insert(
        Alias("alice".to_string()),
        Requirements {
            states: vec![
                State::Named("calm".to_string()),
                State::Named("happy".to_string()),
            ],
        },
    );

    assert_eq!(expected.capacity(), actual.capacity());
    assert_eq!(expected, actual);
}
