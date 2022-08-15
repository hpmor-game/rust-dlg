use std::str::FromStr;

use common_macros::hash_map;
use dlg::{character_requirements, parser::State, prelude::*};

#[test]
fn test_all_dialog_features() {
    let raw = r"
            @ Narrator's text

            @bob Bob without state

            @:state_1 Bob with state_1

            @bob:state_2 Bob with state_2
        ";

    let actual = Dialog::from_str(raw).unwrap();

    let expected = Dialog {
        characters: character_requirements! {
            "bob" => ["state_1", "state_2"]
        },
        sections: hash_map! {
            Section::Initial => vec![
                Line::Phrase {
                    speaker: Mention::Narrator,
                    lines: vec!["Narrator's text".to_owned()]
                },
                Line::Phrase {
                    speaker: Mention::Character(Alias("bob".to_owned()), State::Default),
                    lines: vec!["Bob without state".to_owned()],
                },
                Line::Phrase {
                    speaker: Mention::Character(Alias("bob".to_owned()), State::Named("state_1".to_owned())),
                    lines: vec!["Bob with state_1".to_owned()],
                },
                Line::Phrase {
                    speaker: Mention::Character(Alias("bob".to_owned()), State::Named("state_2".to_owned())),
                    lines: vec!["Bob with state_2".to_owned()],
                },
            ]
        },
    };

    assert_eq!(expected, actual);
}
