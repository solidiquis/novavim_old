mod ansi;
use ansi::erase;
use ansi::cursor;

mod keys;
use keys::watcher;

mod win;
use win::stty;
use win::winsize;

fn main() {
    //let (row, col) = winsize::get_winsize().unwrap_or_else(|err| {
        //panic!("{}", err)
    //});

    stty::unecho_stdin();
    stty::unbuffer_stdin();

    cursor::home();
    erase::screen();

    watcher::Watcher::new().watch_keypress();
}
