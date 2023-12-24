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
//! ```rust,ignore
//! use k_board::{keyboard::Keyboard, keys::Keys};
//!
//!fn main() {
//!    menu(0);
//!    for key in Keyboard::new() {
//!        match key {
//!            Keys::Up => menu(0),
//!            Keys::Down => menu(1),
//!            Keys::Enter => break,
//!            _ => {}
//!        }
//!    }
//!}
//!
//!fn menu(operation: u8) {
//!    std::process::Command::new("clear").status().unwrap();
//!    let mut op: Vec<char> = vec!['*', ' '];
//!    if operation == 1 {
//!        op[0] = ' ';
//!        op[1] = '*';
//!    }
//!    println!(
//!        "[{}] I use k_board lightweight software\n[{}] I use heavyweight software",
//!        op[0], op[1]
//!    );
//!}
//! ```
//!
//! # Contributing
//! Feel free to contribute to the repository. Make each modification to the code formatted with code before with `cargo fmt`. Below, a fragment that allows you to visualize in hexadecimal the key or the event executed on your keyboard:
//!
//! ```rust,ignore
//!use k_board::termio::{restore_termios, setup_raw_mode, termios};
//!use std::io::{Read, Write};
//!
//!fn main() -> std::io::Result<()> {
//!    println!("Press a key or an keyboard event!");
//!    loop {
//!        let _ = get_key();
//!    }
//!}
//!
//!pub fn get_key() -> std::io::Result<()> {
//!    let termios_enviroment: termios = setup_raw_mode()?;
//!    std::io::stdout().flush().unwrap();
//!    let mut buffer: [u8; 3] = [0; 3];
//!    #[allow(clippy::unused_io_amount)]
//!    std::io::stdin().read(&mut buffer)?;
//!    if buffer[0] != 0x00 {
//!        println!(
//!            "[0x{:x?}, 0x{:x?}, 0x{:x?}]",
//!            buffer[0], buffer[1], buffer[2]
//!        );
//!    }
//!    std::io::stdout().flush().unwrap();
//!    restore_termios(&termios_enviroment)?;
//!    Ok(())
//!}
//! ```
//!

pub mod keyboard;
pub mod keys;
pub mod termio;

use crate::{
    keys::{Keys, BYTES},
    termio::{restore, setup_raw_mode, termios},
};
use std::io::{Read, Write};

pub fn get_key_from_keyboard() -> Keys {
    let termios_enviroment: termios = setup_raw_mode().unwrap();
    std::io::stdout().flush().unwrap();
    let mut buffer: [u8; BYTES] = [0; BYTES];
    let mut key: Keys = Keys::Null;
    match std::io::stdin().read(&mut buffer) {
        Ok(_) => (),
        Err(err) => eprintln!("Error: {}", err),
    }

    for &(ref pattern, keys) in crate::keys::ARROWS_ENTER.iter() {
        if buffer == *pattern {
            restore(&termios_enviroment);
            key = keys;
        }
    }

    #[cfg(feature = "standar")]
    for &(ref pattern, keys) in crate::keys::STANDAR.iter() {
        if buffer == *pattern {
            restore(&termios_enviroment);
            key = keys;
        }
    }

    #[cfg(feature = "numbers")]
    for &(ref pattern, keys) in crate::keys::NUMBERS.iter() {
        if buffer == *pattern {
            restore(&termios_enviroment);
            key = keys;
        }
    }

    #[cfg(feature = "lower_letter")]
    for &(ref pattern, keys) in crate::keys::LOWER_LETTERS.iter() {
        if buffer == *pattern {
            restore(&termios_enviroment);
            key = keys;
        }
    }

    #[cfg(feature = "upper_letter")]
    for &(ref pattern, keys) in crate::keys::UPPER_LETTER.iter() {
        if buffer == *pattern {
            restore(&termios_enviroment);
            key = keys;
        }
    }

    #[cfg(feature = "f")]
    for &(ref pattern, keys) in crate::keys::F.iter() {
        if buffer == *pattern {
            restore(&termios_enviroment);
            key = keys;
        }
    }

    #[cfg(feature = "ctrl_lower_letter")]
    for &(ref pattern, keys) in crate::keys::CTRL_LOWER_LETTER.iter() {
        if buffer == *pattern {
            restore(&termios_enviroment);
            key = keys;
        }
    }

    #[cfg(feature = "alt_lower_letter")]
    for &(ref pattern, keys) in crate::keys::ALT_LOWER_LETTER.iter() {
        if buffer == *pattern {
            restore(&termios_enviroment);
            key = keys;
        }
    }

    restore(&termios_enviroment);
    key
}
