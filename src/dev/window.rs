use crate::utils::{ansi_exec, win};
use crate::dev::{Window, CursorNav};

impl Default for Window {
    fn default() -> Self {
        let (col, row) = win::get_winsize().unwrap();

        let width  = col as u8;
        let height = row as u8;

        Self { width, height }
    }
}

impl CursorNav for Window {}

impl Window {
    pub fn clear(&self) {
        ansi_exec::exec(&format!("\x1b[{}J", 2))
    }

    pub fn init_session(&self) {
        self.print_mode("NORMAL");
        self.down(1);

        for _ in 1..self.height-1 {
            println!("~")
        }

        self.home();
    }

    pub fn print_mode(&self, mode: &str) {
        self.save_cursor_position();
        self.down(self.height);
        self.left(self.width);
        ansi_exec::erase_line();
        ansi_exec::bold(mode);
        self.restore_cursor_position()
    }

    pub fn get_width(&self) -> u8 {
        self.width
    }

    pub fn get_height(&self) -> u8 {
        self.height
    }
}
