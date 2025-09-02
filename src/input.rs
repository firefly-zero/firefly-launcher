use firefly_rust::*;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Input {
    Up,
    Down,
    Left,
    Right,
    Select,
    Back,
    None,
}

pub struct InputManager {
    // If there is a new input on this frame.
    pub dirty: bool,
    /// For how long up or down button (pad) is held.
    pub held_for: u32,

    /// The state of buttons on the previous frame.
    old_buttons: Buttons,
    /// The state of direction buttons on the previous frame.
    old_dpad: DPad,
    cached: Input,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            old_buttons: Buttons::default(),
            old_dpad: DPad::default(),
            dirty: true,
            held_for: 0,
            cached: Input::None,
        }
    }

    pub const fn get(&self) -> Input {
        self.cached
    }

    pub fn update(&mut self) {
        let btns = self.update_buttons();
        let pad = self.update_dpad();
        let dirty = btns != Input::None || pad != Input::None;
        if dirty {
            self.dirty = dirty;
        }
        self.cached = if btns == Input::None { pad } else { btns }
    }

    fn update_buttons(&mut self) -> Input {
        let new_buttons = read_buttons(Peer::COMBINED);
        let released = new_buttons.just_released(&self.old_buttons);
        self.old_buttons = new_buttons;
        if released.s || released.w {
            Input::Select
        } else if released.e {
            Input::Back
        } else {
            Input::None
        }
    }

    fn update_dpad(&mut self) -> Input {
        let new_dpad = read_pad(Peer::COMBINED).unwrap_or_default().as_dpad();
        let mut dpad_pressed = new_dpad.just_pressed(&self.old_dpad);

        // If a direction button is held, track for how long.
        if new_dpad.up || new_dpad.down {
            self.held_for += 1;
        } else {
            self.held_for = 0;
        }
        if self.held_for > 30 {
            dpad_pressed = new_dpad;
        }

        self.old_dpad = new_dpad;
        if dpad_pressed.down {
            Input::Down
        } else if dpad_pressed.up {
            Input::Up
        } else if dpad_pressed.left {
            Input::Left
        } else if dpad_pressed.right {
            Input::Right
        } else {
            Input::None
        }
    }
}
