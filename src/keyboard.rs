/***************************************************************************************
 *   keyboard.rs  --  This file is part of k_board.                                    *
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

use crate::{
    keys::{Keys, ARROWS_ENTER, BYTES},
    termio::{restore, setup_raw_mode, termios},
};
use std::io::{stdin, stdout, Read, Write};

#[cfg(any(
    feature = "ctrl_lower_letter",
    feature = "ctrl_upper_letter",
    feature = "full"
))]
use crate::termio::{sig_handler, signal, SIGINT};

#[cfg(any(
    feature = "ctrl_lower_letter",
    feature = "ctrl_upper_letter",
    feature = "full"
))]
use std::sync::Mutex;

/// Global varible to get the CTRL+C event
#[cfg(any(
    feature = "ctrl_lower_letter",
    feature = "ctrl_upper_letter",
    feature = "full"
))]
pub(crate) static CTRL_C: Mutex<bool> = Mutex::new(false);

/// Keyboard struct
pub struct Keyboard;

impl Default for Keyboard {
    fn default() -> Self {
        Self::new()
    }
}

impl Keyboard {
    /// Get a Keyboard instance
    pub fn new() -> Self {
        Keyboard
    }
}

impl Iterator for Keyboard {
    type Item = Keys;
    fn next(&mut self) -> Option<Keys> {
        Some(get_key_from_keyboard())
    }
}

/// Get the key press from the keyboard looking the
/// hex data in the I/O termios
pub fn get_key_from_keyboard() -> Keys {
    #[cfg(any(
        feature = "ctrl_lower_letter",
        feature = "ctrl_upper_letter",
        feature = "full"
    ))]
    unsafe {
        signal(SIGINT, sig_handler as usize)
    };

    let termios_enviroment: termios = setup_raw_mode().unwrap();
    stdout().flush().unwrap();
    let mut buffer: [u8; BYTES] = [0; BYTES];
    let mut key: Keys = Keys::Null;
    match stdin().read(&mut buffer) {
        Ok(_) => (),
        Err(err) => eprintln!("Error: {}", err),
    }

    for &(ref pattern, keys) in ARROWS_ENTER.iter() {
        if buffer == *pattern {
            restore(&termios_enviroment).expect("Error with termios restore");
            key = keys;
        }
    }

    #[cfg(any(feature = "standar", feature = "full"))]
    for &(ref pattern, keys) in crate::keys::STANDAR.iter() {
        if buffer == *pattern {
            restore(&termios_enviroment).expect("Error with termios restore");
            key = keys;
        }
    }

    #[cfg(any(feature = "numbers", feature = "full"))]
    for &(ref pattern, keys) in crate::keys::NUMBERS.iter() {
        if buffer == *pattern {
            restore(&termios_enviroment).expect("Error with termios restore");
            key = keys;
        }
    }

    #[cfg(any(feature = "lower_letter", feature = "full"))]
    for &(ref pattern, keys) in crate::keys::LOWER_LETTERS.iter() {
        if buffer == *pattern {
            restore(&termios_enviroment).expect("Error with termios restore");
            key = keys;
        }
    }

    #[cfg(any(feature = "upper_letter", feature = "full"))]
    for &(ref pattern, keys) in crate::keys::UPPER_LETTER.iter() {
        if buffer == *pattern {
            restore(&termios_enviroment).expect("Error with termios restore");
            key = keys;
        }
    }

    #[cfg(any(feature = "f", feature = "full"))]
    for &(ref pattern, keys) in crate::keys::F.iter() {
        if buffer == *pattern {
            restore(&termios_enviroment).expect("Error with termios restore");
            key = keys;
        }
    }

    #[cfg(any(feature = "ctrl_lower_letter", feature = "full"))]
    for &(ref pattern, keys) in crate::keys::CTRL_LOWER_LETTER.iter() {
        let mut ctrl_c_handler = CTRL_C.lock().unwrap();
        if *ctrl_c_handler {
            *ctrl_c_handler = false;
            restore(&termios_enviroment).expect("Error with termios restore");
            return Keys::Ctrl('c');
        }
        if buffer == *pattern {
            restore(&termios_enviroment).expect("Error with termios restore");
            key = keys;
        }
    }

    #[cfg(any(feature = "ctrl_upper_letter", feature = "full"))]
    for &(ref pattern, keys) in crate::keys::CTRL_UPPER_LETTER.iter() {
        let mut ctrl_c_handler = CTRL_C.lock().unwrap();
        if *ctrl_c_handler {
            *ctrl_c_handler = false;
            restore(&termios_enviroment).expect("Error with termios restore");
            return Keys::Ctrl('C');
        }
        if buffer == *pattern {
            restore(&termios_enviroment).expect("Error with termios restore");
            key = keys;
        }
    }

    #[cfg(any(feature = "ctrl_standar", feature = "full",))]
    #[cfg(not(feature = "standar"))]
    for &(ref pattern, keys) in crate::keys::CTRL_STANDAR.iter() {
        if buffer == *pattern {
            restore(&termios_enviroment).expect("Error with termios restore");
            key = keys;
        }
    }

    #[cfg(any(feature = "ctrl_numbers", feature = "full"))]
    for &(ref pattern, keys) in crate::keys::CTRL_NUMBERS.iter() {
        if buffer == *pattern {
            restore(&termios_enviroment).expect("Error with termios restore");
            key = keys;
        }
    }

    #[cfg(any(feature = "alt_lower_letter", feature = "full"))]
    for &(ref pattern, keys) in crate::keys::ALT_LOWER_LETTER.iter() {
        if buffer == *pattern {
            restore(&termios_enviroment).expect("Error with termios restore");
            key = keys;
        }
    }

    #[cfg(any(feature = "alt_upper_letter", feature = "full"))]
    for &(ref pattern, keys) in crate::keys::ALT_UPPER_LETTER.iter() {
        if buffer == *pattern {
            restore(&termios_enviroment).expect("Error with termios restore");
            key = keys;
        }
    }

    #[cfg(any(feature = "alt_numbers", feature = "full"))]
    for &(ref pattern, keys) in crate::keys::ALT_NUMBERS.iter() {
        if buffer == *pattern {
            restore(&termios_enviroment).expect("Error with termios restore");
            key = keys;
        }
    }

    #[cfg(any(feature = "alt_gr_lower_letter", feature = "full"))]
    for &(ref pattern, keys) in crate::keys::ALT_GR_LOWER_LETTER.iter() {
        if buffer == *pattern {
            restore(&termios_enviroment).expect("Error with termios restore");
            key = keys;
        }
    }

    #[cfg(any(feature = "alt_gr_upper_letter", feature = "full"))]
    for &(ref pattern, keys) in crate::keys::ALT_GR_UPPER_LETTER.iter() {
        if buffer == *pattern {
            restore(&termios_enviroment).expect("Error with termios restore");
            key = keys;
        }
    }

    #[cfg(any(feature = "alt_gr_numbers", feature = "full"))]
    for &(ref pattern, keys) in crate::keys::ALT_GR_NUMBERS.iter() {
        if buffer == *pattern {
            restore(&termios_enviroment).expect("Error with termios restore");
            key = keys;
        }
    }

    restore(&termios_enviroment).expect("Error with termios restore");

    #[cfg(any(
        feature = "ctrl_lower_letter",
        feature = "ctrl_upper_letter",
        feature = "full"
    ))]
    unsafe {
        signal(SIGINT, 0)
    };

    key
}
