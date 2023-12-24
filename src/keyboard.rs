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
