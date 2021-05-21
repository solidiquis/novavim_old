use std::io;
use std::io::{Read, Write};

use crate::keys::keypress::{KeyPress, SpecialKey, key_type};

use crate::ansi::cursor;

pub struct Watcher;

impl Watcher {
    pub fn new() -> Self {
        Watcher{}
    }

    pub fn watch_keypress(&self) {
        let mut stdin = io::stdin();
        let mut buffer = [0;3];

        loop {
            stdin.read(&mut buffer).unwrap();

            let key_type = key_type(&buffer);

            match key_type {
                KeyPress::Backspace => println!("Backspace"),
                KeyPress::Regular   => println!("Regular"),
                KeyPress::Return    => println!("Return"),

                KeyPress::Special(sk) =>
                    match sk {
                        SpecialKey::Escape => println!("ESC!"),
                        SpecialKey::Up     => cursor::up(1),
                        SpecialKey::Down   => cursor::down(1),
                        SpecialKey::Left   => cursor::left(1),
                        SpecialKey::Right  => cursor::right(1),
                        _ => ()
                    },

                _ => ()
            }

            // Re-init buffer
            buffer = [0;3];
        }
    }
}
