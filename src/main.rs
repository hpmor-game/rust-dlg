use std::{
    collections::HashMap,
    io::{self, BufRead},
};

use common_macros::hash_map;
use crossterm::{
    terminal::{Clear, ClearType},
    ExecutableCommand,
};

type Mark<'a> = &'a str; // метка в диалоге
type Lines<'a> = Vec<Line<'a>>; // список элементов диалога
type State<'a> = &'a str; // состояние персонажа во время реплики
type CharAlias<'a> = &'a str; // алиас персонажа в диалоге

struct Character<'a> {
    name: &'a str,
}

struct Dialog<'a> {
    characters: HashMap<CharAlias<'a>, Character<'a>>,
    marks: HashMap<Mark<'a>, Lines<'a>>,
    lines: Lines<'a>,
}

impl<'a> Dialog<'a> {
    fn start(&self) -> DialogState<'a> {
        Default::default()
    }

    fn get_user(&self, alias: CharAlias) -> Option<&Character> {
        self.characters.get(alias)
    }
}

struct MenuOption<'a> {
    label: &'a str,
    mark: Option<Mark<'a>>,
}

impl<'a> MenuOption<'a> {
    fn new(label: &'a str) -> Self {
        Self { label, mark: None }
    }

    fn mark(mut self, mark: Mark<'a>) -> Self {
        self.mark = Some(mark);
        self
    }
}

enum Speaker<'a> {
    Narrator,
    Character(CharAlias<'a>, Option<State<'a>>),
}

enum Line<'a> {
    Phrase {
        speaker: Speaker<'a>,
        phrases: Vec<&'a str>,
    },
    Menu {
        label: &'a str,
        opts: Vec<MenuOption<'a>>,
    },
}

#[derive(Default, Debug)]
struct AddressInDialog<'a> {
    mark: Option<Mark<'a>>,
    phrase_index: usize,
    phrase_part_index: usize,
}

#[derive(Default)]
struct DialogState<'a> {
    address: AddressInDialog<'a>,
}

fn main() -> crossterm::Result<()> {
    let harry = self::Character { name: "Harry" };

    let hermione = self::Character { name: "Hermione" };

    use Line::*;
    use Speaker::*;
    let dialog = Dialog {
        characters: hash_map! {
            "alice" => hermione,
            "bob" => harry
        },
        lines: vec![
            Phrase {
                speaker: Narrator,
                phrases: vec!["Алиса, зевая, заходит в комнату Боба"],
            },
            Phrase {
                speaker: Character("alice", Some("yawning")),
                phrases: vec!["Привет, Боб!"], // TODO: Вставка имени персонажа (это никакой не Боб, это Гарри)
            },
            Phrase {
                speaker: Character("alice", Some("happy")),
                phrases: vec![
                    "Сегодня такой хороший день!\nС самого утра я чувствую воодушение и радость!",
                    "А как у тебя настроение?",
                ],
            },
            Menu {
                label: "Что ей ответить?",
                opts: vec![
                    MenuOption::new("Отличное, спасибо!").mark("happy_mood"), // TODO: Кидать ошибку, если такой метки нет
                    MenuOption::new("Не очень радостное...").mark("sad_mood"),
                ],
            },
        ],
        marks: hash_map! {
        "happy_mood" => vec![
            Phrase {
                speaker: Character("bob", Some("happy")),
                phrases: vec!["У меня сегодня нет никаких планов"],
            },
            Phrase {
                speaker: Character("bob", Some("smile")),
                phrases: vec!["Хочешь, пойдём погуляем?"],
            },
        ],
        "sad_mood" => vec![
            Phrase {
                speaker: Character("bob", Some("sad")),
                phrases: vec!["Я хочу побыть один..."],
            },
        ]
        },
    };

    play(dialog)
}

fn play(dialog: Dialog) -> crossterm::Result<()> {
    let mut state = dialog.start();

    loop {
        io::stdout().execute(Clear(ClearType::All))?;

        let lines = match state.address.mark {
            Some(ref mark) => dialog.marks.get(mark),
            None => Some(&dialog.lines),
        }
        .unwrap();

        let line = lines.get(state.address.phrase_index);

        if let Some(line) = line {
            show_line(&dialog, line, &mut state);
        } else {
            println!("Конец!");
            break;
        }
    }

    wait_for_enter("Нажмите [Enter] для завершения.");

    Ok(())
}

fn wait_for_enter(message: &str) {
    println!();
    println!("{}", message);

    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).expect("Why?");
}

fn show_line<'a>(dialog: &Dialog, line: &Line<'a>, state: &mut DialogState<'a>) {
    match line {
        Line::Phrase { speaker, phrases } => {
            // TODO: Сделать мутации более очевидными с помощью итератора реплик и фраз по стейту
            let current_phrase = match phrases.get(state.address.phrase_part_index) {
                Some(&phrase) => {
                    state.address.phrase_part_index += 1; // следующая фраза

                    phrase
                }
                None => {
                    state.address.phrase_part_index = 0; // следующая реплика
                    state.address.phrase_index += 1;

                    return;
                }
            };

            show_phrase(dialog, speaker, current_phrase);
            wait_for_enter("Нажмите [Enter] для продолжения.");
        }
        Line::Menu { label, opts } => show_menu(label, opts, state),
    }
}

fn show_phrase(dialog: &Dialog, speaker: &Speaker, phrase: &str) {
    if let Speaker::Character(alias, state) = *speaker {
        let state = match state {
            Some(s) => format!(" ({s})"),
            None => String::new(),
        };

        let user = dialog
            .get_user(alias)
            .unwrap_or_else(|| panic!("Character with alias `{}` not found", alias));

        println!("{}{}", user.name, state);
        println!();
    }
    println!("{phrase}");
}

fn show_menu<'a>(label: &str, opts: &[MenuOption<'a>], state: &mut DialogState<'a>) {
    println!("{label}");

    for (i, opt) in opts.iter().enumerate() {
        println!("{}. {}", i + 1, opt.label);
    }

    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();

    if let Ok(index) = input.trim().parse::<usize>() {
        if let Some(opt) = opts.get(index - 1) {
            let mark_name = opt.mark.unwrap();

            // TODO: Сделать крутые методы, которые автоматически сбрасывают меньший индекс при изменении большего
            // Пример:
            // Изменилась метка -> обнуляем реплику и фразу
            // Изменилась реплика -> обнуляем фразу, метку не трогаем
            state.address.mark = Some(mark_name);
            state.address.phrase_index = 0;
            state.address.phrase_part_index = 0;
        }
    }
}
