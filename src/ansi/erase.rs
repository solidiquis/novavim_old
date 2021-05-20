use std::fmt::Display;

pub fn screen() {
    screen_esc(2)
}

fn screen_esc<T: Display>(cmd: T) {
    print!("\x1b[{}J", cmd)    
}
