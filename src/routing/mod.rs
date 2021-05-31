use std::io;
use std::io::Read;

use crate::cache::TextCache;
use crate::ctrls::normal::NormalCtrl;
use crate::ctrls::insert::InsertCtrl;
use crate::ctrls::Ctrl;
use crate::dev::Window;
use crate::models::{Key, Mode, Response};
use crate::utils;

pub struct Mux {
    pub mode: Mode,
    pub text_cache: TextCache,
    pub window: Window
}

impl Default for Mux {
    fn default() -> Self {
        let text_cache = TextCache::default();
        let window = Window::default();

        Self {
            mode: Mode::Normal,
            text_cache,
            window
        }
    }

}

impl Mux {
    pub fn watch_and_serve(&mut self) {
        let mut stdin = io::stdin();
        let mut buffer = [0; 3];
        let mut key: Key;

        loop {
            stdin.read(&mut buffer).unwrap();
            key = utils::keys::key_type(&buffer);
            self.multiplex(key);
            buffer = [0; 3]
        }
    }

    fn multiplex(&mut self, key: Key) {
        let response;

        {
            let mut ctrl = self.select_ctrl();
            response = ctrl.forward_input_to_handler(key);
        }

        match response {
            Response::SwitchMode(nm) => self.set_mode(nm),
            Response::Ok => ()
        }
    }

    fn set_mode(&mut self, new_mode: Mode) {
        self.window.print_mode(new_mode.stringify());
        self.mode = new_mode
    }

    fn select_ctrl(&mut self) -> Box<dyn Ctrl + '_> {
        match self.mode {
            Mode::Normal => Box::new(NormalCtrl::new(&mut self.window, &mut self.text_cache,)),
            Mode::Insert => Box::new(InsertCtrl::new(&mut self.window, &mut self.text_cache)),
            _ => Box::new(NormalCtrl::new(&mut self.window, &mut self.text_cache))
        }
    }
}
