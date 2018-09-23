//! Typedefs for input shortcuts.
pub mod input;

use ggez::event::*;

use std::sync::{Arc, RwLock};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Button {
    Fire,
    Menu,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Axis {
    Vert,
    Horz,
}

pub type InputBinding = input::InputBinding<Axis, Button>;
pub type InputEvent = input::InputEffect<Axis, Button>;
pub type InputState = input::InputState<Axis, Button>;

#[derive(Clone)]
pub struct Input(pub InputState);

impl Default for Input {
    fn default() -> Self {
        Input(InputState::new())
    }
}

/// Create the default keybindings for our input state.
pub fn create_input_binding() -> input::InputBinding<Axis, Button> {
    input::InputBinding::new()
        .bind_key_to_axis(Keycode::Up, Axis::Vert, true)
        .bind_key_to_axis(Keycode::Down, Axis::Vert, false)
        .bind_key_to_axis(Keycode::Left, Axis::Horz, false)
        .bind_key_to_axis(Keycode::Right, Axis::Horz, true)
        .bind_key_to_button(Keycode::Z, Button::Fire)
        .bind_key_to_button(Keycode::Escape, Button::Menu)
}
