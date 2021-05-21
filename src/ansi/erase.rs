use std::fmt::Display;
use crate::ansi::ansi_exec;

pub fn screen() {
    ansi_exec::exec(&format!("\x1b[{}J", 2))
}
