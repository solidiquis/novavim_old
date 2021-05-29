use crate::models::{Key, Mode, SpecialKey, Response};
use crate::ctrls::Ctrl;
use crate::dev::Window;

pub struct InsertCtrl<'a> {
    window: &'a Window
}

impl<'a> InsertCtrl<'a> {
    pub fn new(window: &'a Window) -> Self {
        Self { window }
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
        self.window.blurses.echo(key_press);
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
        self.window.blurses.cursor_left(1);
        Response::SwitchMode(Mode::Normal)
    }

    fn handle_backspace(&self) -> Response {
        self.window.blurses.backspace();
        Response::Ok
    }

    fn handle_return(&self) -> Response {
        // Refactor to once implement cache        
        self.window.blurses.cursor_down(1);
        self.window.blurses.cursor_left(self.window.get_width());
        self.window.blurses.echo(" ");
        self.window.blurses.cursor_left(1);
        Response::Ok
    }
}

