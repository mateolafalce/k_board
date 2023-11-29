// Copyright (C) 2023  Mateo Lafalce
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//!
//! [<img alt="crates.io" src="https://img.shields.io/crates/v/k_board.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/k_board)
//! [<img alt="github" src="https://img.shields.io/badge/github-mateolafalce/k__board-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/mateolafalce/k_board)
//! [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-k__board-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/k_board)
//!
//! A lightweight keyboard mannager developed for dynamic programs by listening to keyboard events in raw mode (without the need to press enter). The handler has all the standard events of a western keyboard.
//!
//! - Gnu/Linux
//!
//! # Examples
//!
//! ```
//! use k_board::{Keyboard, Keys};
//!
//! fn main() {
//!     menu(0);
//!     for key in Keyboard::new() {
//!         match key {
//!             Keys::Up => menu(0),
//!             Keys::Down => menu(1),
//!             Keys::Enter => break,
//!             _ => {}
//!         }
//!     }
//! }
//!
//! fn menu(operation: u8) {
//!     std::process::Command::new("clear").status().unwrap();
//!     let mut op: Vec<char> = vec!['*', ' '];
//!     if operation == 1 {
//!         op[0] = ' ';
//!         op[1] = '*';
//!     }
//!     println!(
//!         "[{}] I use k_board lightweight software\n[{}] I use heavyweight software",
//!         op[0], op[1]
//!     );
//! }
//! ```
//!
//! # Contributing
//! Feel free to contribute to the repository. Make each modification to the code formatted with code before with `cargo fmt`. Below, a fragment that allows you to visualize in hexadecimal the key or the event executed on your keyboard:
//!
//! ```rust
//! use std::io::{Read, Write};
//!
//! fn main() -> std::io::Result<()> {
//!     println!("Press a key or an keyboard event!");
//!     loop {
//!         let _ = get_key();
//!     }
//! }
//!
//! pub fn get_key() -> std::io::Result<()> {
//!     let termios_enviroment: k_board::termios = k_board::setup_raw_mode().unwrap();
//!     std::io::stdout().flush().unwrap();
//!     let mut buffer: [u8; 3] = [0; 3];
//!     std::io::stdin().read(&mut buffer).unwrap();
//!     if buffer[0] != 0x00 {
//!         println!(
//!             "[0x{:x?}, 0x{:x?}, 0x{:x?}]",
//!             buffer[0], buffer[1], buffer[2]
//!         );
//!     }
//!     std::io::stdout().flush().unwrap();
//!     k_board::restore_termios(&termios_enviroment).unwrap();
//!     Ok(())
//! }
//! ```
//!

use std::io::{self, Read, Write};

#[derive(Debug, PartialEq, Eq)]
pub enum Keys {
    Up,
    Down,
    Left,
    Right,
    Enter,
    Space,
    Tab,
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
    OpenExclamationMark,
    ClosedExclamationMark,
    QuotationMark,
    Backquote,
    AcuteAccent,
    Home,
    End,
    Backtab,
    Insert,
    Letter(char),
    Number(u8),
    F(u8),
    Ctrl(char),
    Alt(char),
    Null,
}

