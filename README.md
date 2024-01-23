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
    Char(char)
    F(u8),
    Ctrl(char),
    Alt(char),
    AltGr(char),
    Null,
}
```

---

## Examples

<details>
<summary>Arrows keys & enter for dynamic menus</summary>

Simply `cargo add k_board`. No features.

```rust
use k_board::{keyboard::Keyboard, keys::Keys};

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

</details>

<details>
<summary>Ctrl + key, for standar cli events</summary>

```toml
[dependencies]
k_board = { version = "1.2.4", features = ["ctrl_lower_letter", "ctrl_upper_letter", "lower_letter"] }
```

```rust
use k_board::{keyboard::Keyboard, keys::Keys};

fn main() {
    for key in Keyboard::new() {
        match key {
            Keys::Ctrl('c') => copy_terminal(),
            Keys::Ctrl('s') => paste_into_terminal(),
            // remember upper & lower case in Ctrl + key is the same hex code
            Keys::Ctrl('a') => do_this(),
            Keys::Ctrl('A') => do_this(),
            Keys::Char('q') => break,
            _ => (),
        }
    }
}

fn copy_terminal() {}
fn paste_into_terminal() {}
fn reduce_screen() {}
fn zoom_screen() {}
fn do_this() {}
```

</details>

<details>
<summary>Get the F1, F2, F3 ...</summary>

```toml
[dependencies]
k_board = { version = "1.2.4", features = ["f"] }
```

```rust
use k_board::{keyboard::Keyboard, keys::Keys};

fn main() {
    for key in Keyboard::new() {
        match key {
            Keys::F(5) => update_screen(),
            Keys::F(9) => full_screen(),
            Keys::Enter => break,
            _ => {}
        }
    }
}

fn update_screen() {}
fn full_screen() {}
```

</details>

<details>
<summary>What number you press?</summary>

```toml
[dependencies]
k_board = { version = "1.2.4", features = ["numbers"] }
```

```rust
use k_board::{keyboard::Keyboard, keys::Keys};

fn main() {
    for key in Keyboard::new() {
        match key {
            Keys::Char('0') => break,
            Keys::Char('1') => download(),
            Keys::Char('2') => see_file(),
            Keys::Char('3') => share(),
            _ => {}
        }
    }
}

fn download() {}
fn see_file() {}
fn share() {}
```

</details>

<details>
<summary>Calculator</summary>

```toml
[dependencies]
k_board = { version = "1.2.4", features = ["numbers"] }
```

```rust
use k_board::{keyboard::Keyboard, keys::Keys};
use std::io;

fn main() {
    let mut result: f64 = 0.0;
    let first: f64 = 10.0;
    let second: f64 = 5.69;
    let operation: i8 = get_operation();
    match operation {
        0 => result = first + second,
        1 => result = first - second,
        2 => result = first * second,
        3 => {
            if second != 0.0 {
                result = first / second
            }
        }
        _ => {}
    }
    println!("The result is: {}", result);
}


fn get_operation() -> i8 {
    let mut operation: i8 = 0;
    menu(&mut operation, 0);
    for key in Keyboard::new() {
        match key {
            Keys::Up => menu(&mut operation, -1),
            Keys::Down => menu(&mut operation, 1),
            Keys::Enter => break,
            _ => {}
        }
    }
    operation
}

fn menu(operation: &mut i8, selection: i8) {
    std::process::Command::new("clear").status().unwrap();
    if *operation > 0 || *operation < 3 {
        *operation += selection;
    }
    let mut op = vec![' ', ' ', ' ', ' '];
    for i in 0..4 {
        if i == *operation {
            op[i as usize] = '*';
        }
    }
    println!(
        "{} Add\n{} Subtract\n{} Multiply\n{} Divide",
        op[0], op[1], op[2], op[3]
    );
}
```

</details>

<details>
<summary>ESC, simbols, etc</summary>

```toml
[dependencies]
k_board = { version = "1.2.4", features = ["standar"] }
```

```rust
use k_board::{keyboard::Keyboard, keys::Keys};

fn main() {
    for key in Keyboard::new() {
        match key {
            Keys::Escape => break,
            Keys::Space => jump(),
            Keys::Char('$') => money(),
            Keys::Char('@') => email(),
            _ => {}
        }
    }
}

fn jump() {}
fn money() {}
fn email() {}
```

</details>

