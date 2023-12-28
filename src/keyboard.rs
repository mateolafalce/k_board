/***************************************************************************************
 *   keyboard.rs  --  This file is part of k_board.                                    *
 *                                                                                     *
 *   Copyright (C) 2023 Mateo Lafalce                                                  *
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

use crate::{get_key_from_keyboard, Keys};

pub struct Keyboard;

impl Default for Keyboard {
    fn default() -> Self {
        Self::new()
    }
}

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
