use std::str::FromStr;

use common_macros::hash_map;
use dlg::{character_requirements, parser::State, prelude::*};

#[test]
fn test_lines() {
    let raw = r"
            A
            B
            C
            D

            E
            F
        ";

    let actual = Dialog::from_str(&raw).unwrap();

    let expected = Dialog {
        characters: character_requirements! {},
        sections: hash_map! {
            Section::Initial => vec![
                Line::Phrase {
                    speaker: Mention::Narrator,
                    lines: vec![
                        "A".to_string(),
                        "B".to_string(),
                        "C".to_string(),
                        "D".to_string(),
                    ]
                },
                Line::Phrase {
                    speaker: Mention::Narrator,
                    lines: vec![
                        "E".to_string(),
                        "F".to_string(),
                    ]
                },
            ],
        },
    };

    assert_eq!(expected, actual);
}
