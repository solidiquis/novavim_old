pub mod normal;
pub mod insert;

use crate::models::{Key, Response, SpecialKey};

pub trait Ctrl {
    fn forward_input_to_handler(&self, key: Key) -> Response;
    fn handle_regular_key(&self, key_press: &str) -> Response;
    fn handle_special_key(&self, key_press: SpecialKey) -> Response;
}
