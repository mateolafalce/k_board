<div align="center">

# k_board

[<img alt="crates.io" src="https://img.shields.io/crates/v/k_board.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/k_board)
[<img alt="github" src="https://img.shields.io/badge/github-mateolafalce/k__board-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/mateolafalce/k_board)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-k__board-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/k_board)

</div>


A lightweight keyboard mannager developed for dynamic programs by listening to keyboard events in raw mode (without the need to press enter). The handler has all the standard events of a western keyboard.

- Gnu/Linux && Unix like systems

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
    std::process::Command::new("clear").status().unwrap();
    println!("[*] I use k_board lightweight software");
    println!("[ ] I use heavyweight software");
    for key in Keyboard::new() {
        match key {
            Keys::Up => {
                std::process::Command::new("clear").status().unwrap();
                println!("[*] I use k_board lightweight software");
                println!("[ ] I use heavyweight software");
            }
            Keys::Down => {
                std::process::Command::new("clear").status().unwrap();
                println!("[ ] I use k_board lightweight software");
                println!("[*] I use heavyweight software");
            }
            Keys::Enter => {
                break;
            }
            _ => {}
        }
    }
}
```

## Contributing 

Feel free to contribute to the repository. Make each modification to the code formatted with code before with `cargo fmt`.
