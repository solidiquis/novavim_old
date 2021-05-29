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
    win_width: u8,
    win_height: u8,
    cursor_col: u8,
    cursor_row: u8,
}

impl Default for Blurses {
    fn default() -> Self {
        let (col, row) = get_winsize().unwrap_or_else(|err| {
            panic!("Exiting program with err: {}", err)
        });

        let win_width  = col as u8;
        let win_height = row as u8;
        let cursor_col = 1;
        let cursor_row = 1;

        Self {
            win_width,
            win_height,
            cursor_col,
            cursor_row
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

    pub fn echo(&self, txt: &str) {
        flush_print!("{}", txt)
    }

    pub fn backspace(&self) {
        flush_print!("\x08 \x08")    
    }

    pub fn cursor_home(&self) {
        flush_print!("{}", self.fansi("H", ""))
    }

    pub fn cursor_up(&self, n: u8) {
        flush_print!("{}", self.fansi(n, "A"))
    }

    pub fn cursor_down(&self, n: u8) {
        flush_print!("{}", self.fansi(n, "B"))
    }

    pub fn cursor_right(&self, n: u8) {
        flush_print!("{}", self.fansi(n, "C"))
    }

    pub fn cursor_left(&self, n: u8) {
        flush_print!("{}", self.fansi(n, "D"))
    }

    pub fn cursor_save_position(&self) {
        flush_print!("{}", self.fansi("", "s"))
    }

    pub fn cursor_restore_position(&self) {
        flush_print!("{}", self.fansi("", "u"))
    }

    pub fn display_bold(&self, txt: &str) {
        let ftxt = format!("{}{}", self.fansi("1m", txt), self.fansi("0m", ""));
        flush_print!("{}", ftxt)
    }

    pub fn erase_screen(&self) {
        flush_print!("{}", self.fansi("2J", ""))
    }

    pub fn erase_line(&self) {
        flush_print!("{}", self.fansi("2K", ""))
    }

    pub fn get_win_width(&self) -> u8 {
        self.win_width
    }

    pub fn get_win_height(&self) -> u8 {
        self.win_height
    }

   pub fn get_cursor_col(&self) -> u8 {
       self.cursor_col
   }

   pub fn get_cursor_row(&self) -> u8 {
       self.cursor_row
   }
}
