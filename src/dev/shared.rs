use crate::utils::ansi_exec;

pub trait CursorNav {
    fn home(&self) {
        ansi_exec::exec(&"\x1b[H")
    }

    fn up(&self, n: u8) {
        ansi_exec::exec(&format!("\x1b[{}A", n))
    }

    fn down(&self, n: u8) {
        ansi_exec::exec(&format!("\x1b[{}B", n))
    }

    fn right(&self, n: u8) {
        ansi_exec::exec(&format!("\x1b[{}C", n))
    }

    fn left(&self, n: u8) {
        ansi_exec::exec(&format!("\x1b[{}D", n))
    }

    fn save_cursor_position(&self) {
        ansi_exec::exec(&format!("\x1b[s"))
    }

    fn restore_cursor_position(&self) {
        ansi_exec::exec(&format!("\x1b[u"))
    }
}