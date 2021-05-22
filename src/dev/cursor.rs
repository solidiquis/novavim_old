use crate::utils::ansi_exec;

pub struct Cursor {
    col: u8,
    row: u8
}

impl Default for Cursor {
    fn default() -> Self {
        Self {
            col: 1,
            row: 1,
        }
    }
}

impl Cursor {
    pub fn home(&self) {
        ansi_exec::exec(&"\x1b[H");
    }

    pub fn up(&self, n: i32) {
        ansi_exec::exec(&format!("\x1b[{}A", n));
    }

    pub fn down(&self, n: i32) {
        ansi_exec::exec(&format!("\x1b[{}B", n));
    }

    pub fn right(&self, n: i32) {
        ansi_exec::exec(&format!("\x1b[{}C", n));
    }

    pub fn left(&self, n: i32) {
        ansi_exec::exec(&format!("\x1b[{}D", n));
    }
}
