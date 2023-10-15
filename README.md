<div align="center">

# K_board

</div>

A lightweight keyboard mannager developed for dynamic programs by listening to keyboard events in raw mode (without the need to press enter). The handler has all the standard events of a Western keyboard.

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
    Power,
    Slash,
    Backslash,
    Asterisk,
    Point,
    Comma,
    Hashtag,
    Pipe,
    Percent,
    Ampersand,
    Currency,
    TwoPoints,
    Semicolon,
    OpenSquareBracket,
    CloseSquareBracket,
    OpenCurlyBrace,
    CloseCurlyBrace,
    OpenQuestionMark,
    CloseQuestionMark,
    OpenParenthesis,
    CloseParenthesis,
    LessThan,
    GreaterThan,
    Apostrophe,
    At,
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
