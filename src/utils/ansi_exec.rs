use std::io;
use std::io::Write;

pub fn exec(ansi_esc: &str) {
    print!("{}", ansi_esc);
    io::stdout().flush().unwrap()
}

pub fn bold(txt: &str) {
    exec(&format!("\x1b[1m{}\x1b[0m", txt))
}
