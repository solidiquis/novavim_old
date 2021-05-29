mod routing;
use routing::Mux;

mod utils;
use utils::stty;

mod dev;
use dev::{Cursor, Window, CursorNav};

mod blurses;
mod ctrls;
mod models;

fn main() {
    let window  = Window::default();
    let mut mux = Mux::new(&window);

    stty::unecho_stdin();
    stty::unbuffer_stdin();

    window.blurses.cursor_home();
    window.erase_screen();
    window.init_session();

    mux.watch_and_serve();
}
