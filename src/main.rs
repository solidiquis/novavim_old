mod routing;
use routing::Mux;

mod utils;
use utils::stty;

mod dev;
use dev::{Cursor, Window, CursorNav};

mod ctrls;
mod models;

fn main() {
    let cursor  = Cursor::default();
    let window  = Window::default();
    let mut mux = Mux::new(&window);

    stty::unecho_stdin();
    stty::unbuffer_stdin();

    cursor.home();
    window.clear();
    window.init_session();

    mux.watch_and_serve();
}
