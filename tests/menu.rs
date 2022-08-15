use std::str::FromStr;

use common_macros::hash_map;
use dlg::{
    character_requirements,
    parser::{Menu, MenuOption},
    prelude::*,
};

#[test]
fn test_basic_menu() {
    let raw = r"
            :menu Title
            :opt(#section_1) Section 1
            :opt(#section_2) Section 2


            #section_1

            Text in section 1


            #section_2

            Text in section 2
        ";

    let actual = Dialog::from_str(&raw).unwrap();

    let expected = Dialog {
        characters: character_requirements! {},
        sections: hash_map! {
            Section::Initial => vec![
                Line::Menu(Menu {
                    title: Some("Title".to_string()),
                    options: vec![
                       MenuOption {
                           title: Some("Section 1".to_string()),
                           args: "#section_1".to_string()
                       },
                       MenuOption {
                           title: Some("Section 2".to_string()),
                           args: "#section_2".to_string()
                       }
                    ]
                }),
            ],
            Section::Named("section_1".to_string()) => vec![
                Line::Phrase {
                    speaker: Mention::Narrator,
                    lines: vec![
                        "Text in section 1".to_string()
                    ]
                },
            ],
            Section::Named("section_2".to_string()) => vec![
                Line::Phrase {
                    speaker: Mention::Narrator,
                    lines: vec![
                        "Text in section 2".to_string()
                    ]
                },
            ]
        },
    };

    assert_eq!(expected, actual);
}
