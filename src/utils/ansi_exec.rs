use crate::flush_print;

pub fn exec(ansi_esc: &str) {
    flush_print!("{}", ansi_esc);
}

// Alias for exec
pub fn echo(txt: &str) {
    exec(txt)
}

pub fn backspace() {
    flush_print!("\x08 \x08")
}

pub fn bold(txt: &str) {
    exec(&format!("\x1b[1m{}\x1b[0m", txt))
}

pub fn erase_line() {
    exec("\x1b[2K")
}

