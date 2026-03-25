#![allow(static_mut_refs)]

use crate::*;
use alloc::vec;
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
    let settings = get_settings(get_me());
    let encoding = settings.language.encoding();
    let Some(font) = load_file_buf(encoding) else {
        log_error("failed to load font, ROM is corrupted");
        panic!();
    };
    let peers = firefly_rust::get_peers();
    let is_online = peers.len() > 1;
    let mut state = State {
        scene: Scene::List,
        settings,
        font,
        apps: read_apps(is_online),
        is_online,
        pos: 0,
        top_pos: 0,
        button_group: None,
        input: InputManager::new(),
        shift: 0,
        splash: None,
    };

    // Load previously saved state, if any.
    // Currently, the dump only stores the positions in the list.
    let size = get_file_size("state");
    if size != 0 {
        let mut raw = vec![0; size];
        load_file("state", &mut raw);
        if raw.len() == 12 {
            let n_apps = parse_usize(&raw[..4]);
            // If an app is installed or removed since the dump,
            // don't restore the state, stay at the top of the list.
            if n_apps == state.apps.len() {
                state.pos = parse_usize(&raw[4..8]);
                state.top_pos = parse_usize(&raw[8..12]);
            }
        }
    }

    unsafe { STATE.set(state) }.ok().unwrap();
}

fn parse_usize(raw: &[u8]) -> usize {
    usize::from_le_bytes((raw).try_into().unwrap())
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
            Scene::Catalog => catalog_scene::init(self),
            Scene::Delegate(author_id, app_id) => delegate(self, author_id, app_id),
        }
    }
}

/// Delegate handling of the focused app to the given app;
pub fn delegate(state: &State, author_id: &str, app_id: &str) {
    let app = &state.apps[state.pos];
    let full_id = alloc::format!("{}.{}", app.author_id, app.id);
    let full_id = full_id.as_bytes();
    let target_path = alloc::format!("data/{author_id}/{app_id}/etc/target");
    sudo::dump_file(&target_path, full_id);
    sudo::run_app(author_id, app_id);
}
