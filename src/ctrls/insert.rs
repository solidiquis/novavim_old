use crate::models::{Key, Mode, SpecialKey, Response};
use crate::ctrls::Ctrl;
use crate::utils::ansi_exec::{echo, backspace};
use crate::dev::{Cursor, CursorNav, Window};

pub struct InsertCtrl<'a> {
    cursor: &'a Cursor,
    window: &'a Window
}

impl<'a> InsertCtrl<'a> {
    pub fn new(cursor: &'a Cursor, window: &'a Window) -> Self {
        Self { cursor, window }
    }
}

impl Ctrl for InsertCtrl<'_> {
    fn forward_input_to_handler(&self, key: Key) -> Response {
        let response = match key {
            Key::Regular(k) => self.handle_regular_key(k),
            Key::Special(sk) => self.handle_special_key(sk),
            _ => Response::Ok,
        };

        response
    }

    fn handle_regular_key(&self, key_press: &str) -> Response {
        echo(key_press);
        Response::Ok
    }

    fn handle_special_key(&self, key_press: SpecialKey) -> Response {
        match key_press {
            SpecialKey::Escape => self.handle_escape(),
            SpecialKey::Backspace => self.handle_backspace(),
            SpecialKey::Return => self.handle_return(),

            _ => Response::Ok
        }
    }
}

impl<'a> InsertCtrl<'a> {
    fn handle_escape(&self) -> Response {
        self.cursor.left(1);
        Response::SwitchMode(Mode::Normal)
    }

    fn handle_backspace(&self) -> Response {
        backspace();
        Response::Ok
    }

    fn handle_return(&self) -> Response {
        // Refactor to once implement cache        
        self.cursor.down(1);
        self.cursor.left(self.window.get_width());
        echo(" ");
        self.cursor.left(1);
        Response::Ok
    }
}

