use crate::models::{Key, SpecialKey, Mode, Response};
use crate::ctrls::Ctrl;
use crate::dev::Cursor;

pub struct NormalCtrl<'a> {
    cursor: &'a Cursor
}

impl<'a> NormalCtrl<'a> {
    pub fn new(cursor: &'a Cursor) -> Self {
        Self { cursor }
    }
}

impl Ctrl for NormalCtrl<'_> {
    fn forward_input_to_handler(&self, key: Key) -> Response {
        let response = match key {
            Key::Regular(k) => self.handle_regular_key(k),
            Key::Special(sk) => Response::Ok,
            _ => Response::Ok,
        };

        response
    }

    fn handle_regular_key(&self, key_press: &str) -> Response {
        match key_press {
            "i" => Response::SwitchMode(Mode::Insert),
            _ => Response::Ok
        }
    }

    fn handle_special_key(&self, key_press: SpecialKey) -> Response {
        Response::Ok
    }
}
