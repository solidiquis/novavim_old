// https://unix.superglobalmegacorp.com/Net2/newsrc/sys/ioctl.h.html

extern crate libc;

use libc::{c_ushort, STDOUT_FILENO, TIOCGWINSZ};
use libc::ioctl;

#[repr(C)]
struct winsize {
    ws_row: c_ushort,
    ws_col: c_ushort,
    ws_xpixel: c_ushort,
    ws_ypixel: c_ushort
}

fn get_winsize() -> Result<(isize, isize), &'static str> {
    let w = winsize { ws_row: 0, ws_col: 0, ws_xpixel: 0, ws_ypixel: 0 };
    let r = unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &w) };

    match r {
        0 => Ok((w.ws_col as isize, w.ws_row as isize)),
        _ => Err("Error requesting window size.")
    }
}

use crate::flush_print;
use std::fmt::Display;

pub struct Blurses {
    win_width: usize,
    win_height: usize,
    cursor_col: usize,
    cursor_row: usize,
    saved_cursor_col: usize,
    saved_cursor_row: usize, 
}

impl Default for Blurses {
    fn default() -> Self {
        let (col, row) = get_winsize().unwrap_or_else(|err| {
            panic!("Exiting program with err: {}", err)
        });

        let win_width  = col as usize;
        let win_height = row as usize;
        let cursor_col = 1;
        let cursor_row = 1;
        let saved_cursor_col = 1;
        let saved_cursor_row = 1;

        Self {
            win_width,
            win_height,
            cursor_col,
            cursor_row,
            saved_cursor_col,
            saved_cursor_row
        }
    }
}

impl Blurses {
    pub fn fansi<T, U>(&self, x: T, y: U) -> String
        where T: Display,
              U: Display,
    {
        format!("\x1b[{}{}", x, y)
    }

    pub fn echo(&mut self, txt: &str) {
        // Really have to be careful with null chars because of the way
        // NovaVim converts the incoming bytes into a utf-8 string.

        // null char i.e. \u{0) are counted when computing length so any
        // udb with regard to cursor position may be caused by them.

        // Consider trimming \u{0} or even using regex if it becomes a problem.
        // Maybe even write a test lel.
        self.inc_cursor_col(txt.len() as usize);
        flush_print!("{}", txt)
    }

    pub fn backspace(&mut self) {
        self.dec_cursor_col(1);
        flush_print!("\x08 \x08")    
    }

    pub fn cursor_home(&mut self) {
        self.cursor_col = 1;
        self.cursor_row = 1;
        flush_print!("{}", self.fansi("H", ""))
    }

    pub fn cursor_up(&mut self, n: usize) {
        self.inc_cursor_row(n);
        flush_print!("{}", self.fansi(n, "A"))
    }

    pub fn cursor_down(&mut self, n: usize) {
        self.dec_cursor_row(n);
        flush_print!("{}", self.fansi(n, "B"))
    }

    pub fn cursor_right(&mut self, n: usize) {
        self.inc_cursor_col(n);
        flush_print!("{}", self.fansi(n, "C"))
    }

    pub fn cursor_left(&mut self, n: usize) {
        self.dec_cursor_col(n);
        flush_print!("{}", self.fansi(n, "D"))
    }

    pub fn cursor_set_col(&mut self, col: usize) {
        let pos = format!("{};{}", self.get_cursor_row(), col);
        self.cursor_col = col;
        flush_print!("{}", self.fansi(&pos, "H"))
    }

    pub fn cursor_set_row(&mut self, row: usize) {
        let pos = format!("{};{}", row, self.get_cursor_col());
        self.cursor_row = row;
        flush_print!("{}", self.fansi(&pos, "H"))
    }

    pub fn cursor_set_position(&mut self, row: usize, col: usize) {
        let pos = format!("{};{}", row, col);
        self.cursor_row = row;
        self.cursor_col = col;
        flush_print!("{}", self.fansi(&pos, "H"))
    }

    pub fn cursor_save_position(&mut self) {
        self.saved_cursor_col = self.cursor_col;
        self.saved_cursor_row = self.cursor_row;
        flush_print!("{}", self.fansi("", "s"))
    }

    pub fn cursor_restore_position(&mut self) {
        self.cursor_col = self.saved_cursor_col;
        self.cursor_row = self.saved_cursor_row;
        flush_print!("{}", self.fansi("", "u"))
    }

    pub fn display_bold(&mut self, txt: &str) {
        let ftxt = format!("{}{}", self.fansi("1m", txt), self.fansi("0m", ""));
        self.echo(&ftxt)
    }

    pub fn erase_screen(&self) {
        flush_print!("{}", self.fansi("2J", ""))
    }

    pub fn erase_line(&self) {
        flush_print!("{}", self.fansi("2K", ""))
    }

    pub fn get_win_width(&self) -> usize {
        self.win_width
    }

    pub fn get_win_height(&self) -> usize {
        self.win_height
    }

    pub fn get_win_dimensions(&self) -> (usize, usize) {
        (self.win_width, self.win_height)
    }

    pub fn get_cursor_col(&self) -> usize {
        self.cursor_col
    }

    pub fn get_cursor_row(&self) -> usize {
        self.cursor_row
    }

    pub fn get_cursor_position(&self) -> (usize, usize) {
        (self.cursor_col, self.cursor_row)
    }
 
    fn inc_cursor_col(&mut self, n: usize) {      
        self.cursor_col += n
    }
 
    fn dec_cursor_col(&mut self, n: usize) {
        if (self.cursor_col as i16) - (n as i16) < 1 {
            self.cursor_col = 1;
            return
        }

        self.cursor_col -= n
    }
 
    fn inc_cursor_row(&mut self, n: usize) {
        self.cursor_row += n                
    }
 
    fn dec_cursor_row(&mut self, n: usize) {
        if (self.cursor_row as i16) - (n as i16) < 1 {
            self.cursor_row = 1;
            return
        }

        self.cursor_row -= n
    }
}
