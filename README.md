<div align="center">

# k_board

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
