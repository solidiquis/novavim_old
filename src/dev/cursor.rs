use crate::dev::{Cursor, CursorNav};

impl Default for Cursor {
    fn default() -> Self {
        Self {
            col: 1,
            row: 1,
        }
    }
}

impl CursorNav for Cursor {}
