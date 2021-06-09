use crate::blurses::Blurses;

pub struct Window {
    pub blurses: Blurses
}

impl Default for Window {
    fn default() -> Self {
        let blurses = Blurses::default();

        Self { blurses }
    }
}

impl Window {
    pub fn erase_screen(&self) {
        self.blurses.erase_screen()
    }

    pub fn init_session(&mut self) {
        self.print_mode("NORMAL");
        self.blurses.cursor_down(1);

        for _ in 1..self.blurses.get_win_height() - 1 {
            println!("~")
        }

        self.blurses.cursor_home();
    }

    pub fn print_mode(&mut self, mode: &str) {
        self.blurses.cursor_save_position();
        self.blurses.cursor_down(self.blurses.get_win_height());
        self.blurses.cursor_left(self.blurses.get_win_width());
        self.blurses.erase_line();
        self.blurses.display_bold(mode);
        self.blurses.cursor_restore_position()
    }

    pub fn get_width(&self) -> usize {
        self.blurses.get_win_width()
    }

    pub fn get_height(&self) -> usize {
        self.blurses.get_win_height()
    }

    pub fn get_cursor_position(&self) -> (usize, usize) {
        self.blurses.get_cursor_position()
    }

    pub fn cursor_set_position(&mut self, cursor_coords: (usize, usize)) {
        self.blurses.cursor_set_position(cursor_coords.1, cursor_coords.0)
    }
}

