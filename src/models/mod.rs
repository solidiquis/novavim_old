pub enum Response {
    SwitchMode(Mode),
    Ok        
}

pub enum Mode {
    Normal,
    Insert,
    Visual
}

impl Mode {
    pub fn stringify(&self) -> &str {
        match *self {
            Mode::Normal => "NORMAL",
            Mode::Insert => "INSERT",
            Mode::Visual => "VISUAL",
        }
    }
}

pub enum Key<'a> {
    Regular(&'a str),
    Special(SpecialKey),
    Return,
}

#[derive(Debug)]
pub enum SpecialKey {
    Up,
    Down,
    Left,
    Right,
    Escape,
    Backspace,
    None
}
