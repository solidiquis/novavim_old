use crate::cache::TextCache;
use crate::ctrls::Ctrl;
use crate::dev::Window;
use crate::models::{Key, Mode, SpecialKey, Response};

pub struct InsertCtrl<'a> {
    text_cache: &'a mut TextCache,
    window: &'a mut Window
}

impl Ctrl for InsertCtrl<'_> {
    fn forward_input_to_handler(&mut self, key: Key) -> Response {
        let response = match key {
            Key::Regular(k) => self.handle_regular_key(k),
            Key::Special(sk) => self.handle_special_key(sk),
        };

        response
    }

    fn handle_regular_key(&mut self, key_press: &str) -> Response {
        let (cursor_col, cursor_row) = self.window.get_cursor_position();
        let cursor_coords = (cursor_col, cursor_row);

        let current_line = self.text_cache.current_line(cursor_coords).to_string();

        if current_line.len() < cursor_row {
            self.text_cache.push_str(cursor_row, "")
        };

        if cursor_col < current_line.len() {
            let lslice = &current_line[0..(cursor_col-1)];
            let rslice = &current_line[(cursor_col-1)..current_line.len()];
            let text_to_cache = format!("{}{}{}", lslice, key_press, rslice);

            self.window.blurses.cursor_save_position();
            self.window.blurses.echo(key_press);
            self.window.blurses.echo(rslice);
            self.window.blurses.cursor_restore_position();
            self.window.blurses.cursor_right(1);

            self.text_cache.set_line(cursor_row, text_to_cache);

            return Response::Ok
        }

        self.text_cache.push_str(cursor_row, key_press);
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
    pub fn new(window: &'a mut Window, text_cache: &'a mut TextCache) -> Self {
        Self { text_cache, window }
    }

    fn handle_escape(&mut self) -> Response {
        self.window.blurses.cursor_left(1);
        Response::SwitchMode(Mode::Normal)
    }

    fn handle_backspace(&mut self) -> Response {
        let (cursor_col, cursor_row) = self.window.get_cursor_position();
        let current_line = self.text_cache.current_line((cursor_col, cursor_row));

        if current_line.len() == 0 || cursor_col == 1 {
            return Response::Ok
        }

        let lslice = &current_line[0..(cursor_col - 2)];
        let rslice = &current_line[(cursor_col - 1)..current_line.len()];

        self.window.blurses.erase_line();
        self.window.blurses.cursor_set_col(1);
        self.window.blurses.echo(lslice);
        self.window.blurses.cursor_save_position();
        self.window.blurses.echo(rslice);
        self.window.blurses.cursor_restore_position(); 

        self.text_cache.set_line(cursor_row, format!("{}{}", lslice, rslice));

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

