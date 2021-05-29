mod routing;
use routing::Mux;

mod utils;
use utils::stty;

mod dev;
use dev::Window;

mod blurses;
mod ctrls;
mod models;

fn main() {
    let mut window = Window::default();
    window.blurses.cursor_home();
    window.erase_screen();
    window.init_session();

    let mut mux = Mux::new(&mut window);

    stty::unecho_stdin();
    stty::unbuffer_stdin();

    mux.watch_and_serve();
}
