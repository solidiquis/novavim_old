mod routing;
use routing::mux;

mod utils;
use utils::stty;
//use utils::win;
use utils::erase;
use utils::cursor;

fn main() {
    //let (row, col) = winsize::get_winsize().unwrap_or_else(|err| {
        //panic!("{}", err)
    //});

    stty::unecho_stdin();
    stty::unbuffer_stdin();

    cursor::home();
    erase::screen();

    mux::watch_keypress();
}