<details>
<summary>Ctrl +  & Ctrl - for screen size</summary>

```toml
[dependencies]
k_board = { version = "1.2.4", features = ["ctrl_standar"] }
```

```rust
use k_board::{keyboard::Keyboard, keys::Keys};

fn main() {
    for key in Keyboard::new() {
        match key {
            Keys::Enter => break,
            Keys::Ctrl('-') => less_zoom(),
            Keys::Ctrl('+') => zoom(),
            _ => {}
        }
    }
}

fn less_zoom() {}
fn zoom() {}
```

</details>

<details>
<summary>Alt + key</summary>

```toml
[dependencies]
k_board = { version = "1.2.4", features = ["alt_lower_letter", "alt_upper_letter"] }
```

```rust
use k_board::{keyboard::Keyboard, keys::Keys};

fn main() {
    for key in Keyboard::new() {
        match key {
            Keys::Enter => break,
            Keys::Alt('a') => shy(),
            Keys::Alt('A') => angry(),
            _ => {}
        }
    }
}

fn shy() {}
fn angry() {}
```

</details>

<details>
<summary>Get lower & upper letters</summary>

```toml
[dependencies]
k_board = { version = "1.2.4", features = ["lower_letter", "upper_letter"] }
```

```rust
use k_board::{keyboard::Keyboard, keys::Keys};

fn main() {
    for key in Keyboard::new() {
        match key {
            Keys::Enter => break,
            Keys::Char('b') => lower_case(),
            Keys::Char('B') => upper_case(),
            _ => {}
        }
    }
}

fn lower_case() {}
fn upper_case() {}
```

</details>

<details>
<summary>Get Alt Gr + key</summary>

```toml
[dependencies]
k_board = { version = "1.2.4", features = ["alt_gr_lower_letter", "alt_gr_upper_letter"] }
```

```rust
use k_board::{keyboard::Keyboard, keys::Keys};

fn main() {
    for key in Keyboard::new() {
        match key {
            Keys::Enter => break,
            Keys::AltGr('l') => f1(),
            Keys::AltGr('L') => f2(),
            _ => {}
        }
    }
}

fn f1() {}
fn f2() {}
```

</details>

<details>
<summary>Ctrl, Alt & Alr Gr + number</summary>

```toml
[dependencies]
k_board = { version = "1.2.4", features = ["ctrl_numbers", "alt_numbers", "alt_gr_numbers"] }
```

```rust
use k_board::{keyboard::Keyboard, keys::Keys};

fn main() {
    for key in Keyboard::new() {
        match key {
            Keys::Enter => break,
            Keys::Ctrl('0') => execute(),
            Keys::Alt('1') => read(),
            Keys::AltGr('2') => write(),
            _ => {}
        }
    }
}

fn execute() {}
fn read() {}
fn write() {}
```

</details>

---

## Contributing 

Feel free to contribute to the repository. Run the bash command below to make all ci/cd actions successful. Below, a fragment that allows you to visualize in hexadecimal the key or the event executed on your keyboard.

```bash
clear && 
cargo fmt &&
cargo clippy &&
cargo build --all-features
```

```rust
use k_board::termio::{restore_termios, setup_raw_mode, termios};
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    println!("Press a key or an keyboard event!");
    loop {
        let _ = get_key();
    }
}

pub fn get_key() -> std::io::Result<()> {
    let termios_enviroment: termios = setup_raw_mode()?;
    std::io::stdout().flush().unwrap();
    let mut buffer: [u8; 3] = [0; 3];
    #[allow(clippy::unused_io_amount)]
    std::io::stdin().read(&mut buffer)?;
    if buffer[0] != 0x00 {
        println!(
            "[0x{:x?}, 0x{:x?}, 0x{:x?}]",
            buffer[0], buffer[1], buffer[2]
        );
    }
    std::io::stdout().flush().unwrap();
    restore_termios(&termios_enviroment)?;
    Ok(())
}
```

---

## Why k_board?

k_board, is designed for low-level development (direct interaction with the OS), boasts high and efficient performance compared to other libraries dedicated to keyboard interaction. This is demonstrated by performance tests conducted and that you can also perform to verify the technical superiority of this crate.

This has allowed k_board to be **441.86%** and **1046.51%** lighter than keyboard handling libraries without sacrificing quality and adding in-depth control to the developer over which part of the keyboard to manage and which not to.

<details>
<summary>Space test</summary>

