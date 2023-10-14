extern crate libc;

use libc::{tcgetattr, tcsetattr, termios};
use std::io::{self, Read, Write};

pub const VTIME: usize = 5;
pub const VMIN: usize = 6;
pub const ICANON: u32 = 0x00000002;
pub const ECHO: u32 = 0x00000008;
pub const TCSANOW: i32 = 0;
pub const STDIN_FILENO: i32 = 0;

pub enum Keys {
    Up,
    Down,
    Left,
    Right,
    Enter,
    Space,
    Delete,
    Escape,
    Home,
    End,
    Tab,
    Backtab,
    Insert,
    Letter(char),
    F(u8),
    Ctrl(char),
    Null,
}

pub fn get_key_from_keyboard() -> Keys {
    let termios_enviroment: termios = setup_raw_mode().unwrap();
    io::stdout().flush().unwrap();
    let mut buffer: [u8; 3] = [0; 3];
    let mut key: Keys = Keys::Null;
    io::stdin().read(&mut buffer).unwrap();

    match buffer {
        [0x1B, 0x5B, byte] => {
            key = match byte {
                0x41 => Keys::Up,
                0x42 => Keys::Down,
                0x43 => Keys::Right,
                0x44 => Keys::Left,
                0x48 => Keys::Home,
                0x46 => Keys::End,
                0x09 => Keys::Tab,
                0x5A => Keys::Backtab,
                _ => key,
            };
        }
        [0x0A, _, _] => key = Keys::Enter,
        [0x20, _, _] => key = Keys::Space,
        [0x7F, _, _] => key = Keys::Delete,
        [0x1B, 0x4F, byte] => {
            key = match byte {
                0x50 => Keys::F(1),
                0x51 => Keys::F(2),
                0x52 => Keys::F(3),
                0x53 => Keys::F(5),
                _ => key,
            };
        }
        [0x35, 0x7E, _] => key = Keys::F(5),
        [0x37, 0x7E, _] => key = Keys::F(6),
        [0x38, 0x7E, _] => key = Keys::F(7),
        [0x39, 0x7E, _] => key = Keys::F(8),
        [0x30, 0x7E, _] => key = Keys::F(9),
        [0x31, 0x7E, _] => key = Keys::F(10),
        [0x32, 0x7E, _] => key = Keys::F(11),
        [0x34, 0x7E, _] => key = Keys::F(12),
        [0xB1, byte1, _] => {
            key = match byte1 {
                0xB1 => Keys::Letter('ñ'),
                0x91 => Keys::Letter('Ñ'),
                _ => key,
            };
        }
        [0x01, _, _] => key = Keys::Ctrl('a'),
        [0x02, _, _] => key = Keys::Ctrl('b'),
        [0x03, _, _] => key = Keys::Ctrl('c'),
        [0x04, _, _] => key = Keys::Ctrl('d'),
        [0x05, _, _] => key = Keys::Ctrl('e'),
        [0x06, _, _] => key = Keys::Ctrl('f'),
        [0x07, _, _] => key = Keys::Ctrl('g'),
        [0x08, _, _] => key = Keys::Ctrl('h'),
        [0x09, _, _] => key = Keys::Ctrl('g'),
        [0x0A, _, _] => key = Keys::Ctrl('j'),
        [0x0B, _, _] => key = Keys::Ctrl('k'),
        [0x0C, _, _] => key = Keys::Ctrl('l'),
        [0x0D, _, _] => key = Keys::Ctrl('m'),
        [0x0E, _, _] => key = Keys::Ctrl('n'),
        [0x3B, _, _] => key = Keys::Ctrl('ñ'),
        [0x0F, _, _] => key = Keys::Ctrl('o'),
        [0x10, _, _] => key = Keys::Ctrl('p'),
        [0x11, _, _] => key = Keys::Ctrl('q'),
        [0x12, _, _] => key = Keys::Ctrl('r'),
        [0x13, _, _] => key = Keys::Ctrl('s'),
        [0x14, _, _] => key = Keys::Ctrl('t'),
        [0x15, _, _] => key = Keys::Ctrl('u'),
        [0x16, _, _] => key = Keys::Ctrl('v'),
        [0x17, _, _] => key = Keys::Ctrl('w'),
        [0x18, _, _] => key = Keys::Ctrl('x'),
        [0x19, _, _] => key = Keys::Ctrl('y'),
        [0x1A, _, _] => key = Keys::Ctrl('z'),
        [0x61, _, _] => key = Keys::Letter('a'),
        [0x62, _, _] => key = Keys::Letter('b'),
        [0x63, _, _] => key = Keys::Letter('c'),
        [0x64, _, _] => key = Keys::Letter('d'),
        [0x65, _, _] => key = Keys::Letter('e'),
        [0x66, _, _] => key = Keys::Letter('f'),
        [0x67, _, _] => key = Keys::Letter('g'),
        [0x68, _, _] => key = Keys::Letter('h'),
        [0x69, _, _] => key = Keys::Letter('i'),
        [0x6A, _, _] => key = Keys::Letter('j'),
        [0x6B, _, _] => key = Keys::Letter('k'),
        [0x6C, _, _] => key = Keys::Letter('l'),
        [0x6D, _, _] => key = Keys::Letter('m'),
        [0x6E, _, _] => key = Keys::Letter('n'),
        [0x6F, _, _] => key = Keys::Letter('o'),
        [0x70, _, _] => key = Keys::Letter('p'),
        [0x71, _, _] => key = Keys::Letter('q'),
        [0x72, _, _] => key = Keys::Letter('r'),
        [0x73, _, _] => key = Keys::Letter('s'),
        [0x74, _, _] => key = Keys::Letter('t'),
        [0x75, _, _] => key = Keys::Letter('u'),
        [0x76, _, _] => key = Keys::Letter('v'),
        [0x77, _, _] => key = Keys::Letter('w'),
        [0x78, _, _] => key = Keys::Letter('x'),
        [0x79, _, _] => key = Keys::Letter('y'),
        [0x7A, _, _] => key = Keys::Letter('z'),
        [0x41, _, _] => key = Keys::Letter('A'),
        [0x42, _, _] => key = Keys::Letter('B'),
        [0x43, _, _] => key = Keys::Letter('C'),
        [0x44, _, _] => key = Keys::Letter('D'),
        [0x45, _, _] => key = Keys::Letter('E'),
        [0x46, _, _] => key = Keys::Letter('F'),
        [0x47, _, _] => key = Keys::Letter('G'),
        [0x48, _, _] => key = Keys::Letter('H'),
        [0x49, _, _] => key = Keys::Letter('I'),
        [0x4A, _, _] => key = Keys::Letter('J'),
        [0x4B, _, _] => key = Keys::Letter('K'),
        [0x4C, _, _] => key = Keys::Letter('L'),
        [0x4D, _, _] => key = Keys::Letter('M'),
        [0x4E, _, _] => key = Keys::Letter('N'),
        [0x4F, _, _] => key = Keys::Letter('O'),
        [0x50, _, _] => key = Keys::Letter('P'),
        [0x51, _, _] => key = Keys::Letter('Q'),
        [0x52, _, _] => key = Keys::Letter('R'),
        [0x53, _, _] => key = Keys::Letter('S'),
        [0x54, _, _] => key = Keys::Letter('T'),
        [0x55, _, _] => key = Keys::Letter('U'),
        [0x56, _, _] => key = Keys::Letter('V'),
        [0x57, _, _] => key = Keys::Letter('W'),
        [0x58, _, _] => key = Keys::Letter('X'),
        [0x59, _, _] => key = Keys::Letter('Y'),
        [0x5A, _, _] => key = Keys::Letter('Z'),
        _ => {}
    }
    io::stdout().flush().unwrap();
    restore_termios(termios_enviroment).unwrap();
    key
}

pub struct KeyboardIterator {
    buffer: [u8; 3],
}

impl KeyboardIterator {
    pub fn new() -> Self {
        KeyboardIterator { buffer: [0; 3] }
    }

    pub fn read_key(&mut self) -> Keys {
        get_key_from_keyboard()
    }
}

impl Iterator for KeyboardIterator {
    type Item = Keys;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.read_key())
    }
}

pub fn setup_raw_mode() -> io::Result<termios> {
    let mut termios: termios = unsafe { std::mem::zeroed() };
    if unsafe { tcgetattr(libc::STDIN_FILENO, &mut termios) } < 0 {
        return Err(io::Error::last_os_error());
    }
    let original_termios = termios.clone();
    termios.c_lflag &= !(ICANON | ECHO);
    termios.c_cc[VMIN] = 0;
    termios.c_cc[VTIME] = 1;
    if unsafe { tcsetattr(STDIN_FILENO, TCSANOW, &termios) } < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(original_termios)
}

pub fn restore_termios(original_termios: termios) -> io::Result<()> {
    if unsafe { tcsetattr(STDIN_FILENO, TCSANOW, &original_termios) } < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}
