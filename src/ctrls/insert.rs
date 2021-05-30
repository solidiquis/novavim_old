use crate::cache::TextCache;
use crate::ctrls::Ctrl;
use crate::dev::Window;
use crate::models::{Key, Mode, SpecialKey, Response};

pub struct InsertCtrl<'a> {
    text_cache: &'a mut TextCache,
    window: &'a mut Window
}

impl<'a> InsertCtrl<'a> {
    pub fn new(window: &'a mut Window, text_cache: &'a mut TextCache) -> Self {
        Self { text_cache, window }
    }
}

impl Ctrl for InsertCtrl<'_> {
    fn forward_input_to_handler(&mut self, key: Key) -> Response {
        let response = match key {
            Key::Regular(k) => self.handle_regular_key(k),
            Key::Special(sk) => self.handle_special_key(sk),
            _ => Response::Ok,
        };

        response
    }

    fn handle_regular_key(&mut self, key_press: &str) -> Response {
        let (cu_col, ln_no) = self.window.blurses.get_cursor_position();

        if (self.text_cache.text.len() as u16) < ln_no {
            self.text_cache.text.push("".to_string())
        };

        let current_line = &self.text_cache.text[(ln_no - 1) as usize];
        
        if cu_col < (current_line.len() as u16) {
            let lslice = &current_line[0..(cu_col as usize)];
            let rslice = &current_line[(cu_col as usize)..current_line.len()];
            let text_to_print = format!("{}{}", key_press, rslice);
            let text_to_cache = format!("{}{}{}", lslice, key_press, rslice);

            self.window.blurses.cursor_save_position();
            self.window.blurses.echo(&text_to_print);
            self.window.blurses.cursor_restore_position();
            self.window.blurses.cursor_right(1);

            self.text_cache.text[(ln_no - 1) as usize] = text_to_cache;

            return Response::Ok
        }

        self.text_cache.text[(ln_no - 1) as usize].push_str(key_press);
        self.window.blurses.echo(key_press);

        Response::Ok
    }

    fn handle_special_key(&mut self, key_press: SpecialKey) -> Response {
        match key_press {
            SpecialKey::Escape => self.handle_escape(),
            SpecialKey::Backspace => self.handle_backspace(),
            SpecialKey::Return => self.handle_return(),

            _ => Response::Ok
        }
    }
}

impl<'a> InsertCtrl<'a> {
    fn handle_escape(&mut self) -> Response {
        self.window.blurses.cursor_left(1);
        Response::SwitchMode(Mode::Normal)
    }

    fn handle_backspace(&mut self) -> Response {
        self.window.blurses.backspace();
        Response::Ok
    }

    fn handle_return(&mut self) -> Response {
        // Refactor to once implement cache        
        self.window.blurses.cursor_down(1);
        self.window.blurses.cursor_left(self.window.get_width());
        self.window.blurses.echo(" ");
        self.window.blurses.cursor_left(1);
        Response::Ok
    }
}

