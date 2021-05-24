use crate::models::{Key, SpecialKey, Mode, Response};
use crate::ctrls::Ctrl;
use crate::flush_print;

pub struct NormalCtrl {}

impl Default for NormalCtrl {
    fn default() -> Self {
        Self{}
    }
}

impl Ctrl for NormalCtrl {
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
