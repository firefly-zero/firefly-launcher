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
    old_dpad: DPad4,
    cached: Input,
    pressed: bool,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            old_buttons: Buttons::default(),
            old_dpad: DPad4::default(),
            dirty: true,
            held_for: 0,
            cached: Input::None,
            pressed: false,
        }
    }

    pub const fn get(&self) -> Input {
        self.cached
    }

    pub const fn pressed(&self) -> bool {
        self.pressed
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
        self.pressed = new_buttons.s || new_buttons.e;
        let released = new_buttons.just_released(&self.old_buttons);
        self.old_buttons = new_buttons;
        if released.s || released.e {
            Input::Select
        } else if released.w || released.menu {
            Input::Back
        } else {
            Input::None
        }
    }

    fn update_dpad(&mut self) -> Input {
        let new_dpad = read_pad(Peer::COMBINED).unwrap_or_default().as_dpad4();
        let mut dpad_pressed = new_dpad.just_pressed(self.old_dpad);

        // If a direction button is held, track for how long.
        if new_dpad == DPad4::Up || new_dpad == DPad4::Down {
            self.held_for += 1;
        } else {
            self.held_for = 0;
        }
        if self.held_for > 30 {
            dpad_pressed = new_dpad;
        }

        self.old_dpad = new_dpad;
        match dpad_pressed {
            DPad4::None => Input::None,
            DPad4::Left => Input::Left,
            DPad4::Right => Input::Right,
            DPad4::Up => Input::Up,
            DPad4::Down => Input::Down,
        }
    }
}
