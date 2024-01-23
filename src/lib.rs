/***************************************************************************************
 *   lib.rs  --  This file is part of k_board.                                         *
 *                                                                                     *
 *   Copyright (C) 2024 Mateo Lafalce                                                  *
 *                                                                                     *
 *   k_board is free software: you can redistribute it and/or modify                   *
 *   it under the terms of the GNU General Public License as published                 *
 *   by the Free Software Foundation, either version 3 of the License,                 *
 *   or (at your option) any later version.                                            *
 *                                                                                     *
 *   k_board is distributed in the hope that it will be useful,                        *
 *   but WITHOUT ANY WARRANTY; without even the implied warranty                       *
 *   of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.                           *
 *   See the GNU General Public License for more details.                              *
 *                                                                                     *
 *   You should have received a copy of the GNU General Public License                 *
 *   along with this program.  If not, see http://www.gnu.org/licenses/.               *
 *                                                                                     *
 **************************************************************************************/

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

/// Keyboard struct & impls
pub mod keyboard;
/// All keys tables
pub mod keys;
/// Termios raw ops (linux kernel)
pub mod termio;