pub fn get_key_from_keyboard() -> Keys {
    let termios_enviroment: termios = setup_raw_mode().unwrap();
    std::io::stdout().flush().unwrap();
    let mut buffer: [u8; 3] = [0; 3];
    let mut key: Keys = Keys::Null;
    std::io::stdin().read(&mut buffer).unwrap();
    match buffer {
        [0x1B, 0x5B, 0x41] => key = Keys::Up,
        [0x1B, 0x5B, 0x42] => key = Keys::Down,
        [0x1B, 0x5B, 0x43] => key = Keys::Right,
        [0x1B, 0x5B, 0x44] => key = Keys::Left,
        [0x1B, 0x5B, 0x48] => key = Keys::Home,
        [0x09, 0x00, 0x00] => key = Keys::Tab,
        [0x1B, 0x5B, 0x46] => key = Keys::End,
        [0x1B, 0x5B, 0x5a] => key = Keys::Backtab,
        [0x0A, 0x00, 0x00] => key = Keys::Enter,
        [0x20, 0x00, 0x00] => key = Keys::Space,
        [0x7F, 0x00, 0x00] => key = Keys::Delete,
        [0x2b, 0x00, 0x00] => key = Keys::Plus,
        [0x2d, 0x00, 0x00] => key = Keys::Minus,
        [0x3d, 0x00, 0x00] => key = Keys::Equal,
        [0x2f, 0x00, 0x00] => key = Keys::Slash,
        [0x5c, 0x00, 0x00] => key = Keys::Backslash,
        [0x5e, 0x00, 0x00] => key = Keys::Power,
        [0x2a, 0x00, 0x00] => key = Keys::Asterisk,
        [0x2e, 0x00, 0x00] => key = Keys::Point,
        [0x2c, 0x00, 0x00] => key = Keys::Comma,
        [0x23, 0x00, 0x00] => key = Keys::Hashtag,
        [0x26, 0x00, 0x00] => key = Keys::Ampersand,
        [0x25, 0x00, 0x00] => key = Keys::Percent,
        [0x7c, 0x00, 0x00] => key = Keys::Pipe,
        [0x24, 0x00, 0x00] => key = Keys::Currency,
        [0x3a, 0x00, 0x00] => key = Keys::TwoPoints,
        [0x3b, 0x00, 0x00] => key = Keys::Semicolon,
        [0xc2, 0xbf, 0x00] => key = Keys::OpenQuestionMark,
        [0x3f, 0x00, 0x00] => key = Keys::CloseQuestionMark,
        [0x5b, 0x00, 0x00] => key = Keys::OpenSquareBracket,
        [0x5d, 0x00, 0x00] => key = Keys::CloseSquareBracket,
        [0x7b, 0x00, 0x00] => key = Keys::OpenCurlyBrace,
        [0x7d, 0x00, 0x00] => key = Keys::CloseCurlyBrace,
        [0x28, 0x00, 0x00] => key = Keys::OpenParenthesis,
        [0x29, 0x00, 0x00] => key = Keys::CloseParenthesis,
        [0x3c, 0x00, 0x00] => key = Keys::LessThan,
        [0x3e, 0x00, 0x00] => key = Keys::GreaterThan,
        [0x27, 0x00, 0x00] => key = Keys::Apostrophe,
        [0x40, 0x00, 0x00] => key = Keys::At,
        [0xc2, 0xa1, 0x0] => key = Keys::OpenExclamationMark,
        [0x21, 0x00, 0x00] => key = Keys::ClosedExclamationMark,
        [0x22, 0x0, 0x0] => key = Keys::QuotationMark,
        [0x60, 0x0, 0x0] => key = Keys::Backquote,
        [0xc2, 0xb4, 0x00] => key = Keys::AcuteAccent,
        [0x30, 0x00, 0x00] => key = Keys::Number(0),
        [0x31, 0x00, 0x00] => key = Keys::Number(1),
        [0x32, 0x00, 0x00] => key = Keys::Number(2),
        [0x33, 0x00, 0x00] => key = Keys::Number(3),
        [0x34, 0x00, 0x00] => key = Keys::Number(4),
        [0x35, 0x00, 0x00] => key = Keys::Number(5),
        [0x36, 0x00, 0x00] => key = Keys::Number(6),
        [0x37, 0x00, 0x00] => key = Keys::Number(7),
        [0x38, 0x00, 0x00] => key = Keys::Number(8),
        [0x39, 0x00, 0x00] => key = Keys::Number(9),
        [0x1b, 0x4f, 0x50] => key = Keys::F(1),
        [0x1b, 0x4f, 0x51] => key = Keys::F(2),
        [0x1b, 0x4f, 0x52] => key = Keys::F(3),
        [0x1b, 0x4f, 0x53] => key = Keys::F(4),
        [0x35, 0x7E, 0x00] => key = Keys::F(5),
        [0x37, 0x7E, 0x00] => key = Keys::F(6),
        [0x38, 0x7E, 0x00] => key = Keys::F(7),
        [0x39, 0x7E, 0x00] => key = Keys::F(8),
        [0x30, 0x7E, 0x00] => key = Keys::F(9),
        [0x31, 0x7E, 0x00] => key = Keys::F(10),
        [0x33, 0x7E, 0x00] => key = Keys::F(11),
        [0x34, 0x7E, 0x00] => key = Keys::F(12),
        [0x01, 0x00, 0x00] => key = Keys::Ctrl('a'),
        [0x02, 0x00, 0x00] => key = Keys::Ctrl('b'),
        [0x03, 0x00, 0x00] => key = Keys::Ctrl('c'),
        [0x04, 0x00, 0x00] => key = Keys::Ctrl('d'),
        [0x05, 0x00, 0x00] => key = Keys::Ctrl('e'),
        [0x06, 0x00, 0x00] => key = Keys::Ctrl('f'),
        [0x07, 0x00, 0x00] => key = Keys::Ctrl('g'),
        [0x08, 0x00, 0x00] => key = Keys::Ctrl('h'),
        [0x0B, 0x00, 0x00] => key = Keys::Ctrl('k'),
        [0x0C, 0x00, 0x00] => key = Keys::Ctrl('l'),
        [0x0D, 0x00, 0x00] => key = Keys::Ctrl('m'),
        [0x0E, 0x00, 0x00] => key = Keys::Ctrl('n'),
        [0x0F, 0x00, 0x00] => key = Keys::Ctrl('o'),
        [0x10, 0x00, 0x00] => key = Keys::Ctrl('p'),
        [0x11, 0x00, 0x00] => key = Keys::Ctrl('q'),
        [0x12, 0x00, 0x00] => key = Keys::Ctrl('r'),
        [0x13, 0x00, 0x00] => key = Keys::Ctrl('s'),
        [0x14, 0x00, 0x00] => key = Keys::Ctrl('t'),
        [0x15, 0x00, 0x00] => key = Keys::Ctrl('u'),
        [0x16, 0x00, 0x00] => key = Keys::Ctrl('v'),
        [0x17, 0x00, 0x00] => key = Keys::Ctrl('w'),
        [0x18, 0x00, 0x00] => key = Keys::Ctrl('x'),
        [0x19, 0x00, 0x00] => key = Keys::Ctrl('y'),
        [0x1A, 0x00, 0x00] => key = Keys::Ctrl('z'),
        [0x61, 0x00, 0x00] => key = Keys::Letter('a'),
        [0x62, 0x00, 0x00] => key = Keys::Letter('b'),
        [0x63, 0x00, 0x00] => key = Keys::Letter('c'),
        [0x64, 0x00, 0x00] => key = Keys::Letter('d'),
        [0x65, 0x00, 0x00] => key = Keys::Letter('e'),
        [0x66, 0x00, 0x00] => key = Keys::Letter('f'),
        [0x67, 0x00, 0x00] => key = Keys::Letter('g'),
        [0x68, 0x00, 0x00] => key = Keys::Letter('h'),
        [0x69, 0x00, 0x00] => key = Keys::Letter('i'),
        [0x6A, 0x00, 0x00] => key = Keys::Letter('j'),
        [0x6B, 0x00, 0x00] => key = Keys::Letter('k'),
        [0x6C, 0x00, 0x00] => key = Keys::Letter('l'),
        [0x6D, 0x00, 0x00] => key = Keys::Letter('m'),
        [0x6E, 0x00, 0x00] => key = Keys::Letter('n'),
        [0xb1, 0xb1, 0x00] => key = Keys::Letter('ñ'),
        [0x6F, 0x00, 0x00] => key = Keys::Letter('o'),
        [0x70, 0x00, 0x00] => key = Keys::Letter('p'),
        [0x71, 0x00, 0x00] => key = Keys::Letter('q'),
        [0x72, 0x00, 0x00] => key = Keys::Letter('r'),
        [0x73, 0x00, 0x00] => key = Keys::Letter('s'),
        [0x74, 0x00, 0x00] => key = Keys::Letter('t'),
        [0x75, 0x00, 0x00] => key = Keys::Letter('u'),
        [0x76, 0x00, 0x00] => key = Keys::Letter('v'),
        [0x77, 0x00, 0x00] => key = Keys::Letter('w'),
        [0x78, 0x00, 0x00] => key = Keys::Letter('x'),
        [0x79, 0x00, 0x00] => key = Keys::Letter('y'),
        [0x7A, 0x00, 0x00] => key = Keys::Letter('z'),
        [0x41, 0x00, 0x00] => key = Keys::Letter('A'),
        [0x42, 0x00, 0x00] => key = Keys::Letter('B'),
        [0x43, 0x00, 0x00] => key = Keys::Letter('C'),
        [0x44, 0x00, 0x00] => key = Keys::Letter('D'),
        [0x45, 0x00, 0x00] => key = Keys::Letter('E'),
        [0x46, 0x00, 0x00] => key = Keys::Letter('F'),
        [0x47, 0x00, 0x00] => key = Keys::Letter('G'),
        [0x48, 0x00, 0x00] => key = Keys::Letter('H'),
        [0x49, 0x00, 0x00] => key = Keys::Letter('I'),
        [0x4A, 0x00, 0x00] => key = Keys::Letter('J'),
        [0x4B, 0x00, 0x00] => key = Keys::Letter('K'),
        [0x4C, 0x00, 0x00] => key = Keys::Letter('L'),
        [0x4D, 0x00, 0x00] => key = Keys::Letter('M'),
        [0x4E, 0x00, 0x00] => key = Keys::Letter('N'),
        [0xb1, 0x91, 0x00] => key = Keys::Letter('Ñ'),
        [0x4F, 0x00, 0x00] => key = Keys::Letter('O'),
        [0x50, 0x00, 0x00] => key = Keys::Letter('P'),
        [0x51, 0x00, 0x00] => key = Keys::Letter('Q'),
        [0x52, 0x00, 0x00] => key = Keys::Letter('R'),
        [0x53, 0x00, 0x00] => key = Keys::Letter('S'),
        [0x54, 0x00, 0x00] => key = Keys::Letter('T'),
        [0x55, 0x00, 0x00] => key = Keys::Letter('U'),
        [0x56, 0x00, 0x00] => key = Keys::Letter('V'),
        [0x57, 0x00, 0x00] => key = Keys::Letter('W'),
        [0x58, 0x00, 0x00] => key = Keys::Letter('X'),
        [0x59, 0x00, 0x00] => key = Keys::Letter('Y'),
        [0x5A, 0x00, 0x00] => key = Keys::Letter('Z'),
        [0x1b, 0x61, 0x00] => key = Keys::Alt('a'),
        [0x1b, 0x62, 0x00] => key = Keys::Alt('b'),
        [0x1b, 0x63, 0x00] => key = Keys::Alt('c'),
        [0x1b, 0x64, 0x00] => key = Keys::Alt('d'),
        [0x1b, 0x65, 0x00] => key = Keys::Alt('e'),
        [0x1b, 0x66, 0x00] => key = Keys::Alt('f'),
        [0x1b, 0x67, 0x00] => key = Keys::Alt('g'),
        [0x1b, 0x68, 0x00] => key = Keys::Alt('h'),
        [0x1b, 0x69, 0x00] => key = Keys::Alt('i'),
        [0x1b, 0x6A, 0x00] => key = Keys::Alt('j'),
        [0x1b, 0x6B, 0x00] => key = Keys::Alt('k'),
        [0x1b, 0x6C, 0x00] => key = Keys::Alt('l'),
        [0x1b, 0x6d, 0x00] => key = Keys::Alt('m'),
        [0x1b, 0x6e, 0x00] => key = Keys::Alt('n'),
        [0x1b, 0xc3, 0xb1] => key = Keys::Alt('ñ'),
        [0x1b, 0x6f, 0x00] => key = Keys::Alt('o'),
        [0x1b, 0x70, 0x00] => key = Keys::Alt('p'),
        [0x1b, 0x71, 0x00] => key = Keys::Alt('q'),
        [0x1b, 0x72, 0x00] => key = Keys::Alt('r'),
        [0x1b, 0x73, 0x00] => key = Keys::Alt('s'),
        [0x1b, 0x74, 0x00] => key = Keys::Alt('t'),
        [0x1b, 0x75, 0x00] => key = Keys::Alt('u'),
        [0x1b, 0x76, 0x00] => key = Keys::Alt('v'),
        [0x1b, 0x77, 0x00] => key = Keys::Alt('w'),
        [0x1b, 0x78, 0x00] => key = Keys::Alt('x'),
        [0x1b, 0x79, 0x00] => key = Keys::Alt('y'),
        [0x1b, 0x7a, 0x00] => key = Keys::Alt('z'),
        _ => {}
    }
    std::io::stdout().flush().unwrap();
    restore_termios(&termios_enviroment).unwrap();
    key
}

