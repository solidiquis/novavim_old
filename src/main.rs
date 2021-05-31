mod routing;
use routing::Mux;

mod utils;
use utils::stty;

mod blurses;
mod cache;
mod ctrls;
mod dev;
mod models;

fn main() {
    let mut mux = Mux::default();
    mux.window.blurses.cursor_home();
    mux.window.erase_screen();
    mux.window.init_session();

    stty::unecho_stdin();
    stty::unbuffer_stdin();

    mux.watch_and_serve();
}
