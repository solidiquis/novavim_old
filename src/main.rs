mod routing;
use routing::mux::Mux;

mod utils;
use utils::stty;

mod dev;
use dev::cursor::Cursor;
use dev::window::Window;
use dev::shared::CursorNav;

fn main() {
    let cursor  = Cursor::default();
    let window  = Window::default();
    let mut mux = Mux::default();

    stty::unecho_stdin();
    stty::unbuffer_stdin();

    cursor.home();
    window.clear();
    window.init_session();

    mux.watch_keypress();
}
