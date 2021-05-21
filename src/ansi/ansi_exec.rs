use std::io;
use std::io::Write;

pub fn exec(ansi_esc: &str) {
    print!("{}", ansi_esc);
    io::stdout().flush().unwrap()
}
