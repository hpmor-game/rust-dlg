mod section;
mod tokens;

use crate::{
    parser::tokens::{SemanticToken, Token},
    player::Cursor,
};
use logos::Lexer;
use std::{collections::HashMap, str::FromStr};
use tokens::MentionToken;

pub use section::Section;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Alias(pub String);

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum State {
    Default,
    Named(String),
}

#[derive(PartialEq, Debug, Default)]
pub struct Requirements {
    pub states: Vec<State>,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Mention {
    Narrator,
    Character(Alias, State),
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Menu {
    pub title: Option<String>,
    pub options: Vec<MenuOption>,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct MenuOption {
    pub title: Option<String>,
    pub args: String,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Line {
    Phrase {
        speaker: Mention,
        lines: Vec<String>,
    }, // TODO: replace String to FormattedText,
    Menu(Menu),
}

#[derive(PartialEq, Debug, Default)]
pub struct Dialog {
    pub characters: HashMap<Alias, Requirements>,
    pub sections: HashMap<Section, Vec<Line>>,
}

impl Dialog {
    #[must_use]
    pub fn get_line_by_cursor(&self, cursor: &Cursor) -> Option<&Line> {
        self.sections
            .get(cursor.section())
            .and_then(|s| s.get(cursor.line_index()))
    }

    fn ensure_character_requirement(&mut self, mention: &Mention) {
        if let Mention::Character(alias, state) = mention {
            let req = self.characters.entry(alias.clone()).or_default();

            if let State::Named(_) = state {
                if !req.states.contains(state) {
                    req.states.push(state.clone());
                }
            }
        }
    }

    fn parse_semantics(&mut self, semantics: Vec<SemanticToken>) -> Result<(), String> {
        let mut current_mention = Mention::Narrator;
        let mut current_section = Section::Initial;

        let mut current_menu: Option<Menu> = None;
        let mut current_option: Option<MenuOption> = None;
        for token in semantics {
            let current_lines = self.sections.entry(current_section.clone()).or_default();

            match token {
                SemanticToken::Mention(variant) => {
                    match &variant {
                        MentionToken::Name(name) => {
                            self.characters.entry(Alias(name.clone())).or_default();

                            current_mention =
                                Mention::Character(Alias(name.clone()), State::Default);

                            self.ensure_character_requirement(&current_mention);
                        }
                        MentionToken::State(new_state) => {
                            if let Mention::Character(_, state) = &mut current_mention {
                                // Это очень грустно, уберите клонирование пажалуста
                                *state = State::Named(new_state.clone());

                                self.ensure_character_requirement(&current_mention);
                            } else {
                                return Err("Нельзя устанавливать состояние рассказчику".to_owned());
                            }
                        }
                        MentionToken::NameState(name, state) => {
                            current_mention = Mention::Character(
                                Alias(name.clone()),
                                State::Named(state.clone()),
                            );
                            self.ensure_character_requirement(&current_mention);
                        }
                        MentionToken::Narrator => current_mention = Mention::Narrator,
                    };
                }
                SemanticToken::Link(name) => {
                    if let Some(mut menu) = current_menu {
                        if let Some(option) = current_option {
                            menu.options.push(option);
                        }

                        let lines = self.sections.entry(current_section.clone()).or_default();
                        lines.push(Line::Menu(menu));
                    }

                    current_menu = None;
                    current_option = None;

                    current_section = Section::Named(name);
                    current_mention = Mention::Narrator;
                }

                SemanticToken::Text(lines) => {
                    // TODO: if lines length > 1 and current_menu is not None - stop filling menu
                    if let Some(menu) = &mut current_menu {
                        if lines.is_empty() {
                            panic!("0 lines in text token")
                        } else {
                            let title = lines[0].clone();
                            let left = &lines[1..];

                            if menu.title.is_none() {
                                menu.title = Some(title);
                            // FIXME: .unwrap() & .clone()
                            } else if let Some(option) = &mut current_option {
                                option.title = Some(title);
                            } else {
                                panic!("text after `menu` tag is not allowed cause menu is final statement of section");
                            }

                            if !left.is_empty() {
                                if let Some(option) = &current_option {
                                    menu.options.push(option.clone());
                                    current_lines.push(Line::Menu(menu.clone()));
                                    current_menu = None;
                                }
                            }
                        }
                    } else {
                        current_lines.push(Line::Phrase {
                            speaker: current_mention.clone(),
                            lines,
                        });
                    }
                }
                SemanticToken::Command(command, args) => match command.as_str() {
                    "menu" => {
                        current_menu = Some(Menu {
                            title: None,
                            options: vec![],
                        });
                    }
                    "opt" => {
                        if let Some(menu) = &mut current_menu {
                            if let Some(option) = &current_option {
                                menu.options.push(option.clone());
                            }
                            current_option = Some(MenuOption { title: None, args });
                        } else {
                            panic!("`opt` without the preceding `menu`");
                        }
                    }
                    _ => {}
                },
                SemanticToken::InlineBlock(v) => {
                    panic!("inline block `{}` is not yet implemented :(", v);
                }
            }
        }

        if let Some(mut menu) = current_menu {
            if let Some(option) = current_option {
                menu.options.push(option);
            }

            let lines = self.sections.entry(current_section).or_default();
            lines.push(Line::Menu(menu));
        }

        Ok(())
    }
}

impl FromStr for Dialog {
    type Err = String;
    fn from_str(raw: &str) -> Result<Dialog, Self::Err> {
        let mut lex = Lexer::<Token>::new(raw);

        let mut buf = String::new();
        let mut semantics = vec![];
        while let Some(token) = lex.next() {
            let value = lex.slice().trim();

            if let Token::Text = token {
                buf.push_str(lex.slice());
            } else if !buf.is_empty() {
                let lines = buf
                    .lines()
                    .collect::<Vec<_>>()
                    .chunks(2)
                    .filter_map(|e| {
                        let str = e.join("\n").trim().to_owned();
                        if str.is_empty() {
                            None
                        } else {
                            Some(str)
                        }
                    })
                    .collect::<Vec<_>>();

                if !lines.is_empty() {
                    semantics.push(SemanticToken::Text(lines));
                }
                buf.clear();
            }

            let semantic_token = match token {
                Token::Mention => {
                    Some(if value.len() == 1 {
                        SemanticToken::Mention(MentionToken::Narrator)
                    } else {
                        let splitted = value[1..].split(':').collect::<Vec<_>>();
                        match splitted.len() {
                            1 => SemanticToken::Mention(MentionToken::Name(splitted[0].into())), // no state?
                            2 => {
                                if splitted[0].trim().is_empty() {
                                    SemanticToken::Mention(MentionToken::State(splitted[1].into()))
                                } else {
                                    SemanticToken::Mention(MentionToken::NameState(
                                        splitted[0].into(),
                                        splitted[1].into(),
                                    ))
                                }
                            }
                            _ => panic!("Ты чево наделол"),
                        }
                    })
                }
                Token::InlineBlock => Some(SemanticToken::InlineBlock(String::from(value))),
                Token::Link => Some(SemanticToken::Link(String::from(&value[1..]))),
                Token::Command => {
                    if let Some(index) = value.find('(') {
                        let name = &value[1..index];
                        let args = &value[index + 1..value.len() - 1];

                        Some(SemanticToken::Command(name.to_string(), args.to_string()))
                    } else {
                        Some(SemanticToken::Command(value[1..].into(), String::new()))
                    }
                }
                _ => None,
            };

            if let Some(token) = semantic_token {
                semantics.push(token);
            }
        }

        if !buf.is_empty() {
            let lines = buf
                .lines()
                .collect::<Vec<_>>()
                .chunks(2)
                .filter_map(|e| {
                    let str = e.join("\n").trim().to_owned();
                    if str.is_empty() {
                        None
                    } else {
                        Some(str)
                    }
                })
                .collect::<Vec<_>>();

            if !lines.is_empty() {
                semantics.push(SemanticToken::Text(lines));
            }
            buf.clear();
        }

        let mut dlg: Dialog = Dialog::default();

        dlg.parse_semantics(semantics)?;

        Ok(dlg)
    }
}
