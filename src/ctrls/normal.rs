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
            "i" | "I" | "a" | "A" | "o" | "O" => self.insert_mode(key_press),
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

    pub fn insert_mode(&mut self, key_press: &str) -> Response {
        match key_press {
            "a" => self.window.blurses.cursor_right(1),

            "A" => {
                let cursor_coords = self.window.get_cursor_position();
                let line_len = self.text_cache.current_line(cursor_coords).len();

                self.window.blurses.cursor_set_col(line_len + 1)
            },

            "i" | _ => ()
        }

        Response::SwitchMode(Mode::Insert)
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
        let (cursor_col, cursor_row) = self.window.get_cursor_position();
        let cursor_coords = (cursor_col, cursor_row);
        let current_line = self.text_cache.current_line(cursor_coords);

        match key_press {
            "l" => {
                if cursor_col + 1 <= current_line.len() {
                    self.window.blurses.cursor_right(1)    
                }
            },

            "E" => {
                // Todo: Add logic to traverse all lines.
                if current_line.len() == 0 || cursor_col == current_line.len() {
                    return
                }

                let next_char;
                let mut whitespace_occurrence = 1;

                if let Ok(ch) = self.text_cache.compute_next_char(cursor_coords) {
                    next_char = ch
                } else {
                    return
                }

                if next_char == ' ' {
                    whitespace_occurrence += 1;    
                }                 

                let ch_col;
                let ch_row;

                let coords = self.text_cache.next_nth_occurrence_of_char(&' ', whitespace_occurrence, cursor_coords);

                match coords {
                    Ok(p) => {
                        if p.1 > cursor_col {
                            ch_col = current_line.len();
                            ch_row = p.1;
                        } else {
                            ch_col = p.0;
                            ch_row = p.1;
                        }
                    },
                    _ => {
                        ch_row = self.text_cache.line_count();
                        ch_col = self.text_cache.current_line(cursor_coords).len();
                    }
                }

                self.window.blurses.cursor_set_position(ch_row, ch_col)
            },

            "e" => {
                let (cursor_col, cursor_row) = self.window.get_cursor_position();
                let cursor_coords = (cursor_col, cursor_row);

                if self.text_cache.end_of_text(cursor_coords) { return };

                let current_char = self.text_cache
                    .compute_current_char(cursor_coords)
                    .unwrap();

                let next_char = self.text_cache
                    .compute_next_char(cursor_coords)
                    .unwrap();

                let mut new_cursor_position = self.text_cache.last_char_position();

                if self.text_cache.is_word_char(&current_char) {

                    if self.text_cache.is_word_char(&next_char) {
                        let res = self.text_cache.re_first_match_position(TextCache::NON_WORDCHAR_BOUNDARY, 0, cursor_coords);

                        match res {
                            Ok(c) => new_cursor_position = c,
                            _ => ()
                        }

                    } else if next_char == ' ' {
                        let dist_to_whitespace = 
                            if let Ok(d) = self.text_cache.distance_to_pattern_from_cursor(TextCache::WHITESPACE, 2, cursor_coords) {
                                d 
                            } else {
                                f64::INFINITY
                            };

                        let dist_to_nonword_char =
                            if let Ok(d) = self.text_cache.distance_to_pattern_from_cursor(TextCache::NON_WORDCHAR_WS, 2, cursor_coords) {
                                d
                            } else {
                                f64::INFINITY
                            };

                        let pattern = 
                            if dist_to_whitespace > dist_to_nonword_char {
                                TextCache::NON_WORDCHAR
                            } else if dist_to_nonword_char > dist_to_whitespace {
                                TextCache::WHITESPACE_BOUNDARY
                            } else {
                                ""
                            };

                        if pattern != "" {
                            let res = self.text_cache.re_first_match_position(pattern, 2, cursor_coords);
                            match res {
                                Ok(c) => new_cursor_position = c,
                                _ => ()
                            }
                        }

                    }
                }

                self.window.cursor_set_position(new_cursor_position)

            },

            _ => ()
        }
    }
}

