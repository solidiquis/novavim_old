use crate::utils::{ansi_exec, win};

pub struct Window {
    width: u8,
    height: u8
}

impl Default for Window {
    fn default() -> Self {
        let (col, row) = win::get_winsize().unwrap();

        let width  = col as u8;
        let height = row as u8;

        Self { width, height }
    }
}

impl Window {
    pub fn clear(&self) {
        ansi_exec::exec(&format!("\x1b[{}J", 2))
    }
}
