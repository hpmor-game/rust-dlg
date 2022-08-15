mod cursor;

use std::fmt::{write, Display};

pub use cursor::Cursor;

use crate::{parser::Menu, prelude::*};

pub trait Player {
    fn play(dialog: Dialog);
    fn process_line(state: &mut DialogState, line: &Line);
    fn end();
}

#[derive(Default, Debug)]
pub struct Animation {
    pub current: usize,
    pub target: usize,
    pub waited: usize,
}

impl Display for Animation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.current, self.target)
    }
}

#[derive(Default, Debug)]
pub struct DialogState {
    pub cursor: Cursor,
    pub animation: Animation,
}

pub struct ConsoleDialogPlayer;

impl ConsoleDialogPlayer {
    fn clear_terminal() {}

    fn wait_for_enter(message: &str) {
        use std::io::{self, BufRead};

        println!();
        println!("{}", message);

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.lock().read_line(&mut input).expect("Why?");
    }
}

impl Player for ConsoleDialogPlayer {
    fn play(dialog: Dialog) {
        let mut state = DialogState {
            cursor: Cursor::default(),
            animation: Animation::default(),
        };

        while let Some(line) = dialog.get_line_by_cursor(&state.cursor) {
            Self::process_line(&mut state, line);
        }

        Self::end();
    }

    fn process_line(state: &mut DialogState, line: &Line) {
        Self::clear_terminal();

        match line {
            Line::Phrase { speaker, lines } => {
                if let Some(line) = lines.get(state.cursor.phrase_index()) {
                    println!("{:?}: {:?}", speaker, line);
                    state.cursor.next_phrase_index();
                } else {
                    state.cursor.next_line_index();
                    return;
                }
            }
            Line::Menu(menu) => {
                let Menu { title, options } = menu;
                println!("{:?}", title);
                println!();
                for (index, opt) in options.iter().enumerate() {
                    println!("{}. {:?}", index + 1, opt.title);
                }
            }
        }

        Self::wait_for_enter("Нажмите [Enter] для продолжения");
    }

    fn end() {
        println!("End");
    }
}
