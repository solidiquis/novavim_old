use crate::models::{Key, SpecialKey, Mode, Response};
use crate::ctrls::Ctrl;
use crate::dev::Window;

pub struct NormalCtrl<'a> {
    window: &'a mut Window
}

impl<'a> NormalCtrl<'a> {
    pub fn new(window: &'a mut Window) -> Self {
        Self {  window }
    }
}

impl Ctrl for NormalCtrl<'_> {
    fn forward_input_to_handler(&mut self, key: Key) -> Response {
        let response = match key {
            Key::Regular(k) => self.handle_regular_key(k),
            Key::Special(sk) => Response::Ok,
            _ => Response::Ok,
        };

        response
    }

    fn handle_regular_key(&mut self, key_press: &str) -> Response {
        match key_press {
            "i" => Response::SwitchMode(Mode::Insert),
            _ => Response::Ok
        }
    }

    fn handle_special_key(&mut self, key_press: SpecialKey) -> Response {
        Response::Ok
    }
}
