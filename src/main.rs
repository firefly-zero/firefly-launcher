#![no_std]
#![no_main]
#![deny(
    rust_2018_idioms,
    redundant_lifetimes,
    redundant_semicolons,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::allow_attributes
)]
#![expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::collapsible_if,
    clippy::wildcard_imports
)]

extern crate alloc;

mod apps;
mod button_group;
mod catalog_scene;
mod components;
mod info_scene;
mod input;
mod list_scene;
mod scroll;
mod state;
mod stats_scene;
mod translations;
mod utils;

use alloc::vec::Vec;
use apps::*;
use button_group::ButtonGroup;
use components::*;
use firefly_rust::*;
use input::{Input, InputManager};
use state::*;
use translations::Message;
use utils::*;

/// If the splash screen was rendered on the screen.
static mut RENDERED: bool = false;
/// If the state is not initialized yet.
static mut LOADING: bool = true;

#[derive(Clone, Copy)]
pub enum Scene {
    /// List all installed apps.
    List,
    /// Show the app context menu with basic app info and action buttons.
    Info,
    Stats,
    /// Show QR code with a link to the app in catalog.fireflyzero.com.
    Catalog,
    /// Delegate handling of the focused app to the given app.
    Delegate(&'static str, &'static str),
}

#[unsafe(no_mangle)]
extern "C" fn boot() {
    let splash = load_file_buf("_splash").unwrap();
    let splash = splash.as_image();
    draw_image(&splash, Point::MIN);
}

#[unsafe(no_mangle)]
extern "C" fn before_exit() {
    let state = get_state();
    dump_state(state);
}

/// Save in FS some of the launcher state (list positions).
fn dump_state(state: &State) {
    let mut raw = Vec::with_capacity(12);
    raw.extend_from_slice(&state.apps.len().to_le_bytes());
    raw.extend_from_slice(&state.pos.to_le_bytes());
    raw.extend_from_slice(&state.top_pos.to_le_bytes());
    dump_file("state", &raw);
}

#[unsafe(no_mangle)]
extern "C" fn update() {
    // Wait for the splash screen to be rendered.
    if !unsafe { RENDERED } {
        return;
    }
    if unsafe { LOADING } {
        init_state();
        unsafe { LOADING = false };
        return;
    }

    let state = get_state();
    state.input.update();
    match state.scene() {
        Scene::List => list_scene::update(state),
        Scene::Info => info_scene::update(state),
        Scene::Stats => stats_scene::update(state),
        Scene::Catalog => catalog_scene::update(state),
        Scene::Delegate(_, _) => {}
    }
}

#[unsafe(no_mangle)]
extern "C" fn render() {
    // During the first rendering iteration,
    // we don't render anything from the "render" callback.
    // Instead, we let the runtime to display the splash screen
    // rendered earlier from "boot".
    if !unsafe { RENDERED } {
        unsafe { RENDERED = true };
        return;
    }
    // Don't render until the list of apps is loaded.
    if unsafe { LOADING } {
        return;
    }

    let state = get_state();
    // TODO(@orsinium): enable when we have support for menu_closed callback.
    // When menu is opened and closed, we have to mark input as dirty
    // to re-render the frame.
    //
    // if !state.input.dirty {
    //     return;
    // }
    state.input.dirty = false;
    match state.scene() {
        Scene::List => list_scene::render(state),
        Scene::Info => info_scene::render(state),
        Scene::Stats => stats_scene::render(state),
        Scene::Catalog => catalog_scene::render(state),
        Scene::Delegate(_, _) => {}
    }
}

#[unsafe(no_mangle)]
extern "C" fn cheat(cmd: i32, val: i32) -> i32 {
    if cmd == 1 {
        let state = get_state();
        let Ok(index) = usize::try_from(val) else {
            return 0;
        };
        if let Some(app) = state.apps.get(index) {
            sudo::run_app(&app.author_id, &app.id);
            return 1;
        }
        return 0;
    }
    0
}
