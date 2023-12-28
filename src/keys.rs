/***************************************************************************************
 *   keys.rs  --  This file is part of k_board.                                        *
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

pub const BYTES: usize = 3;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Keys {
    Up,
    Down,
    Left,
    Right,
    Enter,
    Home,
    Tab,
    Backtab,
    Space,
    Delete,
    Escape,
    End,
    Char(char),
    F(u8),
    Ctrl(char),
    Alt(char),
    AltGr(char),
    Null,
}

pub const ARROWS_ENTER: [([u8; BYTES], Keys); 5] = [
    ([0x1B, 0x5B, 0x41], Keys::Up),
    ([0x1B, 0x5B, 0x42], Keys::Down),
    ([0x1B, 0x5B, 0x43], Keys::Right),
    ([0x1B, 0x5B, 0x44], Keys::Left),
    ([0x0A, 0x00, 0x00], Keys::Enter),
];

#[cfg(any(feature = "standar", feature = "full"))]
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

#[cfg(any(feature = "numbers", feature = "full"))]
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

#[cfg(any(feature = "lower_letter", feature = "full"))]
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

#[cfg(any(feature = "upper_letter", feature = "full"))]
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

#[cfg(any(feature = "f", feature = "full"))]
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

#[cfg(any(
    feature = "ctrl_lower_letter",
    feature = "ctrl_upper_letter",
    feature = "full"
))]
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

#[cfg(any(feature = "ctrl_upper_letter", feature = "full"))]
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

#[cfg(any(feature = "ctrl_standar", feature = "full"))]
pub const CTRL_STANDAR: [([u8; BYTES], Keys); 2] = [
    ([0x2b, 0x00, 0x00], Keys::Ctrl('+')),
    ([0x1f, 0x00, 0x00], Keys::Ctrl('-')),
];

#[cfg(any(
    feature = "ctrl_numbers",
    feature = "alt_numbers",
    feature = "alt_gr_numbers",
    feature = "full"
))]
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

#[cfg(any(feature = "alt_lower_letter", feature = "full"))]
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

#[cfg(any(feature = "alt_upper_letter", feature = "full"))]
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

#[cfg(any(feature = "alt_numbers", feature = "full"))]
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

#[cfg(any(feature = "alt_gr_lower_letter", feature = "full"))]
pub const ALT_GR_LOWER_LETTER: [([u8; BYTES], Keys); 27] = [
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

#[cfg(any(feature = "alt_gr_upper_letter", feature = "full"))]
pub const ALT_GR_UPPER_LETTER: [([u8; BYTES], Keys); 27] = [
    ([0xc3, 0xa6, 0x00], Keys::AltGr('A')),
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

#[cfg(any(feature = "alt_gr_numbers", feature = "full"))]
pub const ALT_GR_NUMBERS: [([u8; BYTES], Keys); 10] = [
    (CTRL_NUMBERS[0].0, Keys::AltGr('0')),
    (CTRL_NUMBERS[1].0, Keys::AltGr('1')),
    (CTRL_NUMBERS[2].0, Keys::AltGr('2')),
    (CTRL_NUMBERS[3].0, Keys::AltGr('3')),
    (CTRL_NUMBERS[4].0, Keys::AltGr('4')),
    (CTRL_NUMBERS[5].0, Keys::AltGr('5')),
    (CTRL_NUMBERS[6].0, Keys::AltGr('6')),
    (CTRL_NUMBERS[7].0, Keys::AltGr('7')),
    (CTRL_NUMBERS[8].0, Keys::AltGr('8')),
    (CTRL_NUMBERS[9].0, Keys::AltGr('9')),
];
