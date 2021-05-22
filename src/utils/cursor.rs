use crate::utils::ansi_exec;

pub fn home() {
    ansi_exec::exec(&"\x1b[H");
}

pub fn up(n: i32) {
    ansi_exec::exec(&format!("\x1b[{}A", n));
}

pub fn down(n: i32) {
    ansi_exec::exec(&format!("\x1b[{}B", n));
}

pub fn right(n: i32) {
    ansi_exec::exec(&format!("\x1b[{}C", n));
}

pub fn left(n: i32) {
    ansi_exec::exec(&format!("\x1b[{}D", n));
}
