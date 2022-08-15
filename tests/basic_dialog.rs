use std::str::FromStr;

use common_macros::hash_map;
use dlg::{character_requirements, parser::State, prelude::*};

#[test]
fn test_basic_dialog() {
    let raw = r"
            @ Narrator's text

            @bob:state Bob's text
        ";

    let actual = Dialog::from_str(&raw).unwrap();

    let expected = Dialog {
        characters: hash_map! {
            Alias("bob".to_owned()) => Requirements {
                states: vec![State::Named("state".to_owned())]
            }
        },
        sections: hash_map! {
            Section::Initial => vec![
                Line::Phrase {
                    speaker: Mention::Narrator, lines: vec!["Narrator's text".to_owned()]
                },
                Line::Phrase {
                    speaker: Mention::Character(Alias("bob".to_owned()), State::Named("state".to_owned())),
                    lines: vec!["Bob's text".to_owned()],
                },
            ]
        },
    };

    assert_eq!(expected, actual);
}

#[test]
fn test_dialog_with_sections() {
    let raw = r"
            @ Narrator's text

            @bob:state Bob's text

            #section_with_items

            Narrator's text again

            #section_without_text
        ";

    let actual = Dialog::from_str(&raw).unwrap();

    let expected = Dialog {
        characters: character_requirements! {
            "bob" => ["state"]
        },
        sections: hash_map! {
            Section::Initial => vec![
                Line::Phrase {
                    speaker: Mention::Narrator,
                    lines: vec!["Narrator's text".to_owned()]
                },
                Line::Phrase {
                    speaker: Mention::Character(Alias("bob".to_owned()), State::Named("state".to_owned())),
                    lines: vec!["Bob's text".to_owned()],
                },
            ],
            Section::Named("section_with_items".to_owned()) => vec![
                Line::Phrase {
                    speaker: Mention::Narrator,
                    lines: vec!["Narrator's text again".to_owned()]
                },
            ]
        },
    };

    assert_eq!(expected, actual);
}

#[test]
fn test_dialog_with_dumb_text() {
    let raw = r"
            @bob:state шгагsgiu32:232r;asdlf@423rkjl;:dsxwasdlk:
        ";

    let actual = Dialog::from_str(&raw).unwrap();

    let expected = Dialog {
        characters: hash_map! {
            Alias("bob".to_owned()) => Requirements {
                states: vec![State::Named("state".to_owned())]
            }
        },
        sections: hash_map! {
            Section::Initial => vec![
                Line::Phrase {
                    speaker: Mention::Character(Alias("bob".to_owned()), State::Named("state".to_owned())),
                    lines: vec!["шгагsgiu32:232r;asdlf@423rkjl;:dsxwasdlk:".to_owned()],
                },
            ],
        },
    };

    assert_eq!(expected, actual);
}