last versions of all crates to date.

k_board(1.2.4) vs termion(3.0.0) vs crossterm(0.27.0)

```bash
# for k_board

cargo new k_board_ &&
cd k_board_/ &&
cargo add k_board &&
cargo build && 
cd .. && 
du k_board_/ -h &&
rm -rf k_board_

# for termion

cargo new termion_ &&
cd termion_/ &&
cargo add termion &&
cargo build && 
cd .. && 
du termion_/ -h &&
rm -rf termion_

# for crossterm

cargo new crossterm_ &&
cd crossterm_/ &&
cargo add crossterm &&
cargo build && 
cd .. && 
du crossterm_/ -h &&
rm -rf crossterm_

```

Results: 

- k_board: 4,2 MB 
- termion: 18 MB
- crossterm: 43 MB

<div align="center">

### Space used by crate in a normal hello world! (MB)

![space](https://private-user-images.githubusercontent.com/98977436/293250921-5c5546f8-9cf9-4ebb-b97f-af5a0882fa4b.svg?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTEiLCJleHAiOjE3MDM3ODU0NDgsIm5iZiI6MTcwMzc4NTE0OCwicGF0aCI6Ii85ODk3NzQzNi8yOTMyNTA5MjEtNWM1NTQ2ZjgtOWNmOS00ZWJiLWI5N2YtYWY1YTA4ODJmYTRiLnN2Zz9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFJV05KWUFYNENTVkVINTNBJTJGMjAyMzEyMjglMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjMxMjI4VDE3MzkwOFomWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPWQzYzRlNDZjYTU1ZjNiYmQ4N2MzM2Q4YmVhZWU4Yzc0M2U5OWZmNzVkOTlkOThlMmViY2Y5NGMyZGM2OTA4ZGQmWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0JmFjdG9yX2lkPTAma2V5X2lkPTAmcmVwb19pZD0wIn0.IJ5sr2A5GLr_FApmtlQKaaRPoTTEzd7gxdU8zLDoqVU)

</div>

</details>

---

## Features

The library has different features depending on the developer's needs.

<details>
<summary>no-feature(default)</summary>

```rust
pub const ARROWS_ENTER: [([u8; BYTES], Keys); 5] = [
    ([0x1B, 0x5B, 0x41], Keys::Up),
    ([0x1B, 0x5B, 0x42], Keys::Down),
    ([0x1B, 0x5B, 0x43], Keys::Right),
    ([0x1B, 0x5B, 0x44], Keys::Left),
    ([0x0A, 0x00, 0x00], Keys::Enter),
];
```

</details>

<details>
<summary>standar</summary>

```rust
pub const STANDAR: [([u8; BYTES], Keys); 40] = [
    ([0x1B, 0x5B, 0x48], Keys::Home),
    ([0x09, 0x00, 0x00], Keys::Tab),
    ([0x1B, 0x5B, 0x46], Keys::End),
    ([0x1B, 0x5B, 0x5a], Keys::Backtab),
    ([0x1b, 0x00, 0x00], Keys::Escape),
    ([0x20, 0x00, 0x00], Keys::Space),
    ([0x7F, 0x00, 0x00], Keys::Delete),
    ([0x2b, 0x00, 0x00], Keys::Char('+')),
    ([0x2d, 0x00, 0x00], Keys::Char('-')),
    ([0x3d, 0x00, 0x00], Keys::Char('=')),
    ([0x2f, 0x00, 0x00], Keys::Char('/')),
    ([0x5c, 0x00, 0x00], Keys::Char('\\')),
    ([0x5e, 0x00, 0x00], Keys::Char('^')),
    ([0x2a, 0x00, 0x00], Keys::Char('*')),
    ([0x2e, 0x00, 0x00], Keys::Char('.')),
    ([0x2c, 0x00, 0x00], Keys::Char(',')),
    ([0x23, 0x00, 0x00], Keys::Char('#')),
    ([0x26, 0x00, 0x00], Keys::Char('&')),
    ([0x25, 0x00, 0x00], Keys::Char('%')),
    ([0x7c, 0x00, 0x00], Keys::Char('|')),
    ([0x24, 0x00, 0x00], Keys::Char('$')),
    ([0x3a, 0x00, 0x00], Keys::Char(':')),
    ([0x3b, 0x00, 0x00], Keys::Char(';')),
    ([0xc2, 0xbf, 0x00], Keys::Char('¿')),
    ([0x3f, 0x00, 0x00], Keys::Char('?')),
    ([0x5b, 0x00, 0x00], Keys::Char('[')),
    ([0x5d, 0x00, 0x00], Keys::Char(']')),
    ([0x7b, 0x00, 0x00], Keys::Char('{')),
    ([0x7d, 0x00, 0x00], Keys::Char('}')),
    ([0x28, 0x00, 0x00], Keys::Char('(')),
    ([0x29, 0x00, 0x00], Keys::Char(')')),
    ([0x3c, 0x00, 0x00], Keys::Char('<')),
    ([0x3e, 0x00, 0x00], Keys::Char('>')),
    ([0x27, 0x00, 0x00], Keys::Char('\'')),
    ([0x40, 0x00, 0x00], Keys::Char('@')),
    ([0xc2, 0xa1, 0x0], Keys::Char('¡')),
    ([0x21, 0x00, 0x00], Keys::Char('!')),
    ([0x22, 0x0, 0x0], Keys::Char('"')),
    ([0x60, 0x0, 0x0], Keys::Char('`')),
    ([0xc2, 0xb4, 0x00], Keys::Char('´')),
];
```

</details>

<details>
<summary>numbers</summary>

```rust
pub const NUMBERS: [([u8; BYTES], Keys); 10] = [
    ([0x30, 0x00, 0x00], Keys::Char('0')),
    ([0x31, 0x00, 0x00], Keys::Char('1')),
    ([0x32, 0x00, 0x00], Keys::Char('2')),
    ([0x33, 0x00, 0x00], Keys::Char('3')),
    ([0x34, 0x00, 0x00], Keys::Char('4')),
    ([0x35, 0x00, 0x00], Keys::Char('5')),
    ([0x36, 0x00, 0x00], Keys::Char('6')),
    ([0x37, 0x00, 0x00], Keys::Char('7')),
    ([0x38, 0x00, 0x00], Keys::Char('8')),
    ([0x39, 0x00, 0x00], Keys::Char('9')),
];
```
</details>

<details>
<summary>lower_letter</summary>

```rust
pub const LOWER_LETTERS: [([u8; BYTES], Keys); 27] = [
    ([0x61, 0x00, 0x00], Keys::Char('a')),
    ([0x62, 0x00, 0x00], Keys::Char('b')),
    ([0x63, 0x00, 0x00], Keys::Char('c')),
    ([0x64, 0x00, 0x00], Keys::Char('d')),
    ([0x65, 0x00, 0x00], Keys::Char('e')),
    ([0x66, 0x00, 0x00], Keys::Char('f')),
    ([0x67, 0x00, 0x00], Keys::Char('g')),
    ([0x68, 0x00, 0x00], Keys::Char('h')),
    ([0x69, 0x00, 0x00], Keys::Char('i')),
    ([0x6A, 0x00, 0x00], Keys::Char('j')),
    ([0x6B, 0x00, 0x00], Keys::Char('k')),
    ([0x6C, 0x00, 0x00], Keys::Char('l')),
    ([0x6D, 0x00, 0x00], Keys::Char('m')),
    ([0x6E, 0x00, 0x00], Keys::Char('n')),
    ([0xb1, 0xb1, 0x00], Keys::Char('ñ')),
    ([0x6F, 0x00, 0x00], Keys::Char('o')),
    ([0x70, 0x00, 0x00], Keys::Char('p')),
    ([0x71, 0x00, 0x00], Keys::Char('q')),
    ([0x72, 0x00, 0x00], Keys::Char('r')),
    ([0x73, 0x00, 0x00], Keys::Char('s')),
    ([0x74, 0x00, 0x00], Keys::Char('t')),
    ([0x75, 0x00, 0x00], Keys::Char('u')),
    ([0x76, 0x00, 0x00], Keys::Char('v')),
    ([0x77, 0x00, 0x00], Keys::Char('w')),
    ([0x78, 0x00, 0x00], Keys::Char('x')),
    ([0x79, 0x00, 0x00], Keys::Char('y')),
    ([0x7A, 0x00, 0x00], Keys::Char('z')),
];
```
</details>

<details>
<summary>upper_letter</summary>

```rust
pub const UPPER_LETTER: [([u8; BYTES], Keys); 27] = [
    ([0x41, 0x00, 0x00], Keys::Char('A')),
    ([0x42, 0x00, 0x00], Keys::Char('B')),
    ([0x43, 0x00, 0x00], Keys::Char('C')),
    ([0x44, 0x00, 0x00], Keys::Char('D')),
    ([0x45, 0x00, 0x00], Keys::Char('E')),
    ([0x46, 0x00, 0x00], Keys::Char('F')),
    ([0x47, 0x00, 0x00], Keys::Char('G')),
    ([0x48, 0x00, 0x00], Keys::Char('H')),
    ([0x49, 0x00, 0x00], Keys::Char('I')),
    ([0x4A, 0x00, 0x00], Keys::Char('J')),
    ([0x4B, 0x00, 0x00], Keys::Char('K')),
    ([0x4C, 0x00, 0x00], Keys::Char('L')),
    ([0x4D, 0x00, 0x00], Keys::Char('M')),
    ([0x4E, 0x00, 0x00], Keys::Char('N')),
    ([0xb1, 0x91, 0x00], Keys::Char('Ñ')),
    ([0x4F, 0x00, 0x00], Keys::Char('O')),
    ([0x50, 0x00, 0x00], Keys::Char('P')),
    ([0x51, 0x00, 0x00], Keys::Char('Q')),
    ([0x52, 0x00, 0x00], Keys::Char('R')),
    ([0x53, 0x00, 0x00], Keys::Char('S')),
    ([0x54, 0x00, 0x00], Keys::Char('T')),
    ([0x55, 0x00, 0x00], Keys::Char('U')),
    ([0x56, 0x00, 0x00], Keys::Char('V')),
    ([0x57, 0x00, 0x00], Keys::Char('W')),
    ([0x58, 0x00, 0x00], Keys::Char('X')),
    ([0x59, 0x00, 0x00], Keys::Char('Y')),
    ([0x5A, 0x00, 0x00], Keys::Char('Z')),
];
```
</details>

<details>
<summary>f</summary>

```rust
pub const F: [([u8; BYTES], Keys); 12] = [
    ([0x1b, 0x4f, 0x50], Keys::F(1)),
    ([0x1b, 0x4f, 0x51], Keys::F(2)),
    ([0x1b, 0x4f, 0x52], Keys::F(3)),
    ([0x1b, 0x4f, 0x53], Keys::F(4)),
    ([0x35, 0x7E, 0x00], Keys::F(5)),
    ([0x37, 0x7E, 0x00], Keys::F(6)),
    ([0x38, 0x7E, 0x00], Keys::F(7)),
    ([0x39, 0x7E, 0x00], Keys::F(8)),
    ([0x30, 0x7E, 0x00], Keys::F(9)),
    ([0x31, 0x7E, 0x00], Keys::F(10)),
    ([0x33, 0x7E, 0x00], Keys::F(11)),
    ([0x34, 0x7E, 0x00], Keys::F(12)),
];
```

</details>

<details>
<summary>ctrl_lower_letter</summary>

```rust
pub const CTRL_LOWER_LETTER: [([u8; BYTES], Keys); 24] = [
    ([0x01, 0x00, 0x00], Keys::Ctrl('a')),
    ([0x02, 0x00, 0x00], Keys::Ctrl('b')),
    ([0x03, 0x00, 0x00], Keys::Ctrl('c')),
    ([0x04, 0x00, 0x00], Keys::Ctrl('d')),
    ([0x05, 0x00, 0x00], Keys::Ctrl('e')),
    ([0x06, 0x00, 0x00], Keys::Ctrl('f')),
    ([0x07, 0x00, 0x00], Keys::Ctrl('g')),
    ([0x08, 0x00, 0x00], Keys::Ctrl('h')),
    ([0x0B, 0x00, 0x00], Keys::Ctrl('k')),
    ([0x0C, 0x00, 0x00], Keys::Ctrl('l')),
    ([0x0D, 0x00, 0x00], Keys::Ctrl('m')),
    ([0x0E, 0x00, 0x00], Keys::Ctrl('n')),
    ([0x0F, 0x00, 0x00], Keys::Ctrl('o')),
    ([0x10, 0x00, 0x00], Keys::Ctrl('p')),
    ([0x11, 0x00, 0x00], Keys::Ctrl('q')),
    ([0x12, 0x00, 0x00], Keys::Ctrl('r')),
    ([0x13, 0x00, 0x00], Keys::Ctrl('s')),
    ([0x14, 0x00, 0x00], Keys::Ctrl('t')),
    ([0x15, 0x00, 0x00], Keys::Ctrl('u')),
    ([0x16, 0x00, 0x00], Keys::Ctrl('v')),
    ([0x17, 0x00, 0x00], Keys::Ctrl('w')),
    ([0x18, 0x00, 0x00], Keys::Ctrl('x')),
    ([0x19, 0x00, 0x00], Keys::Ctrl('y')),
    ([0x1A, 0x00, 0x00], Keys::Ctrl('z')),
];
```

</details>

<details>
<summary>ctrl_upper_letter</summary>

* remember upper & lower case in Ctrl + key is the same hex code. 

```rust
pub const CTRL_UPPER_LETTER: [([u8; BYTES], Keys); 24] = [
    (CTRL_LOWER_LETTER[0].0, Keys::Ctrl('A')),
    (CTRL_LOWER_LETTER[1].0, Keys::Ctrl('B')),
    (CTRL_LOWER_LETTER[2].0, Keys::Ctrl('C')),
    (CTRL_LOWER_LETTER[3].0, Keys::Ctrl('D')),
    (CTRL_LOWER_LETTER[4].0, Keys::Ctrl('E')),
    (CTRL_LOWER_LETTER[5].0, Keys::Ctrl('F')),
    (CTRL_LOWER_LETTER[6].0, Keys::Ctrl('G')),
    (CTRL_LOWER_LETTER[7].0, Keys::Ctrl('H')),
    (CTRL_LOWER_LETTER[8].0, Keys::Ctrl('K')),
    (CTRL_LOWER_LETTER[9].0, Keys::Ctrl('L')),
    (CTRL_LOWER_LETTER[10].0, Keys::Ctrl('M')),
    (CTRL_LOWER_LETTER[11].0, Keys::Ctrl('N')),
    (CTRL_LOWER_LETTER[12].0, Keys::Ctrl('O')),
    (CTRL_LOWER_LETTER[13].0, Keys::Ctrl('P')),
    (CTRL_LOWER_LETTER[14].0, Keys::Ctrl('Q')),
    (CTRL_LOWER_LETTER[15].0, Keys::Ctrl('R')),
    (CTRL_LOWER_LETTER[16].0, Keys::Ctrl('S')),
    (CTRL_LOWER_LETTER[17].0, Keys::Ctrl('T')),
    (CTRL_LOWER_LETTER[18].0, Keys::Ctrl('U')),
    (CTRL_LOWER_LETTER[19].0, Keys::Ctrl('V')),
    (CTRL_LOWER_LETTER[20].0, Keys::Ctrl('W')),
    (CTRL_LOWER_LETTER[21].0, Keys::Ctrl('X')),
    (CTRL_LOWER_LETTER[22].0, Keys::Ctrl('Y')),
    (CTRL_LOWER_LETTER[23].0, Keys::Ctrl('Z')),
];
```

</details>

<details>
<summary>ctrl_standar</summary>

```rust
pub const CTRL_STANDAR: [([u8; BYTES], Keys); 2] = [
    ([0x2b, 0x00, 0x00], Keys::Ctrl('+')),
    ([0x1f, 0x00, 0x00], Keys::Ctrl('-')),
];
```

</details>

<details>
<summary>ctrl_numbers</summary>

```rust
pub const CTRL_NUMBERS: [([u8; BYTES], Keys); 10] = [
    ([0x30, 0x00, 0x00], Keys::Ctrl('0')),
    ([0x31, 0x00, 0x00], Keys::Ctrl('1')),
    ([0x32, 0x00, 0x00], Keys::Ctrl('2')),
    ([0x33, 0x00, 0x00], Keys::Ctrl('3')),
    ([0x34, 0x00, 0x00], Keys::Ctrl('4')),
    ([0x35, 0x00, 0x00], Keys::Ctrl('5')),
    ([0x36, 0x00, 0x00], Keys::Ctrl('6')),
    ([0x37, 0x00, 0x00], Keys::Ctrl('7')),
    ([0x38, 0x00, 0x00], Keys::Ctrl('8')),
    ([0x39, 0x00, 0x00], Keys::Ctrl('9')),
];
```

</details>


<details>
<summary>alt_lower_letter</summary>

```rust
pub const ALT_LOWER_LETTER: [([u8; BYTES], Keys); 27] = [
    ([0x1b, 0x61, 0x00], Keys::Alt('a')),
    ([0x1b, 0x62, 0x00], Keys::Alt('b')),
    ([0x1b, 0x63, 0x00], Keys::Alt('c')),
    ([0x1b, 0x64, 0x00], Keys::Alt('d')),
    ([0x1b, 0x65, 0x00], Keys::Alt('e')),
    ([0x1b, 0x66, 0x00], Keys::Alt('f')),
    ([0x1b, 0x67, 0x00], Keys::Alt('g')),
    ([0x1b, 0x68, 0x00], Keys::Alt('h')),
    ([0x1b, 0x69, 0x00], Keys::Alt('i')),
    ([0x1b, 0x6A, 0x00], Keys::Alt('j')),
    ([0x1b, 0x6B, 0x00], Keys::Alt('k')),
    ([0x1b, 0x6C, 0x00], Keys::Alt('l')),
    ([0x1b, 0x6d, 0x00], Keys::Alt('m')),
    ([0x1b, 0x6e, 0x00], Keys::Alt('n')),
    ([0x1b, 0xc3, 0xb1], Keys::Alt('ñ')),
    ([0x1b, 0x6f, 0x00], Keys::Alt('o')),
    ([0x1b, 0x70, 0x00], Keys::Alt('p')),
    ([0x1b, 0x71, 0x00], Keys::Alt('q')),
    ([0x1b, 0x72, 0x00], Keys::Alt('r')),
    ([0x1b, 0x73, 0x00], Keys::Alt('s')),
    ([0x1b, 0x74, 0x00], Keys::Alt('t')),
    ([0x1b, 0x75, 0x00], Keys::Alt('u')),
    ([0x1b, 0x76, 0x00], Keys::Alt('v')),
    ([0x1b, 0x77, 0x00], Keys::Alt('w')),
    ([0x1b, 0x78, 0x00], Keys::Alt('x')),
    ([0x1b, 0x79, 0x00], Keys::Alt('y')),
    ([0x1b, 0x7a, 0x00], Keys::Alt('z')),
];
```
</details>


<details>
<summary>alt_upper_letter</summary>

```rust
pub const ALT_UPPER_LETTER: [([u8; BYTES], Keys); 27] = [
    ([0x1b, 0x41, 0x00], Keys::Alt('A')),
    ([0x1b, 0x42, 0x00], Keys::Alt('B')),
    ([0x1b, 0x43, 0x00], Keys::Alt('C')),
    ([0x1b, 0x44, 0x00], Keys::Alt('D')),
    ([0x1b, 0x45, 0x00], Keys::Alt('E')),
    ([0x1b, 0x46, 0x00], Keys::Alt('F')),
    ([0x1b, 0x47, 0x00], Keys::Alt('G')),
    ([0x1b, 0x48, 0x00], Keys::Alt('H')),
    ([0x1b, 0x49, 0x00], Keys::Alt('I')),
    ([0x1b, 0x4A, 0x00], Keys::Alt('J')),
    ([0x1b, 0x4B, 0x00], Keys::Alt('K')),
    ([0x1b, 0x4C, 0x00], Keys::Alt('L')),
    ([0x1b, 0x4D, 0x00], Keys::Alt('M')),
    ([0x1b, 0x4E, 0x00], Keys::Alt('N')),
    ([0x1b, 0xc3, 0x91], Keys::Alt('Ñ')),
    ([0x1b, 0x4f, 0x00], Keys::Alt('O')),
    ([0x1b, 0x50, 0x00], Keys::Alt('P')),
    ([0x1b, 0x51, 0x00], Keys::Alt('Q')),
    ([0x1b, 0x52, 0x00], Keys::Alt('R')),
    ([0x1b, 0x53, 0x00], Keys::Alt('S')),
    ([0x1b, 0x54, 0x00], Keys::Alt('T')),
    ([0x1b, 0x55, 0x00], Keys::Alt('U')),
    ([0x1b, 0x56, 0x00], Keys::Alt('V')),
    ([0x1b, 0x57, 0x00], Keys::Alt('W')),
    ([0x1b, 0x58, 0x00], Keys::Alt('X')),
    ([0x1b, 0x59, 0x00], Keys::Alt('Y')),
    ([0x1b, 0x5A, 0x00], Keys::Alt('Z')),
];
```
</details>

<details>
<summary>alt_numbers</summary>

* remember Alt + number is the same hex code as Ctrl + number. 

```rust
pub const ALT_NUMBERS: [([u8; BYTES], Keys); 10] = [
    (CTRL_NUMBERS[0].0, Keys::Alt('0')),
    (CTRL_NUMBERS[1].0, Keys::Alt('1')),
    (CTRL_NUMBERS[2].0, Keys::Alt('2')),
    (CTRL_NUMBERS[3].0, Keys::Alt('3')),
    (CTRL_NUMBERS[4].0, Keys::Alt('4')),
    (CTRL_NUMBERS[5].0, Keys::Alt('5')),
    (CTRL_NUMBERS[6].0, Keys::Alt('6')),
    (CTRL_NUMBERS[7].0, Keys::Alt('7')),
    (CTRL_NUMBERS[8].0, Keys::Alt('8')),
    (CTRL_NUMBERS[9].0, Keys::Alt('9')),
];
```

</details>

<details>
<summary>alt_gr_letter</summary>

```rust
pub const ALT_GR_LETTER: [([u8; BYTES], Keys); 27] = [
    ([0xc3, 0xa6, 0x00], Keys::AltGr('a')),
    ([0xe2, 0x80, 0x9c], Keys::AltGr('b')),
    ([0xc2, 0xa2, 0x00], Keys::AltGr('c')),
    ([0xc3, 0xb0, 0x00], Keys::AltGr('d')),
    ([0xe2, 0x82, 0xac], Keys::AltGr('e')),
    ([0xc4, 0x91, 0x00], Keys::AltGr('f')),
    ([0xc5, 0x8b, 0x00], Keys::AltGr('g')),
    ([0xc4, 0xa7, 0x00], Keys::AltGr('h')),
    ([0xe2, 0x86, 0x92], Keys::AltGr('i')),
    ([0xcc, 0x89, 0x00], Keys::AltGr('j')),
    ([0xc4, 0xb8, 0x00], Keys::AltGr('k')),
    ([0xc5, 0x82, 0x00], Keys::AltGr('l')),
    ([0xc2, 0xb5, 0x00], Keys::AltGr('m')),
    ([0xe2, 0x80, 0x9d], Keys::AltGr('n')),
    ([0x7e, 0x00, 0x00], Keys::AltGr('ñ')),
    ([0xc3, 0xb8, 0x00], Keys::AltGr('o')),
    ([0xc3, 0xbe, 0x00], Keys::AltGr('p')),
    ([0x40, 0x00, 0x00], Keys::AltGr('q')),
    ([0xc2, 0xb6, 0x00], Keys::AltGr('r')),
    ([0xc3, 0x9f, 0x00], Keys::AltGr('s')),
    ([0xc5, 0xa7, 0x00], Keys::AltGr('t')),
    ([0xe2, 0x86, 0x93], Keys::AltGr('u')),
    ([0xe2, 0x80, 0x9e], Keys::AltGr('v')),
    ([0xc5, 0xbf, 0x00], Keys::AltGr('w')),
    ([0xc2, 0xbb, 0x00], Keys::AltGr('x')),
    ([0xe2, 0x86, 0x90], Keys::AltGr('y')),
    ([0xc2, 0xab, 0x00], Keys::AltGr('z')),
];
```

</details>

<details>
<summary>alt_gr_numbers</summary>

* remember Alt Gr + number is the same hex code as Ctrl + number. 

```rust
pub const ALT_GR_NUMBERS: [([u8; BYTES], Keys); 10] = [
    (CTRL_NUMBERS[0].0, Keys::Alt('0')),
    (CTRL_NUMBERS[1].0, Keys::Alt('1')),
    (CTRL_NUMBERS[2].0, Keys::Alt('2')),
    (CTRL_NUMBERS[3].0, Keys::Alt('3')),
    (CTRL_NUMBERS[4].0, Keys::Alt('4')),
    (CTRL_NUMBERS[5].0, Keys::Alt('5')),
    (CTRL_NUMBERS[6].0, Keys::Alt('6')),
    (CTRL_NUMBERS[7].0, Keys::Alt('7')),
    (CTRL_NUMBERS[8].0, Keys::Alt('8')),
    (CTRL_NUMBERS[9].0, Keys::Alt('9')),
];
```

</details>

</details>

<details>
<summary>full</summary>

all features!

</details>