pub struct Keyboard;

impl Keyboard {
    pub fn new() -> Self {
        Keyboard
    }
    pub fn read_key(&mut self) -> Keys {
        get_key_from_keyboard()
    }
}

impl Iterator for Keyboard {
    type Item = Keys;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.read_key())
    }
}

pub fn setup_raw_mode() -> io::Result<termios> {
    let mut termios: termios = unsafe { std::mem::zeroed() };
    if unsafe { tcgetattr(0, &mut termios) } < 0 {
        return Err(io::Error::last_os_error());
    }
    let original_termios = termios.clone();
    termios.c_lflag &= !(0x00000002 | 0x00000008);
    termios.c_iflag &= !(0x00000400 | 0x00001000 | 0x00000800);
    termios.c_cc[6] = 0;
    termios.c_cc[5] = 1;
    if unsafe { tcsetattr(0, 0, &termios) } < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(original_termios)
}

pub fn restore_termios(original_termios: &termios) -> io::Result<()> {
    if unsafe { tcsetattr(0, 0, original_termios) } < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

// look at https://linux.die.net/man/3/termios
#[derive(Clone)]
#[repr(C)]
pub struct termios {
    pub c_iflag: u32,
    pub c_oflag: u32,
    pub c_cflag: u32,
    pub c_lflag: u32,
    c_line: u8,
    pub c_cc: [u8; 32],
    c_ispeed: u32,
    c_ospeed: u32,
}

#[link(name = "c")]
extern "C" {
    pub fn tcsetattr(
        fd: std::ffi::c_int,
        optional_actions: std::ffi::c_int,
        termios_p: *const termios,
    ) -> std::ffi::c_int;
    pub fn tcgetattr(fd: std::ffi::c_int, termios: *mut termios) -> std::ffi::c_int;
}
