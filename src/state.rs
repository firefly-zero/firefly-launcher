#![allow(static_mut_refs)]

use crate::*;
use alloc::vec::Vec;
use core::cell::OnceCell;
use firefly_rust::*;

static mut STATE: OnceCell<State> = OnceCell::new();

/// All the global state. Created in [`boot`], updated in [`update`] and [`render`].
pub struct State {
    scene: Scene,
    pub settings: Settings,
    pub font: FileBuf,
    /// The list of all installed apps.
    pub apps: Vec<App>,
    pub is_online: bool,
    /// The currently selected app index.
    pub pos: usize,
    pub board_pos: usize,
    pub dialog_yes: bool,
    /// The index of the firs app on the screen.
    pub top_pos: usize,
    pub button_group: Option<ButtonGroup>,
    pub input: InputManager,
    pub shift: i32,
    pub splash: Option<alloc::string::String>,
}

/// Get the global state
pub fn get_state() -> &'static mut State {
    unsafe { STATE.get_mut() }.unwrap()
}

pub fn init_state() {
    let Some(font) = load_file_buf("font") else {
        log_error("failed to load font, ROM is corrupted");
        panic!();
    };
    let peers = firefly_rust::get_peers();
    let is_online = peers.len() > 1;
    let state = State {
        scene: Scene::List,
        settings: get_settings(get_me()),
        font,
        apps: read_apps(is_online),
        is_online,
        pos: 0,
        board_pos: 0,
        dialog_yes: false,
        top_pos: 0,
        button_group: None,
        input: InputManager::new(),
        shift: 0,
        splash: None,
    };
    unsafe { STATE.set(state) }.ok().unwrap();
}

impl State {
    pub const fn scene(&self) -> &Scene {
        &self.scene
    }

    pub fn transition_to(&mut self, scene: Scene) {
        self.scene = scene;
        self.input.dirty = true;
        match self.scene {
            Scene::List => list_scene::init(self),
            Scene::Info => info_scene::init(self),
            Scene::Stats => stats_scene::init(self),
            Scene::Badges => badges_scene::init(self),
            Scene::Boards => boards_scene::init(self),
            Scene::Scores(i) => scores_scene::init(self, i),
            Scene::Catalog => catalog_scene::init(self),
            Scene::ClearData => delete_scene::init(self),
        }
    }
}
