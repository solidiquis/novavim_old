use crate::models::{Key, Mode, SpecialKey, Response};
use crate::ctrls::Ctrl;
use crate::flush_print;

pub struct InsertCtrl {}

impl Default for InsertCtrl {
    fn default() -> Self {
        Self{}
    }
}

impl Ctrl for InsertCtrl {
    fn forward_input_to_handler(&self, key: Key) -> Response {
        let response = match key {
            Key::Regular(k) => self.handle_regular_key(k),
            Key::Special(sk) => self.handle_special_key(sk),
            _ => Response::Ok,
        };

        response
    }

    fn handle_regular_key(&self, key_press: &str) -> Response {
        flush_print!("{}", key_press);
        Response::Ok
    }

    fn handle_special_key(&self, key_press: SpecialKey) -> Response {
        match key_press {
            SpecialKey::Escape => Response::SwitchMode(Mode::Normal),
            _ => Response::Ok
        }
    }
}
