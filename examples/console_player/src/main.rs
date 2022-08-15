use crossterm::event::KeyModifiers;
use dlg::parser::Menu;
use dlg::prelude::*;
use dlg::{parser::State, player::DialogState};
use io::Error as IoError;
use unicode_segmentation::UnicodeSegmentation;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use std::{error::Error, io, str::FromStr};
use tui::widgets::{List, ListItem};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

#[derive(Debug)]
enum InputMode {
    NextPhrase,
    NextLine,
    Menu { selection: usize, menu: Menu },
}

#[derive(Debug)]
enum ViewMode {
    Animation(f64),
    Input(InputMode),
    End,
}

/// Holds the state of the application
#[derive(Debug)]
struct App {
    view_mode: ViewMode,
    dialog: Dialog,
    state: DialogState,
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let args: Vec<String> = std::env::args().collect();

    let path = args.get(1);

    let dialog = if let Some(file) = path {
        let file = PathBuf::from(file);
        if file.exists() {
            let raw = fs::read_to_string(file).expect("error in reading file");
            let raw = format!("\n{}\n", raw); // FIXME: improve parsing of mentions in start of file
            Dialog::from_str(&raw).expect("can't load dialog from file")
        } else {
            panic!("file is not exists");
        }
    } else {
        run_select_file_menu(&mut terminal).unwrap().unwrap()
    };

    let app = App {
        view_mode: ViewMode::Animation(0.0),
        dialog,
        state: DialogState::default(),
    };

    let res = run_dialog(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn find_all_dialog_files<P: AsRef<Path>>(path: P) -> Vec<PathBuf> {
    let mut files = vec![];

    let paths = fs::read_dir(path).unwrap();

    for res in paths {
        let entry = res.unwrap().path();

        if entry.is_dir() {
            files.append(&mut find_all_dialog_files(&entry));
        }

        if let Some(ext) = entry.extension() {
            if ext == "dlg" {
                files.push(entry);
            }
        }
    }

    files
}

fn run_select_file_menu<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<Option<Dialog>> {
    let mut selection = 0;
    let files = find_all_dialog_files("./");

    loop {
        terminal.draw(|f| {
            select_file_ui(f, &files, selection);
        })?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Esc = key.code {
                return Ok(None);
            } else if key.modifiers.contains(KeyModifiers::CONTROL) {
                if let KeyCode::Char('c') = key.code {
                    return Ok(None);
                }
            };

            if let KeyCode::Enter = key.code {
                if let Some(path) = files.get(selection) {
                    let raw = fs::read_to_string(path).expect("can't read the file");
                    let raw = format!("\n{}\n", raw); // TODO: fix mention in beginning of file

                    let dialog = Dialog::from_str(&raw);

                    match dialog {
                        Ok(dialog) => return Ok(Some(dialog)),
                        Err(_) => {
                            return Err(IoError::new(ErrorKind::Other, "сan't parse dialog!"))
                        }
                    }
                }
                return Err(IoError::new(ErrorKind::NotFound, "file not found!"));
            }

            let raw_selection = match key.code {
                KeyCode::Up => selection as isize - 1isize,
                KeyCode::Down => selection as isize + 1isize,
                _ => selection as isize,
            };

            let len = files.len();
            let rem = raw_selection % len as isize;
            selection = (rem + (len * usize::from(rem < 0)) as isize) as usize;
        }
    }
}

fn select_file_ui<B: Backend>(f: &mut Frame<B>, files: &[PathBuf], selection: usize) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Length(1), Constraint::Min(1)].as_ref())
        .split(f.size());

    let (msg, style) = (
        vec![
            Span::raw("Use "),
            Span::styled("Arrows", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to select dialog."),
        ],
        Style::default().add_modifier(Modifier::RAPID_BLINK),
    );

    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let mut items = files
        .iter()
        .map(|b| b.display())
        .map(|f| Span::raw(f.to_string().as_str()[2..].to_owned())) // chop leading `./` for beauty
        .collect::<Vec<_>>();

    if let Some(item) = items.get_mut(selection) {
        item.style = Style::default().fg(Color::Yellow);
    }

    let messages = List::new(items.into_iter().map(ListItem::new).collect::<Vec<_>>()).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Select dialog file"),
    );
    f.render_widget(messages, chunks[1]);
}

