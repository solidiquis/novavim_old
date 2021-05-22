use std::io;
use std::io::Read;
use crate::utils::keypress::{KeyPress, SpecialKey, key_type};
use crate::dev::cursor::Cursor;
use crate::dev::shared::CursorNav;

pub fn watch_keypress(cursor: &Cursor) {
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
                    SpecialKey::Up     => cursor.up(1),
                    SpecialKey::Down   => cursor.down(1),
                    SpecialKey::Left   => cursor.left(1),
                    SpecialKey::Right  => cursor.right(1),
                    _ => ()
                },
        }

        // Re-init buffer
        buffer = [0;3];
    }
}


//pub struct Mux;

//impl Mux {
    //pub fn new() -> Self {
        //Mux{}
    //}

    //pub fn watch_keypress(&self) {
        //let mut stdin = io::stdin();
        //let mut buffer = [0;3];

        //loop {
            //stdin.read(&mut buffer).unwrap();

            //let key_type = key_type(&buffer);

            //match key_type {
                //KeyPress::Backspace => println!("Backspace"),
                //KeyPress::Regular   => println!("Regular"),
                //KeyPress::Return    => println!("Return"),

                //KeyPress::Special(sk) =>
                    //match sk {
                        //SpecialKey::Escape => println!("ESC!"),
                        //SpecialKey::Up     => cursor::up(1),
                        //SpecialKey::Down   => cursor::down(1),
                        //SpecialKey::Left   => cursor::left(1),
                        //SpecialKey::Right  => cursor::right(1),
                        //_ => ()
                    //},

                //_ => ()
            //}

            //// Re-init buffer
            //buffer = [0;3];
        //}
    //}

    //fn multiplex(&self, &)
//}
