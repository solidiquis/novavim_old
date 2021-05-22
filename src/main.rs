mod routing;
use routing::mux;

mod utils;
use utils::stty;
//use utils::win;

mod dev;
use dev::cursor::Cursor;
use dev::window::Window;

fn main() {
    //let (row, col) = winsize::get_winsize().unwrap_or_else(|err| {
        //panic!("{}", err)
    //});

    let cursor = Cursor::default();
    let window = Window::default();

    stty::unecho_stdin();
    stty::unbuffer_stdin();

    cursor.home();
    window.clear();

    mux::watch_keypress(&cursor);
}
