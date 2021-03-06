use glfw::{Key, MouseButton, Scancode, Action, Modifiers, Window};

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct KeyEvent(pub Key, pub Scancode, pub Action, pub Modifiers);

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct MouseButtonEvent(pub MouseButton, pub Action, pub Modifiers);

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct MouseEvent {
    pub x_pos: f32,
    pub y_pos: f32,
    pub x_offset: f32,
    pub y_offset: f32,
    pub is_scroll: bool,
    pub button_event: Option<MouseButtonEvent>
}

pub trait InputEvent {
    fn mouse_event(&mut self, event: MouseEvent);
    fn keyboard_event(&mut self, event: KeyEvent);
}

pub trait InputControl {
    fn on_mouse(&mut self, mouse: MouseEvent, delta_time: f32);
    fn on_keyboard(&mut self, key: KeyEvent, delta_time: f32);
    fn on_input(&mut self, window: &Window, delta_time: f32);
}