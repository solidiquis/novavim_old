use std::io;
use std::io::Read;

pub enum Mode {
    Normal,
    Insert,
    Visual
}

pub enum KeyPress {
    Regular,
    Backspace,
    Return,
    Special(SpecialKey),
}

pub enum SpecialKey {
    Up,
    Down,
    Left,
    Right,
    Escape,
    None
}

pub fn key_type(bytes: &[u8; 3]) -> KeyPress {
    match bytes[0] {
        127 => KeyPress::Backspace,
        27  => KeyPress::Special(determine_special_key(bytes)),
        10  => KeyPress::Return,
        _   => KeyPress::Regular
    }
}

fn determine_special_key(bytes: &[u8]) -> SpecialKey {
    match bytes[2] {
        0  => SpecialKey::Escape,
        65 => SpecialKey::Up,
        66 => SpecialKey::Down,
        67 => SpecialKey::Right,
        68 => SpecialKey::Left,
        _  => SpecialKey::None,
    }
}

pub struct Mux {
    pub mode: Mode,
    buffer: [u8; 3]
}

impl Default for Mux {
    fn default() -> Self {
        Self {
            mode: Mode::Normal,
            buffer: [0; 3]
        }
    }
}

impl Mux {
    pub fn watch_keypress(&mut self) {
        let mut stdin = io::stdin();
        let mut key: KeyPress;

        loop {
            stdin.read(&mut self.buffer).unwrap();
            key = key_type(&self.buffer);
            self.multiplex(&key);
            self.reinit_buffer()
        }
    }

    fn multiplex(&self, key: &KeyPress) {
        match key {
            KeyPress::Backspace => println!("Backspace"),
            KeyPress::Regular   => println!("Regular"),
            KeyPress::Return    => println!("Return"),

            KeyPress::Special(sk) =>
                match sk {
                    SpecialKey::Escape => println!("ESC!"),
                    SpecialKey::Up     => println!("Up"),
                    SpecialKey::Down   => println!("Down"),
                    SpecialKey::Left   => println!("Left"),
                    SpecialKey::Right  => println!("Right"),
                    _ => ()
                },
        }
    }

    fn reinit_buffer(&mut self) {
        self.buffer = [0; 3]
    }
}
