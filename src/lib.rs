use std::{
    ffi::c_int,
    io::{self, Read, Write},
};

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
        [0x1B, 0x5B, 0x46] => key = Keys::End,
        [0x1B, 0x5B, 0x09] => key = Keys::Tab,
        [0x1B, 0x5B, 0x5a] => key = Keys::Backtab,
        [0x0A, 0x00, _] => key = Keys::Enter,
        [0x20, _, _] => key = Keys::Space,
        [0x7F, _, _] => key = Keys::Delete,
        [0x2b, _, _] => key = Keys::Plus,
        [0x2d, _, _] => key = Keys::Minus,
        [0x3d, _, _] => key = Keys::Equal,
        [0x2f, _, _] => key = Keys::Slash,
        [0x5c, _, _] => key = Keys::Backslash,
        [0x5e, _, _] => key = Keys::Power,
        [0x2a, _, _] => key = Keys::Asterisk,
        [0x2e, _, _] => key = Keys::Point,
        [0x2c, _, _] => key = Keys::Comma,
        [0x23, _, _] => key = Keys::Hashtag,
        [0x26, _, _] => key = Keys::Ampersand,
        [0x25, _, _] => key = Keys::Percent,
        [0x7c, _, _] => key = Keys::Pipe,
        [0x24, _, _] => key = Keys::Currency,
        [0x3a, _, _] => key = Keys::TwoPoints,
        [0x3b, 0x00, 0x00] => key = Keys::Semicolon,
        [0xc2, 0xbf, _] => key = Keys::OpenQuestionMark,
        [0x3f, _, _] => key = Keys::CloseQuestionMark,
        [0x5b, _, _] => key = Keys::OpenSquareBracket,
        [0x5d, _, _] => key = Keys::CloseSquareBracket,
        [0x7b, _, _] => key = Keys::OpenCurlyBrace,
        [0x7d, _, _] => key = Keys::CloseCurlyBrace,
        [0x28, _, _] => key = Keys::OpenParenthesis,
        [0x29, _, _] => key = Keys::CloseParenthesis,
        [0x3c, _, _] => key = Keys::LessThan,
        [0x3e, _, _] => key = Keys::GreaterThan,
        [0x27, _, _] => key = Keys::Apostrophe,
        [0x40, _, _] => key = Keys::At,
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
        [0x35, 0x7E, _] => key = Keys::F(5),
        [0x37, 0x7E, _] => key = Keys::F(6),
        [0x38, 0x7E, _] => key = Keys::F(7),
        [0x39, 0x7E, _] => key = Keys::F(8),
        [0x30, 0x7E, _] => key = Keys::F(9),
        [0x31, 0x7E, _] => key = Keys::F(10),
        [0x32, 0x7E, _] => key = Keys::F(11),
        [0x34, 0x7E, _] => key = Keys::F(12),
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
        [0xb1, 0xb1, _] => key = Keys::Letter('ñ'),
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
        [0xb1, 0x91, _] => key = Keys::Letter('Ñ'),
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
        [0x1b, 0x61, _] => key = Keys::Alt('a'),
        [0x1b, 0x62, _] => key = Keys::Alt('b'),
        [0x1b, 0x63, _] => key = Keys::Alt('c'),
        [0x1b, 0x64, _] => key = Keys::Alt('d'),
        [0x1b, 0x65, _] => key = Keys::Alt('e'),
        [0x1b, 0x66, _] => key = Keys::Alt('f'),
        [0x1b, 0x67, _] => key = Keys::Alt('g'),
        [0x1b, 0x68, _] => key = Keys::Alt('h'),
        [0x1b, 0x69, _] => key = Keys::Alt('i'),
        [0x1b, 0x6A, _] => key = Keys::Alt('j'),
        [0x1b, 0x6B, _] => key = Keys::Alt('k'),
        [0x1b, 0x6C, _] => key = Keys::Alt('l'),
        [0x1b, 0x6d, _] => key = Keys::Alt('m'),
        [0x1b, 0x6e, _] => key = Keys::Alt('n'),
        [0x1b, 0xc3, 0xb1] => key = Keys::Alt('ñ'),
        [0x1b, 0x6f, _] => key = Keys::Alt('o'),
        [0x1b, 0x70, _] => key = Keys::Alt('p'),
        [0x1b, 0x71, _] => key = Keys::Alt('q'),
        [0x1b, 0x72, _] => key = Keys::Alt('r'),
        [0x1b, 0x73, _] => key = Keys::Alt('s'),
        [0x1b, 0x74, _] => key = Keys::Alt('t'),
        [0x1b, 0x75, _] => key = Keys::Alt('u'),
        [0x1b, 0x76, _] => key = Keys::Alt('v'),
        [0x1b, 0x77, _] => key = Keys::Alt('w'),
        [0x1b, 0x78, _] => key = Keys::Alt('x'),
        [0x1b, 0x79, _] => key = Keys::Alt('y'),
        [0x1b, 0x7a, _] => key = Keys::Alt('z'),
        [0x1b, 0x41, _] => key = Keys::Alt('A'),
        [0x1b, 0x42, _] => key = Keys::Alt('B'),
        [0x1b, 0x43, _] => key = Keys::Alt('C'),
        [0x1b, 0x44, _] => key = Keys::Alt('D'),
        [0x1b, 0x45, _] => key = Keys::Alt('E'),
        [0x1b, 0x46, _] => key = Keys::Alt('F'),
        [0x1b, 0x47, _] => key = Keys::Alt('G'),
        [0x1b, 0x48, _] => key = Keys::Alt('H'),
        [0x1b, 0x49, _] => key = Keys::Alt('I'),
        [0x1b, 0x4a, _] => key = Keys::Alt('J'),
        [0x1b, 0x4b, _] => key = Keys::Alt('K'),
        [0x1b, 0x4c, _] => key = Keys::Alt('L'),
        [0x1b, 0x4d, _] => key = Keys::Alt('M'),
        [0x1b, 0x4e, _] => key = Keys::Alt('N'),
        [0x1b, 0xc3, 0x91] => key = Keys::Alt('Ñ'),
        [0x1b, 0x4f, 0x00] => key = Keys::Alt('O'),
        [0x1b, 0x50, _] => key = Keys::Alt('P'),
        [0x1b, 0x51, _] => key = Keys::Alt('Q'),
        [0x1b, 0x52, _] => key = Keys::Alt('R'),
        [0x1b, 0x53, _] => key = Keys::Alt('S'),
        [0x1b, 0x54, _] => key = Keys::Alt('T'),
        [0x1b, 0x55, _] => key = Keys::Alt('U'),
        [0x1b, 0x56, _] => key = Keys::Alt('V'),
        [0x1b, 0x57, _] => key = Keys::Alt('W'),
        [0x1b, 0x58, _] => key = Keys::Alt('X'),
        [0x1b, 0x59, _] => key = Keys::Alt('Y'),
        [0x1b, 0x5a, _] => key = Keys::Alt('Z'),
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
    pub fn tcsetattr(fd: c_int, optional_actions: c_int, termios_p: *const termios) -> c_int;
    pub fn tcgetattr(fd: c_int, termios: *mut termios) -> c_int;
}
