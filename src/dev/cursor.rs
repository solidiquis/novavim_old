use crate::dev::shared::CursorNav;

pub struct Cursor {
    col: u8,
    row: u8
}

impl Default for Cursor {
    fn default() -> Self {
        Self {
            col: 1,
            row: 1,
        }
    }
}

impl CursorNav for Cursor {}
