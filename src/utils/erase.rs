use crate::utils::ansi_exec;

pub fn screen() {
    ansi_exec::exec(&format!("\x1b[{}J", 2))
}
