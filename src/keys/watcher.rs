use std::io;
use std::io::Read;
use std::io::Write;

pub struct Watcher;

impl Watcher {
    pub fn new() -> Self {
        Watcher{}
    }

    pub fn watch_keypress(&self) {
        let stdout = io::stdout();

        let mut stdin = io::stdin();
        let mut buffer = [0;1];

        while buffer[0] != 113 {
            stdout.lock().flush().unwrap();
            stdin.read_exact(&mut buffer).unwrap();
            println!("You have hit: {:?}", buffer);
        }
    }
}
