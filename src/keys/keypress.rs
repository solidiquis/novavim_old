pub enum KeyPress {
    Regular,
    Backspace,
    Return,
    Special(SpecialKey),
}

pub enum SpecialKey {
    Up,
    Down,
    Left,
    Right,
    Escape,
}

pub fn key_type(bytes: &[u8]) -> KeyPress {
    match bytes[0] {
        127 => KeyPress::Backspace,
        27  => KeyPress::Special(determine_special_key(bytes)),
        10  => KeyPress::Return,
        _   => KeyPress::Regular
    }
}

fn determine_special_key(bytes: &[u8]) -> SpecialKey {
    match bytes[2] {
        0  => SpecialKey::Escape,
        65 => SpecialKey::Up,
        66 => SpecialKey::Down,
        67 => SpecialKey::Right,
        68 => SpecialKey::Left,
        _  => SpecialKey::Escape, // Need error.
    }
}

