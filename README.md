# DLG

Нереальный парсер и проигрыватель диалогов формата .dlg

## Сборка и запуск

1. Качаем [Rust](https://www.rust-lang.org/)
2. Для запуска вызываем ```cargo run```
3. Для сборки вызываем ```cargo build```

## Example

```rust

static RAW_DIALOG: &'static str = r"
Alice came into the room and waved her hand

@alice Hi, Bob!

@bob Hi!

@alice What are you going to do?

:menu What should I say to her?
:opt(#walk) I'm going to go for a walk
:opt(#sleep) I want to sleep


#walk

@alice Great! May I come with you?

@bob Sure!

You went for a walk and had a good time


#sleep

@alice Oh, okay. Then I'll come back later. Sweet dreams

@bob Are made of this...
";

fn main() { 
    let dialog = Dialog::from_str(RAW_DIALOG).expect("can't parse dialog");
}
```