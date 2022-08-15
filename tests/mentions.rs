use std::str::FromStr;

use common_macros::hash_map;
use dlg::{character_requirements, parser::State, prelude::*};

#[test]
fn test_mentions() {
    let raw = r"
            @
            narrator

            @ narrator

            @bob bob
            
            @:calm calm bob

            @ narrator
        ";

    let actual = Dialog::from_str(&raw).unwrap();

    let expected = Dialog {
        characters: character_requirements! {
            "bob" => ["calm"]
        },
        sections: hash_map! {
            Section::Initial => vec![
                Line::Phrase {
                    speaker: Mention::Narrator,
                    lines: vec![ "narrator".to_string() ]
                },
                Line::Phrase {
                    speaker: Mention::Narrator,
                    lines: vec![ "narrator".to_string() ]
                },
                Line::Phrase {
                    speaker: Mention::Character(Alias("bob".to_string()), State::Default),
                    lines: vec![ "bob".to_string() ]
                },
                Line::Phrase {
                    speaker: Mention::Character(Alias("bob".to_string()), State::Named("calm".to_string())),
                    lines: vec![ "calm bob".to_string() ]
                },
                Line::Phrase {
                    speaker: Mention::Narrator,
                    lines: vec![ "narrator".to_string() ]
                },
            ],
        },
    };

    assert_eq!(expected, actual);
}
