use crate::models::{Key, SpecialKey};

pub fn key_type(bytes: &[u8; 3]) -> Key {
    match bytes[0] {
        127 => Key::Backspace,
        27  => Key::Special(determine_special_key(bytes)),
        10  => Key::Return,
        _   => {
            let key = bytes_to_str(bytes).unwrap_or_else(|_| {
                ""
            });

            Key::Regular(key)
        }
    }
}

pub fn bytes_to_str(bytes: &[u8; 3]) -> Result<&str, std::str::Utf8Error> {
    use std::str;

    let mut ch = str::from_utf8(bytes)?;
    ch = ch.trim_end_matches("\u{0}");

    Ok(ch)
}

fn determine_special_key(bytes: &[u8]) -> SpecialKey {
    match bytes[2] {
        0  => SpecialKey::Escape,
        65 => SpecialKey::Up,
        66 => SpecialKey::Down,
        67 => SpecialKey::Right,
        68 => SpecialKey::Left,
        _  => SpecialKey::None,
    }
}
