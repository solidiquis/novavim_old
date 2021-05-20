mod ansi;
use ansi::erase;
use ansi::cursor;

mod keys;
use keys::watcher;

mod win;
use win::stty;

fn main() {
    erase::screen();
    cursor::home();

    let watcher = watcher::Watcher::new();

    stty::unecho_stdin();
    stty::unbuffer_stdin();

    watcher.watch_keypress()
}
