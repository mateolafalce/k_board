<div align="center">

# k_board

[<img alt="crates.io" src="https://img.shields.io/crates/v/k_board.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/k_board)
[<img alt="github" src="https://img.shields.io/badge/github-mateolafalce/k__board-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/mateolafalce/k_board)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-k__board-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/k_board)

</div>


A lightweight keyboard mannager developed for dynamic programs by listening to keyboard events in raw mode (without the need to press enter). The handler has all the standard events of a western keyboard.

- Gnu/Linux 

```rust
pub enum Keys {
    Up,
    Down,
    Left,
    Right,
    Enter,
    Space,
    Delete,
    Escape,
    Plus,
    Minus,
    Equal,
    Power,                 ^
    Slash,                 /
    Backslash,             \
    Asterisk,              *
    Point,
    Comma,
    Hashtag,
    Pipe,                  |
    Percent,               %
    Ampersand,
    Currency,              $
    TwoPoints,             :
    Semicolon,             ;
    OpenSquareBracket,     [
    CloseSquareBracket,    ]
    OpenCurlyBrace,        {
    CloseCurlyBrace,       }
    OpenQuestionMark,      ¿
    CloseQuestionMark,     ?
    OpenParenthesis,       (
    CloseParenthesis,      )
    LessThan,              <
    GreaterThan,           >
    Apostrophe,            ' 
    At,                    @
    Home,
    End,
    Tab,
    Backtab,
    Insert,
    Letter(char),
    Number(u8),
    F(u8),
    Ctrl(char),
    Alt(char),
    Null,
}
```

## Example

```rust
use k_board::{Keyboard, Keys};

fn main() {
    menu(0);
    for key in Keyboard::new() {
        match key {
            Keys::Up => menu(0),
            Keys::Down => menu(1),
            Keys::Enter => break,
            _ => {}
        }
    }
}

fn menu(operation: u8) {
    std::process::Command::new("clear").status().unwrap();
    let mut op: Vec<char> = vec!['*', ' '];
    if operation == 1 {
        op[0] = ' ';
        op[1] = '*';
    }
    println!(
        "[{}] I use k_board lightweight software\n[{}] I use heavyweight software",
        op[0], op[1]
    );
}
```

## Contributing 

Feel free to contribute to the repository. Make each modification to the code formatted with code before with `cargo fmt`. Below, a fragment that allows you to visualize in hexadecimal the key or the event executed on your keyboard:

```rust
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    println!("Press a key or an keyboard event!");
    loop {
        let _ = get_key();
    }
}

pub fn get_key() -> std::io::Result<()> {
    let termios_enviroment: k_board::termios = k_board::setup_raw_mode().unwrap();
    std::io::stdout().flush().unwrap();
    let mut buffer: [u8; 3] = [0; 3];
    std::io::stdin().read(&mut buffer).unwrap();
    if buffer[0] != 0x00 {
        println!(
            "[0x{:x?}, 0x{:x?}, 0x{:x?}]",
            buffer[0], buffer[1], buffer[2]
        );
    }
    std::io::stdout().flush().unwrap();
    k_board::restore_termios(&termios_enviroment).unwrap();
    Ok(())
}
```