fn run_dialog<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(50);
    loop {
        terminal.draw(|f| app.view_mode = dialog_ui(f, &mut app))?;

        if let ViewMode::End = app.view_mode {
            return Ok(());
        }

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                let cursor = &mut app.state.cursor;

                if let KeyCode::Esc = key.code {
                    return Ok(());
                } else if key.modifiers.contains(KeyModifiers::CONTROL) {
                    if let KeyCode::Char('c') = key.code {
                        return Ok(());
                    }
                };

                match &mut app.view_mode {
                    ViewMode::Input(input) => {
                        match input {
                            InputMode::NextLine => {
                                if let KeyCode::Enter = key.code {
                                    cursor.next_line_index();
                                    app.view_mode = ViewMode::Animation(0.);
                                };
                            }
                            InputMode::NextPhrase => {
                                if let KeyCode::Enter = key.code {
                                    cursor.next_phrase_index();
                                    app.view_mode = ViewMode::Animation(0.);
                                };
                            }
                            InputMode::Menu { selection, menu } => {
                                if let KeyCode::Enter = key.code {
                                    let opt = menu.options.get(*selection);

                                    if let Some(opt) = opt {
                                        let section = &opt.args[1..]; // FIXME: remove hash from link
                                        let _ = &app.state.cursor.set_section(section.to_string());
                                        app.view_mode = ViewMode::Animation(0.);
                                    } else {
                                        panic!(
                                            "Out of bounds: index `{}`, len `{}`",
                                            *selection,
                                            menu.options.len()
                                        )
                                    }
                                    continue;
                                }

                                let raw_selection = match key.code {
                                    KeyCode::Up => *selection as isize - 1isize,
                                    KeyCode::Down => *selection as isize + 1isize,
                                    _ => *selection as isize,
                                };

                                let len = menu.options.len();
                                let rem = raw_selection % len as isize;
                                let new_selection =
                                    (rem + (len * usize::from(rem < 0)) as isize) as usize;

                                *input = InputMode::Menu {
                                    selection: new_selection,
                                    menu: menu.clone(),
                                };
                            }
                        }
                    }
                    ViewMode::End => return Ok(()),
                    ViewMode::Animation(progress) => {
                        if let KeyCode::Enter = key.code {
                            *progress = 1.;
                        };
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
            if let ViewMode::Animation(progress) = &mut app.view_mode {
                *progress += 0.05;
            }
        }
    }
}

fn dialog_ui<B: Backend>(f: &mut Frame<B>, app: &mut App) -> ViewMode {
    let cursor = &app.state.cursor;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    let mut animation_progress = 1.;
    let (msg, style) = match &app.view_mode {
        ViewMode::Input(input) => match input {
            InputMode::NextPhrase | InputMode::NextLine => (
                vec![
                    Span::raw("Press "),
                    Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to continue."),
                ],
                Style::default().add_modifier(Modifier::RAPID_BLINK),
            ),
            InputMode::Menu { .. } => (
                vec![
                    Span::raw("Используйте "),
                    Span::styled("Стрелки", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" для выбора варианта."),
                ],
                Style::default(),
            ),
        },
        ViewMode::Animation(progress) => {
            animation_progress = *progress;
            (
                vec![
                    Span::raw("Press "),
                    Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to skip animation."),
                ],
                Style::default(),
            )
        }
        ViewMode::End => return ViewMode::End,
    };

    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let input = Paragraph::new(format!("Cursor: {}", &app.state.cursor))
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("State"));
    f.render_widget(input, chunks[1]);

    // TODO: Provide line from outside
    let line = match app.dialog.get_line_by_cursor(cursor) {
        Some(line) => line,
        None => return ViewMode::End,
    };

    match line {
        Line::Phrase { speaker, lines } => {
            let name = match speaker {
                Speaker::Narrator => String::from("Narrator"),
                Speaker::Character(alias, state) => match state {
                    State::Default => alias.0.to_string(),
                    State::Named(state) => format!("{}: {}", alias.0, state),
                },
            };

            if let Some(line) = lines.get(cursor.phrase_index()) {
                let progress = f64::min(animation_progress, 1.0);
                let letters = line.graphemes(true).collect::<Vec<_>>();
                let len = (letters.len() as f64 * progress) as usize;
                let line_part = &letters[0..len];

                let messages = Paragraph::new(line_part.join(""))
                    .block(Block::default().borders(Borders::ALL).title(name));
                f.render_widget(messages, chunks[2]);

                if animation_progress < 1. {
                    ViewMode::Animation(animation_progress)
                } else if cursor.phrase_index() == lines.len() - 1 {
                    ViewMode::Input(InputMode::NextLine)
                } else {
                    ViewMode::Input(InputMode::NextPhrase)
                }
            } else {
                panic!("no line");
                // let messages = Paragraph::new("А где")
                //     .block(Block::default().borders(Borders::ALL).title(name));
                // f.render_widget(messages, chunks[2]);

                // Some(InputMode::NextLine)
            }
        }
        Line::Menu(menu) => {
            let current_selection = match &app.view_mode {
                ViewMode::Input(InputMode::Menu { selection, .. }) => *selection,
                _ => 0,
            };

            let mut items = menu
                .options
                .iter()
                .map(|o| Span::raw(o.title.clone().unwrap()))
                .collect::<Vec<_>>();

            if let Some(item) = items.get_mut(current_selection) {
                item.style = Style::default().fg(Color::Yellow);
            }

            let messages = List::new(items.into_iter().map(ListItem::new).collect::<Vec<_>>())
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(menu.title.clone().unwrap()),
                );
            f.render_widget(messages, chunks[2]);

            ViewMode::Input(InputMode::Menu {
                selection: current_selection,
                menu: menu.clone(),
            })
        }
    }
}
