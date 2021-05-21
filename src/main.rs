mod ansi;
use ansi::erase;
use ansi::cursor;

mod keys;
use keys::watcher;

mod win;
use win::stty;

fn main() {
    stty::unecho_stdin();
    stty::unbuffer_stdin();

    cursor::home();
    erase::screen();

    watcher::Watcher::new().watch_keypress();
}
