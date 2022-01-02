use crate::prelude::*;

 
pub struct Keys {
    pub right_pressed: bool,
    pub left_pressed: bool,
    pub down_pressed: bool,
    pub up_pressed: bool,
    pub shift_pressed: bool,
    pub enter_pressed: bool,
    pub q_pressed: bool,
    pub enter_full: bool,
    pub request_quit: bool,
}

pub enum Direction {
    None,
    Up,
    Down,
    Left,
    Right,
}

 

impl Keys {

    pub fn new()->Self {
        Keys {
            right_pressed:false,
            left_pressed:false,
            down_pressed:false,
            up_pressed:false,
            shift_pressed:false,
            enter_full:false,
            enter_pressed:false,
            request_quit:false,
            q_pressed:false,
        }
    }

    pub fn update(&mut self, event_pump: &mut sdl2::EventPump) {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    self.request_quit = true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    self.up_pressed = true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    self.up_pressed = false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    self.down_pressed = true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    self.down_pressed = false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    self.left_pressed = true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    self.left_pressed = false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    self.right_pressed = true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    self.right_pressed = false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => {
                    self.enter_pressed = true;
                    self.enter_full = true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Return),
                    ..
                } => {
                    self.enter_pressed = false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::LShift),
                    ..
                } => {
                    self.shift_pressed = true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::LShift),
                    ..
                } => {
                    self.shift_pressed = false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => {
                    self.q_pressed = true;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Q),
                    ..
                } => {
                    self.q_pressed = false;
                }
                _ => {}
            }
        }
    }
}
