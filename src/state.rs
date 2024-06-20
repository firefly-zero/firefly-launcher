use alloc::vec::Vec;
use core::cell::OnceCell;
use firefly_rust::*;

use crate::*;

static mut STATE: OnceCell<State> = OnceCell::new();

/// All the global state. Created in [`boot`], updated in [`update`] and [`render`].
pub struct State {
    pub font: FileBuf,
    /// The list of all installed apps.
    pub apps: Vec<App>,
    /// The currently selected app index.
    pub pos: usize,
    /// The index of the firs app on the screen.
    pub top_pos: usize,
    /// The state of buttons on the previous frame.
    pub old_buttons: Buttons,
    /// The state of direction buttons on the previous frame.
    pub old_dpad: DPad,
    /// The next command to run when rendering.
    pub command: Option<Command>,
    /// For how long up or down button (pad) is held.
    pub held_for: u32,
    pub shift: i32,
}

/// Get the global state
pub fn get_state() -> &'static mut State {
    unsafe { STATE.get_mut() }.unwrap()
}

pub fn init_state() {
    let state = State {
        font: rom::load_buf("font"),
        apps: read_apps(),
        pos: 0,
        top_pos: 0,
        old_buttons: Buttons::default(),
        old_dpad: DPad::default(),
        command: None,
        held_for: 0,
        shift: 0,
    };
    unsafe { STATE.set(state) }.ok().unwrap();
}
