use std::char;

use crate::models::{Key, SpecialKey, Mode, Response};
use crate::cache::TextCache;
use crate::ctrls::Ctrl;
use crate::dev::Window;

pub struct NormalCtrl<'a> {
    text_cache: &'a mut TextCache,
    window: &'a mut Window
}

impl Ctrl for NormalCtrl<'_> {
    fn forward_input_to_handler(&mut self, key: Key) -> Response {
        let response = match key {
            Key::Regular(k) => self.handle_regular_key(k),
            Key::Special(sk) => Response::Ok,
        };

        response
    }

    fn handle_regular_key(&mut self, key_press: &str) -> Response {
        match key_press {
            "i" => Response::SwitchMode(Mode::Insert),
             _ => self.handle_navigation(key_press),
        }
    }

    fn handle_special_key(&mut self, key_press: SpecialKey) -> Response {
        Response::Ok
    }
}

impl<'a> NormalCtrl<'a> {
    pub fn new(window: &'a mut Window, text_cache: &'a mut TextCache) -> Self {
        Self { text_cache, window }
    }

    pub fn handle_navigation(&mut self, key_press: &str) -> Response {
        match key_press {
            "h" | "b" | "B" | "^" | "0" => self.left_navigation(key_press),
            "j" => self.window.blurses.cursor_down(1),
            "k" => self.window.blurses.cursor_up(1),
            "l" | "e" | "E" => self.right_navigation(key_press),
            _ => ()
        }

        Response::Ok
    }

    fn left_navigation(&mut self, key_press: &str) {
        match key_press {
            "h" => self.window.blurses.cursor_left(1),
            _ => ()
        }
    }

    fn right_navigation(&mut self, key_press: &str) {
        let (cursor_col, cursor_row) = self.window.blurses.get_cursor_position();
        let current_line = &self.text_cache.text[(cursor_row as usize) - 1];
        let line_len = current_line.len() as u16;

        match key_press {
            "l" => {
                if cursor_col + 1 <= line_len {
                    self.window.blurses.cursor_right(1)    
                }
            },

            "e" => {
                if current_line.len() == 0 || cursor_col == line_len {
                    return
                }

                let mut last_alphanumeric_index = 0;
                let current_char = current_line.chars().nth((cursor_col - 1) as usize).unwrap();
                let next_char = current_line.chars().nth((cursor_col) as usize).unwrap();
                let whitespace = char::from_u32(0x0020).unwrap();

                let start;

                // Order matters.
                if current_char == whitespace {
                    start = cursor_col as usize
                } else if next_char == whitespace {
                    start = (cursor_col as usize) + 1
                } else if !next_char.is_alphanumeric() {
                    self.window.blurses.cursor_right(1);
                    return
                } else if !current_char.is_alphanumeric() {
                    start = (cursor_col as usize) + 1
                } else {
                    start = (cursor_col as usize) - 1
                }

                for i in start..current_line.len() {
                    let ch = current_line.chars().nth(i).unwrap();

                    if i == current_line.len() - 1 {
                        last_alphanumeric_index = i;
                        break;
                    }

                    if !ch.is_alphanumeric() {
                        last_alphanumeric_index = i - 1;
                        break;
                    }
                }

                self.window.blurses.cursor_set_col((last_alphanumeric_index + 1) as u16)
            }

            _ => ()
        }
    }
}

