use std::io;
use std::io::Read;
use crate::ctrls::normal::NormalCtrl;
use crate::ctrls::insert::InsertCtrl;
use crate::ctrls::Ctrl;
use crate::models::{Key, Mode, Response};
use crate::utils;
use crate::dev::{Window, Cursor};


pub struct Mux<'a> {
    pub mode: Mode,

    window: &'a Window,
    cursor: &'a Cursor,
}

impl<'a> Mux<'a> {
    pub fn new(window: &'a Window, cursor: &'a Cursor) -> Self {
        Self {
            mode: Mode::Normal,
            window,
            cursor,
        }
    }

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
        let ctrl = self.select_ctrl();

        let response = ctrl.forward_input_to_handler(key);

        match response {
            Response::SwitchMode(nm) => self.set_mode(nm),
            Response::Ok => ()
        }
    }

    fn set_mode(&mut self, new_mode: Mode) {
        self.window.print_mode(new_mode.stringify());
        self.mode = new_mode
    }

    fn select_ctrl(&self) -> Box<dyn Ctrl> {
        match self.mode {
            Mode::Normal => Box::new(NormalCtrl::new(self.cursor)),
            Mode::Insert => Box::new(InsertCtrl::new(self.cursor)),
            _ => Box::new(NormalCtrl::new(self.cursor))
        }
    }
}
