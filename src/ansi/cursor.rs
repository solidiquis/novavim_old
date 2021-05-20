pub fn home() {
    print!("\x1b[H")
}

pub fn up(n: i32) {
    print!("\x1b[{}A", n)
}

pub fn down(n: i32) {
    print!("\x1b[{}B", n)
}

pub fn right(n: i32) {
    print!("\x1b[{}C", n)
}

pub fn left(n: i32) {
    print!("\x1b[{}D", n)
}
